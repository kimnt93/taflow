//! Incremental Ultimate Oscillator (ULTOSC).

use crate::error::{TaError, TaResult};
use crate::stream::Window;

/// The element pushed `lag` pushes ago (`1 ..= len`), via contiguous slices.
#[inline]
fn ring_lag(window: &Window, lag: usize) -> f64 {
    let (front, back) = window.as_slices();
    let index = front.len() + back.len() - lag;
    if index < front.len() {
        front[index]
    } else {
        back[index - front.len()]
    }
}

/// One (buying-pressure, true-range) sliding sum pair for a single period.
#[derive(Debug, Clone, Copy)]
struct FlowSums {
    period: usize,
    bp_sum: f64,
    tr_sum: f64,
}

impl FlowSums {
    /// Same per-sum arithmetic order as the previous per-period rings:
    /// subtract the evicted element, then add the new one.
    #[inline]
    fn advance(&mut self, old_bp: Option<f64>, old_tr: Option<f64>, bp: f64, tr: f64) {
        if let Some(old) = old_bp {
            self.bp_sum -= old;
        }
        if let Some(old) = old_tr {
            self.tr_sum -= old;
        }
        self.bp_sum += bp;
        self.tr_sum += tr;
    }

    #[inline]
    fn ratio(&self) -> f64 {
        if self.tr_sum > 0.0 {
            self.bp_sum / self.tr_sum
        } else {
            0.0
        }
    }
}

/// Persistent Ultimate Oscillator with constant work per appended HLC bar.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `UltimateOscillator`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
///
/// The three periods previously kept six deques (a bp+tr ring each). They
/// now share ONE bp ring and ONE tr ring of the largest period (M4 dedup);
/// the shorter periods read their eviction candidates at the matching lag.
pub struct UltimateOscillator {
    previous_close: Option<f64>,
    max_period: usize,
    /// Flows pushed so far, saturating at `max_period`.
    seen: usize,
    bp: Window,
    tr: Window,
    first: FlowSums,
    second: FlowSums,
    third: FlowSums,
    value: Option<f64>,
}
impl UltimateOscillator {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod1: usize, timeperiod2: usize, timeperiod3: usize) -> TaResult<Self> {
        if timeperiod1 == 0 || timeperiod2 == 0 || timeperiod3 == 0 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: "0".to_string(),
                reason: "must be >= 1",
            });
        }
        let max_period = timeperiod1.max(timeperiod2).max(timeperiod3);
        let sums = |period| FlowSums {
            period,
            bp_sum: 0.0,
            tr_sum: 0.0,
        };
        Ok(Self {
            previous_close: None,
            max_period,
            seen: 0,
            bp: Window::new(max_period)?,
            tr: Window::new(max_period)?,
            first: sums(timeperiod1),
            second: sums(timeperiod2),
            third: sums(timeperiod3),
            value: None,
        })
    }

    #[inline]
    fn combined(first: &FlowSums, second: &FlowSums, third: &FlowSums) -> f64 {
        100.0 * (4.0 * first.ratio() + 2.0 * second.ratio() + third.ratio()) / 7.0
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let Some(previous) = self.previous_close.replace(close) else {
            return None;
        };
        let true_low = low.min(previous);
        let true_high = high.max(previous);
        let bp = close - true_low;
        let tr = true_high - true_low;
        // Eviction candidates are read at each period's lag BEFORE pushing;
        // each period evicts exactly when its own ring would have been full.
        let old = |sums: &FlowSums| {
            (self.seen >= sums.period).then(|| {
                (
                    ring_lag(&self.bp, sums.period),
                    ring_lag(&self.tr, sums.period),
                )
            })
        };
        let old1 = old(&self.first);
        let old2 = old(&self.second);
        let old3 = old(&self.third);
        self.bp.push(bp);
        self.tr.push(tr);
        self.seen = (self.seen + 1).min(self.max_period);
        for (sums, old) in [
            (&mut self.first, old1),
            (&mut self.second, old2),
            (&mut self.third, old3),
        ] {
            sums.advance(old.map(|o| o.0), old.map(|o| o.1), bp, tr);
        }
        self.value = (self.seen >= self.max_period)
            .then(|| Self::combined(&self.first, &self.second, &self.third));
        self.value
    }

    /// Bulk kernel: O(1) add/evict recurrences on the three sum pairs, with
    /// new and evicted bp/tr recomputed directly from the input slices (the
    /// recomputation is deterministic, so evicted values are bit-identical
    /// to what the rings held). Outputs and post-run state are bit-identical
    /// to per-bar [`Self::append`].
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        let n = high.len();
        if n != low.len() || n != close.len() {
            return Err(TaError::LengthMismatch {
                expected: n,
                got: low.len().min(close.len()),
            });
        }
        let max_period = self.max_period;
        output.reserve(n);
        // Warm-up prologue. After `max_period + 1` appends both rings hold
        // exactly the flows of steps 1..=max_period of this slice.
        let prologue = n.min(max_period + 1);
        for i in 0..prologue {
            output.push(self.append(high[i], low[i], close[i]).unwrap_or(f64::NAN));
        }
        if n <= max_period + 1 {
            return Ok(());
        }
        // bp/tr for the transition into bar `i` (`i >= 1`).
        let flow = |i: usize| {
            let previous = close[i - 1];
            let true_low = low[i].min(previous);
            let true_high = high[i].max(previous);
            (close[i] - true_low, true_high - true_low)
        };
        let mut first = self.first;
        let mut second = self.second;
        let mut third = self.third;
        let mut last = f64::NAN;
        for i in (max_period + 1)..n {
            let (bp, tr) = flow(i);
            // Evicted flows: generated `period` steps ago, all within slice.
            for sums in [&mut first, &mut second, &mut third] {
                let (old_bp, old_tr) = flow(i - sums.period);
                sums.advance(Some(old_bp), Some(old_tr), bp, tr);
            }
            last = Self::combined(&first, &second, &third);
            output.push(last);
        }
        self.first = first;
        self.second = second;
        self.third = third;
        self.previous_close = Some(close[n - 1]);
        self.seen = max_period;
        self.value = Some(last);
        // Rebuild the shared rings so subsequent appends continue bit-identically.
        self.bp.clear();
        self.tr.clear();
        for i in (n - max_period)..n {
            let (bp, tr) = flow(i);
            self.bp.push(bp);
            self.tr.push(tr);
        }
        Ok(())
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.seen = 0;
        self.bp.clear();
        self.tr.clear();
        for sums in [&mut self.first, &mut self.second, &mut self.third] {
            sums.bp_sum = 0.0;
            sums.tr_sum = 0.0;
        }
        self.value = None;
    }
}
