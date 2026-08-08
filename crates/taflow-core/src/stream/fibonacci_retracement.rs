//! Stateful rolling Fibonacci retracement levels.

use crate::error::TaResult;

/// Ratios applied to the rolling range, 0% through 100%.
const RATIOS: [f64; 7] = [0.0, 0.236, 0.382, 0.5, 0.618, 0.786, 1.0];

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

/// Rolling high/low range converted to seven Fibonacci levels.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `FibonacciRetracement`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct FibonacciRetracement {
    period: usize,
    maxima: MonotonicRing<true>,
    minima: MonotonicRing<false>,
    seen: u64,
    value: Option<[f64; 7]>,
}

impl FibonacciRetracement {
    /// Creates the retracement calculator with a positive rolling window.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 1 {
            return Err(super::invalid_period("window", period, 1));
        }
        Ok(Self {
            period,
            maxima: MonotonicRing::new(period),
            minima: MonotonicRing::new(period),
            seen: 0,
            value: None,
        })
    }

    /// Appends one close and returns levels from 0% through 100%.
    pub fn append(&mut self, close: f64) -> [f64; 7] {
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
        let levels = RATIOS.map(|ratio| high - span * ratio);
        self.value = Some(levels);
        levels
    }

    /// Returns the latest seven retracement levels.
    pub fn value(&self) -> Option<[f64; 7]> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// The pre-optimisation implementation, kept verbatim as the oracle.
    struct Oracle {
        period: usize,
        closes: VecDeque<f64>,
    }

    impl Oracle {
        fn new(period: usize) -> Self {
            Self {
                period,
                closes: VecDeque::with_capacity(period),
            }
        }

        fn append(&mut self, close: f64) -> [f64; 7] {
            self.closes.push_back(close);
            if self.closes.len() > self.period {
                self.closes.pop_front();
            }
            let low = self.closes.iter().copied().fold(f64::INFINITY, f64::min);
            let high = self
                .closes
                .iter()
                .copied()
                .fold(f64::NEG_INFINITY, f64::max);
            let span = high - low;
            RATIOS.map(|ratio| high - span * ratio)
        }
    }

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

    #[test]
    fn matches_the_full_window_scan_bitwise() {
        let input = lcg_series(5_000, 0xF1B0_0001);
        for period in [1usize, 2, 3, 7, 120, 5_001] {
            let mut state = FibonacciRetracement::new(period).unwrap();
            let mut oracle = Oracle::new(period);
            for (bar, &close) in input.iter().enumerate() {
                let actual = state.append(close);
                let expected = oracle.append(close);
                for level in 0..7 {
                    assert_eq!(
                        actual[level].to_bits(),
                        expected[level].to_bits(),
                        "period {period} bar {bar} level {level}"
                    );
                }
            }
        }
    }

    #[test]
    fn matches_the_full_window_scan_with_plateaus_and_nans() {
        // Long runs of equal values exercise the monotonic tie handling; NaN
        // samples exercise the fold's NaN-ignoring behaviour, including a
        // fully-NaN window.
        let mut input = Vec::new();
        for (bar, &value) in lcg_series(2_000, 0xF1B0_0002).iter().enumerate() {
            input.push(if bar % 7 == 0 { 100.0 } else { value });
            if bar % 23 == 0 {
                input.push(f64::NAN);
            }
        }
        input.extend(std::iter::repeat(f64::NAN).take(200));
        input.extend(lcg_series(500, 0xF1B0_0003));

        for period in [1usize, 2, 5, 60, 120] {
            let mut state = FibonacciRetracement::new(period).unwrap();
            let mut oracle = Oracle::new(period);
            for (bar, &close) in input.iter().enumerate() {
                let actual = state.append(close);
                let expected = oracle.append(close);
                for level in 0..7 {
                    assert_eq!(
                        actual[level].to_bits(),
                        expected[level].to_bits(),
                        "period {period} bar {bar} level {level}"
                    );
                }
            }
        }
    }

    #[test]
    fn reset_restores_a_fresh_state() {
        let input = lcg_series(1_000, 0xF1B0_0004);
        let mut state = FibonacciRetracement::new(30).unwrap();
        for &close in &input {
            state.append(close);
        }
        state.reset();
        assert!(state.value().is_none());

        let mut fresh = FibonacciRetracement::new(30).unwrap();
        for &close in &input {
            let after_reset = state.append(close);
            let from_fresh = fresh.append(close);
            for level in 0..7 {
                assert_eq!(after_reset[level].to_bits(), from_fresh[level].to_bits());
            }
        }
    }
}
