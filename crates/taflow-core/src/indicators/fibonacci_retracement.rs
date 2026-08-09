//! Stateful rolling Fibonacci retracement levels.

use crate::error::TaResult;

/// Amortised O(1) monotonic ring over the trailing window.
///
/// `KEEP_MAX == true` yields the window maximum, `false` the minimum. The
/// front entry is the extreme; entries are stored with the absolute bar index
/// that produced them so expiry is a front pop.
///
/// NaN inputs are never inserted, which reproduces `fold(±INFINITY, f64::min
/// / f64::max)` exactly: `f64::min(a, NaN) == a`, so NaN samples contribute
/// nothing. With no live entry the caller falls back to the same `±INFINITY`
/// seed the fold used. Every reported extreme is an unmodified window sample,
/// so the result is bit-identical to the scan it replaces.
#[derive(Debug, Clone)]
struct MonotonicRing<const KEEP_MAX: bool> {
    /// `(value, absolute bar index)` in monotonic order from the front.
    entries: Box<[(f64, u64)]>,
    head: usize,
    len: usize,
}

impl<const KEEP_MAX: bool> MonotonicRing<KEEP_MAX> {
    fn new(period: usize) -> Self {
        // `period + 1` slots: a push happens before the expiry pop, so the
        // ring transiently holds one more entry than the window length.
        Self {
            entries: vec![(0.0, 0); period + 1].into_boxed_slice(),
            head: 0,
            len: 0,
        }
    }

    #[inline]
    fn physical(&self, offset: usize) -> usize {
        let index = self.head + offset;
        if index >= self.entries.len() {
            index - self.entries.len()
        } else {
            index
        }
    }

    /// Pushes `value` observed at `index`, then expires entries older than
    /// `index + 1 - period`. Returns the current extreme, or `None` when no
    /// non-NaN sample is live.
    #[inline]
    fn push(&mut self, value: f64, index: u64, period: u64) -> Option<f64> {
        if !value.is_nan() {
            // Drop dominated entries from the back. Strict comparison keeps
            // the oldest of equal values, which never changes the reported
            // extreme (the values are equal) but bounds the ring by `period`.
            while self.len > 0 {
                let back = self.entries[self.physical(self.len - 1)].0;
                let dominated = if KEEP_MAX { back < value } else { back > value };
                if !dominated {
                    break;
                }
                self.len -= 1;
            }
            let slot = self.physical(self.len);
            self.entries[slot] = (value, index);
            self.len += 1;
        }
        let oldest_live = index + 1 - period.min(index + 1);
        while self.len > 0 && self.entries[self.head].1 < oldest_live {
            self.head = self.physical(1);
            self.len -= 1;
        }
        (self.len > 0).then(|| self.entries[self.head].0)
    }

    fn reset(&mut self) {
        self.head = 0;
        self.len = 0;
    }
}

/// Named rolling Fibonacci levels from zero through one hundred percent.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FibonacciRetracementValue {
    /// Rolling high, or zero-percent retracement.
    pub level_zero: f64,
    /// 23.6% retracement from the rolling high.
    pub level_twenty_three_point_six: f64,
    /// 38.2% retracement from the rolling high.
    pub level_thirty_eight_point_two: f64,
    /// 50% retracement from the rolling high.
    pub level_fifty: f64,
    /// 61.8% retracement from the rolling high.
    pub level_sixty_one_point_eight: f64,
    /// 78.6% retracement from the rolling high.
    pub level_seventy_eight_point_six: f64,
    /// Rolling low, or one-hundred-percent retracement.
    pub level_one_hundred: f64,
}

/// Persistent rolling high/low state converted to seven Fibonacci levels.
#[derive(Debug, Clone)]
pub struct FibonacciRetracement {
    period: usize,
    maxima: MonotonicRing<true>,
    minima: MonotonicRing<false>,
    seen: u64,
    value: Option<FibonacciRetracementValue>,
}

impl FibonacciRetracement {
    /// Creates the retracement calculator with a positive rolling window.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 1 {
            return Err(crate::stream::invalid_period("window", period, 1));
        }
        Ok(Self {
            period,
            maxima: MonotonicRing::new(period),
            minima: MonotonicRing::new(period),
            seen: 0,
            value: None,
        })
    }

    /// Appends one close and returns named levels from 0% through 100%.
    pub fn append(&mut self, close: f64) -> FibonacciRetracementValue {
        let index = self.seen;
        self.seen += 1;
        let period = self.period as u64;
        // Amortised O(1) replacements for the two full-window folds; the
        // `±INFINITY` fallbacks match the empty/all-NaN fold seeds exactly.
        let high = self
            .maxima
            .push(close, index, period)
            .unwrap_or(f64::NEG_INFINITY);
        let low = self
            .minima
            .push(close, index, period)
            .unwrap_or(f64::INFINITY);
        let span = high - low;
        let value = FibonacciRetracementValue {
            level_zero: high - span * 0.0,
            level_twenty_three_point_six: high - span * 0.236,
            level_thirty_eight_point_two: high - span * 0.382,
            level_fifty: high - span * 0.5,
            level_sixty_one_point_eight: high - span * 0.618,
            level_seventy_eight_point_six: high - span * 0.786,
            level_one_hundred: high - span * 1.0,
        };
        self.value = Some(value);
        value
    }

    /// Appends a close slice into seven aligned output histories.
    #[allow(clippy::too_many_arguments)]
    pub fn extend_slice_into(
        &mut self,
        close: &[f64],
        level_zero: &mut Vec<f64>,
        level_twenty_three_point_six: &mut Vec<f64>,
        level_thirty_eight_point_two: &mut Vec<f64>,
        level_fifty: &mut Vec<f64>,
        level_sixty_one_point_eight: &mut Vec<f64>,
        level_seventy_eight_point_six: &mut Vec<f64>,
        level_one_hundred: &mut Vec<f64>,
    ) {
        level_zero.reserve(close.len());
        level_twenty_three_point_six.reserve(close.len());
        level_thirty_eight_point_two.reserve(close.len());
        level_fifty.reserve(close.len());
        level_sixty_one_point_eight.reserve(close.len());
        level_seventy_eight_point_six.reserve(close.len());
        level_one_hundred.reserve(close.len());
        for &close in close {
            let value = self.append(close);
            level_zero.push(value.level_zero);
            level_twenty_three_point_six.push(value.level_twenty_three_point_six);
            level_thirty_eight_point_two.push(value.level_thirty_eight_point_two);
            level_fifty.push(value.level_fifty);
            level_sixty_one_point_eight.push(value.level_sixty_one_point_eight);
            level_seventy_eight_point_six.push(value.level_seventy_eight_point_six);
            level_one_hundred.push(value.level_one_hundred);
        }
    }

    /// Returns the latest seven named retracement levels.
    pub fn value(&self) -> Option<FibonacciRetracementValue> {
        self.value
    }

    /// Clears rolling history and levels.
    pub fn reset(&mut self) {
        self.maxima.reset();
        self.minima.reset();
        self.seen = 0;
        self.value = None;
    }
}
