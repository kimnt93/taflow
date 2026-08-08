//! Rolling median state.

use super::operator_states::validate_period;
use super::sorted_ring::SortedRing;
use crate::TaResult;

/// Computes the causal median over a fixed trailing window.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingMedian`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingMedian {
    window: SortedRing,
    timeperiod: usize,
    value: Option<f64>,
}

impl RollingMedian {
    /// Creates an empty rolling-median state.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self {
            window: SortedRing::new(timeperiod),
            timeperiod,
            value: None,
        })
    }

    /// Appends one observation and returns the median after warm-up.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.window.push(input);
        self.value = if self.window.is_full() {
            let sorted = self.window.sorted();
            let middle = self.timeperiod / 2;
            Some(if self.timeperiod % 2 == 1 {
                sorted[middle]
            } else {
                (sorted[middle - 1] + sorted[middle]) * 0.5
            })
        } else {
            None
        };
        self.value
    }

    /// Returns the latest median, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clears the trailing window and latest output.
    pub fn reset(&mut self) {
        self.window.clear();
        self.value = None;
    }
}

/// Rolling median. Warm-up values are `NaN`; even windows average the two
/// Compute the rolling median result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_median(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_period(timeperiod)?;
    let mut state = RollingMedian::new(timeperiod)?;
    Ok(input
        .iter()
        .map(|&value| state.append(value).unwrap_or(f64::NAN))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// Verbatim pre-sorted-ring implementation, kept as the bitwise oracle.
    struct OldRollingMedian {
        values: VecDeque<f64>,
        timeperiod: usize,
    }

    impl OldRollingMedian {
        fn new(timeperiod: usize) -> Self {
            Self {
                values: VecDeque::with_capacity(timeperiod),
                timeperiod,
            }
        }

        fn append(&mut self, input: f64) -> Option<f64> {
            if self.values.len() == self.timeperiod {
                self.values.pop_front();
            }
            self.values.push_back(input);
            if self.values.len() == self.timeperiod {
                let mut sorted: Vec<f64> = self.values.iter().copied().collect();
                sorted.sort_by(f64::total_cmp);
                let middle = self.timeperiod / 2;
                Some(if self.timeperiod % 2 == 1 {
                    sorted[middle]
                } else {
                    (sorted[middle - 1] + sorted[middle]) * 0.5
                })
            } else {
                None
            }
        }
    }

    fn lcg_bars(n: usize) -> Vec<f64> {
        let mut state = 0x853C49E6748FEA9Bu64;
        (0..n)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                // Quantized so exact-value ties occur.
                ((state >> 33) % 199) as f64 * 0.125 - 12.0
            })
            .collect()
    }

    fn assert_bits(a: Option<f64>, b: Option<f64>, i: usize, p: usize) {
        let a = a.unwrap_or(f64::NAN);
        let b = b.unwrap_or(f64::NAN);
        assert_eq!(a.to_bits(), b.to_bits(), "bar {i} period {p}: {a} vs {b}");
    }

    #[test]
    fn bitwise_matches_old_implementation() {
        let bars = lcg_bars(5_000);
        for period in [2usize, 5, 14, 30, 200] {
            // Bulk (per-bar) vs old oracle.
            let mut old = OldRollingMedian::new(period);
            let expected: Vec<Option<f64>> = bars.iter().map(|&v| old.append(v)).collect();
            let mut state = RollingMedian::new(period).unwrap();
            for (i, &v) in bars.iter().enumerate() {
                assert_bits(state.append(v), expected[i], i, period);
            }
            // Batch helper.
            let batch = rolling_median(&bars, period).unwrap();
            for (i, &v) in batch.iter().enumerate() {
                assert_bits(Some(v), expected[i], i, period);
            }
            // Chunked feeds (1/7/97) stay invariant.
            for chunk in [1usize, 7, 97] {
                let mut state = RollingMedian::new(period).unwrap();
                let mut i = 0;
                for block in bars.chunks(chunk) {
                    for &v in block {
                        assert_bits(state.append(v), expected[i], i, period);
                        i += 1;
                    }
                }
            }
            // Continue after bulk.
            let (head, tail) = bars.split_at(4_000);
            let mut state = RollingMedian::new(period).unwrap();
            let _ = rolling_median(head, period).unwrap();
            for &v in head {
                state.append(v);
            }
            for (j, &v) in tail.iter().enumerate() {
                assert_bits(state.append(v), expected[4_000 + j], 4_000 + j, period);
            }
            // Reset reuses the state cleanly.
            state.reset();
            for (i, &v) in bars.iter().take(500).enumerate() {
                assert_bits(state.append(v), expected[i], i, period);
            }
        }
    }
}
