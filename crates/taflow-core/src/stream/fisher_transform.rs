//! Persistent and batch Fisher Transform implementation.

use super::{MonotonicMax, MonotonicMin};
use crate::error::{TaError, TaResult};

/// Causal Fisher Transform over the midpoint of aligned high/low bars.
///
/// Rolling extrema use monotonic deques, so each appended bar is amortized
/// O(1). The first complete window seeds the pandas-ta-classic recurrence at
/// zero; subsequent values apply the bounded logarithmic transform.
#[derive(Debug, Clone)]
pub struct FisherTransform {
    highs: MonotonicMax,
    lows: MonotonicMin,
    previous_position: f64,
    previous_fisher: f64,
    seeded: bool,
    value: Option<f64>,
}

impl FisherTransform {
    /// Create an empty transform with a positive trailing window length.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        if timeperiod == 0 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: timeperiod.to_string(),
                reason: "must be >= 1",
            });
        }
        Ok(Self {
            highs: MonotonicMax::new(timeperiod)?,
            lows: MonotonicMin::new(timeperiod)?,
            previous_position: 0.0,
            previous_fisher: 0.0,
            seeded: false,
            value: None,
        })
    }

    /// Append one high/low bar and return the latest value after warm-up.
    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        let midpoint = (high + low) * 0.5;
        let maximum = self.highs.append(midpoint);
        let minimum = self.lows.append(midpoint);
        self.value = maximum.zip(minimum).map(|(high, low)| {
            if !self.seeded {
                self.seeded = true;
                self.previous_position = 0.0;
                self.previous_fisher = 0.0;
                return 0.0;
            }
            let position = if high != low {
                (midpoint - low) / (high - low) - 0.5
            } else {
                0.0
            };
            let raw = 0.66 * position + 0.67 * self.previous_position;
            let bounded = if raw < -0.99 {
                -0.999
            } else if raw > 0.99 {
                0.999
            } else {
                raw
            };
            let fisher = 0.5 * (((1.0 + bounded) / (1.0 - bounded)).ln() + self.previous_fisher);
            self.previous_position = bounded;
            self.previous_fisher = fisher;
            fisher
        });
        self.value
    }

    /// Return the latest value, or `None` before the first complete window.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restore fresh-state behavior while retaining deque allocations.
    pub fn reset(&mut self) {
        self.highs.reset();
        self.lows.reset();
        self.previous_position = 0.0;
        self.previous_fisher = 0.0;
        self.seeded = false;
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// pandas-ta-classic recurrence with deque extrema scans.
    struct Reference {
        period: usize,
        values: VecDeque<f64>,
        previous_position: f64,
        previous_fisher: f64,
        seeded: bool,
    }

    impl Reference {
        fn new(period: usize) -> Self {
            Self {
                period,
                values: VecDeque::with_capacity(period),
                previous_position: 0.0,
                previous_fisher: 0.0,
                seeded: false,
            }
        }

        fn append(&mut self, high: f64, low: f64) -> Option<f64> {
            if self.values.len() == self.period {
                self.values.pop_front();
            }
            self.values.push_back((high + low) * 0.5);
            if self.values.len() != self.period {
                return None;
            }
            if !self.seeded {
                self.seeded = true;
                return Some(0.0);
            }
            let maximum = self
                .values
                .iter()
                .copied()
                .fold(f64::NEG_INFINITY, f64::max);
            let minimum = self.values.iter().copied().fold(f64::INFINITY, f64::min);
            let position = if maximum != minimum {
                (self.values.back().copied().unwrap() - minimum) / (maximum - minimum) - 0.5
            } else {
                0.0
            };
            let raw = 0.66 * position + 0.67 * self.previous_position;
            let bounded = if raw < -0.99 {
                -0.999
            } else if raw > 0.99 {
                0.999
            } else {
                raw
            };
            let fisher = 0.5 * (((1.0 + bounded) / (1.0 - bounded)).ln() + self.previous_fisher);
            self.previous_position = bounded;
            self.previous_fisher = fisher;
            Some(fisher)
        }
    }

    fn lcg_series(len: usize, seed: u64) -> Vec<f64> {
        let mut state = seed;
        (0..len)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                90.0 + ((state >> 11) as f64 / (1u64 << 53) as f64) * 20.0
            })
            .collect()
    }

    #[test]
    fn matches_reference_bitwise_and_survives_chunking() {
        let base = lcg_series(5_000, 0xB1_5EED_81);
        let high: Vec<f64> = base.iter().map(|v| v + 0.6).collect();
        let low: Vec<f64> = base.iter().map(|v| v - 0.6).collect();
        // A flat stretch exercises the `high == low` branch.
        let mut high = high;
        let mut low = low;
        for i in 1_000..1_050 {
            high[i] = 100.0;
            low[i] = 100.0;
        }
        for period in [1usize, 2, 5, 9, 200] {
            let mut reference = Reference::new(period);
            let expected: Vec<f64> = (0..base.len())
                .map(|i| reference.append(high[i], low[i]).unwrap_or(f64::NAN))
                .collect();
            let mut state = FisherTransform::new(period).unwrap();
            for (i, want) in expected.iter().enumerate() {
                let got = state.append(high[i], low[i]).unwrap_or(f64::NAN);
                assert_eq!(want.to_bits(), got.to_bits(), "p={period} bar {i}");
            }
            state.reset();
            let mut fresh = Reference::new(period);
            for i in 0..512 {
                let want = fresh.append(high[i], low[i]).unwrap_or(f64::NAN);
                let got = state.append(high[i], low[i]).unwrap_or(f64::NAN);
                assert_eq!(want.to_bits(), got.to_bits(), "p={period} post-reset {i}");
            }
        }
    }
}
