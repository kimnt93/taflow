//! Incremental Ultimate Oscillator (ULTOSC).

use super::Window;
use crate::error::{TaError, TaResult};

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

/// Ultimate Oscillator (ULTOSC)
///
/// ULTOSC = 100 * (4*avg7 + 2*avg14 + avg28) / 7
/// Here `avg_n = sum(BP, n) / sum(TR, n)`.
/// BP (Buying Pressure) = close - min(low, prev_close)
/// TR (True Range) = max(high, prev_close) - min(low, prev_close)
pub fn ultimate_oscillator(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    timeperiod1: usize,
    timeperiod2: usize,
    timeperiod3: usize,
) -> TaResult<Vec<f64>> {
    let len = high.len();
    if len != low.len() || len != close.len() {
        return Err(TaError::LengthMismatch {
            expected: len,
            got: low.len().min(close.len()),
        });
    }
    let max_period = timeperiod1.max(timeperiod2).max(timeperiod3);
    if len <= max_period {
        return Err(TaError::InsufficientData {
            need: max_period + 1,
            got: len,
        });
    }

    // Compute buying pressure (BP) and true range (TR).
    let mut bp = vec![0.0; len];
    let mut tr = vec![0.0; len];
    for i in 1..len {
        let true_low = low[i].min(close[i - 1]);
        let true_high = high[i].max(close[i - 1]);
        bp[i] = close[i] - true_low;
        tr[i] = true_high - true_low;
    }

    let mut output = vec![0.0_f64; len];
    output[..max_period].fill(f64::NAN);

    // Initialize sliding sums for the first output position (i = max_period)
    // Window for period p at position i covers bp[(i+1-p)..=i]
    let i0 = max_period;
    let mut sum_bp1: f64 = bp[(i0 + 1 - timeperiod1)..=i0].iter().sum();
    let mut sum_tr1: f64 = tr[(i0 + 1 - timeperiod1)..=i0].iter().sum();
    let mut sum_bp2: f64 = bp[(i0 + 1 - timeperiod2)..=i0].iter().sum();
    let mut sum_tr2: f64 = tr[(i0 + 1 - timeperiod2)..=i0].iter().sum();
    let mut sum_bp3: f64 = bp[(i0 + 1 - timeperiod3)..=i0].iter().sum();
    let mut sum_tr3: f64 = tr[(i0 + 1 - timeperiod3)..=i0].iter().sum();

    let avg1 = if sum_tr1 > 0.0 {
        sum_bp1 / sum_tr1
    } else {
        0.0
    };
    let avg2 = if sum_tr2 > 0.0 {
        sum_bp2 / sum_tr2
    } else {
        0.0
    };
    let avg3 = if sum_tr3 > 0.0 {
        sum_bp3 / sum_tr3
    } else {
        0.0
    };
    output[i0] = 100.0 * (4.0 * avg1 + 2.0 * avg2 + avg3) / 7.0;

    // Slide sums forward: add new element, remove oldest
    for i in (max_period + 1)..len {
        sum_bp1 += bp[i] - bp[i - timeperiod1];
        sum_tr1 += tr[i] - tr[i - timeperiod1];
        sum_bp2 += bp[i] - bp[i - timeperiod2];
        sum_tr2 += tr[i] - tr[i - timeperiod2];
        sum_bp3 += bp[i] - bp[i - timeperiod3];
        sum_tr3 += tr[i] - tr[i - timeperiod3];

        let avg1 = if sum_tr1 > 0.0 {
            sum_bp1 / sum_tr1
        } else {
            0.0
        };
        let avg2 = if sum_tr2 > 0.0 {
            sum_bp2 / sum_tr2
        } else {
            0.0
        };
        let avg3 = if sum_tr3 > 0.0 {
            sum_bp3 / sum_tr3
        } else {
            0.0
        };
        output[i] = 100.0 * (4.0 * avg1 + 2.0 * avg2 + avg3) / 7.0;
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lcg_series(n: usize, mut state: u64) -> Vec<f64> {
        (0..n)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                90.0 + (state >> 11) as f64 / (1u64 << 53) as f64 * 20.0
            })
            .collect()
    }

    fn assert_same_bits(actual: &[f64], expected: &[f64], label: &str) {
        assert_eq!(actual.len(), expected.len(), "{label}: length");
        for (i, (a, b)) in actual.iter().zip(expected).enumerate() {
            assert_eq!(a.to_bits(), b.to_bits(), "{label}: bar {i}");
        }
    }

    fn hlc(n: usize, seed: u64) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let close = lcg_series(n, seed);
        let spread_hi = lcg_series(n, seed ^ 0xDEAD_BEEF);
        let spread_lo = lcg_series(n, seed ^ 0x1234_5678);
        let high: Vec<f64> = close
            .iter()
            .zip(&spread_hi)
            .map(|(c, s)| c + (s - 89.0).abs() * 0.1)
            .collect();
        let low: Vec<f64> = close
            .iter()
            .zip(&spread_lo)
            .map(|(c, s)| c - (s - 89.0).abs() * 0.1)
            .collect();
        (high, low, close)
    }

    #[test]
    fn ultosc_bulk_is_bitwise_identical_to_per_bar_append() {
        let (high, low, close) = hlc(5_000, 0x5EED_0D5C);
        let (th, tl, tc) = hlc(256, 0x7A11_0D5C);
        for (p1, p2, p3) in [
            (7usize, 14usize, 28usize),
            (2, 3, 5),
            (5, 5, 5),
            (28, 14, 7),
            (30, 60, 200),
        ] {
            let mut per_bar = UltimateOscillator::new(p1, p2, p3).unwrap();
            let reference: Vec<f64> = (0..close.len())
                .map(|i| {
                    per_bar
                        .append(high[i], low[i], close[i])
                        .unwrap_or(f64::NAN)
                })
                .collect();
            let tail_reference: Vec<f64> = (0..tc.len())
                .map(|i| per_bar.append(th[i], tl[i], tc[i]).unwrap_or(f64::NAN))
                .collect();

            for chunk in [usize::MAX, 1, 7, 97] {
                let mut state = UltimateOscillator::new(p1, p2, p3).unwrap();
                let mut out = Vec::new();
                let mut start = 0;
                while start < close.len() {
                    let end = (start + chunk.min(close.len())).min(close.len());
                    state
                        .extend_slices_into(
                            &high[start..end],
                            &low[start..end],
                            &close[start..end],
                            &mut out,
                        )
                        .unwrap();
                    start = end;
                }
                let label = format!("ULTOSC {p1}/{p2}/{p3} chunk {chunk}");
                assert_same_bits(&out, &reference, &label);
                let tail_out: Vec<f64> = (0..tc.len())
                    .map(|i| state.append(th[i], tl[i], tc[i]).unwrap_or(f64::NAN))
                    .collect();
                assert_same_bits(&tail_out, &tail_reference, &format!("{label} tail"));
            }
        }
    }
}
