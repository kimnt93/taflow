//! Batch implementation for `rolling_winsorize`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `rolling_winsorize` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the rolling winsorize result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
/// * `lower` - Input series or configuration value.
/// * `upper` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_winsorize(
    input: &[f64],
    timeperiod: usize,
    lower: f64,
    upper: f64,
) -> TaResult<Vec<f64>> {
    let mut state = RollingWinsorize::new(timeperiod, lower, upper)?;
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
    struct OldRollingWinsorize {
        values: VecDeque<f64>,
        timeperiod: usize,
        lower: f64,
        upper: f64,
    }

    impl OldRollingWinsorize {
        fn new(timeperiod: usize, lower: f64, upper: f64) -> Self {
            Self {
                values: VecDeque::with_capacity(timeperiod),
                timeperiod,
                lower,
                upper,
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
                let quantile = |q: f64| {
                    let position = q * (sorted.len() - 1) as f64;
                    let lower = position.floor() as usize;
                    let upper = position.ceil() as usize;
                    sorted[lower] + (sorted[upper] - sorted[lower]) * (position - lower as f64)
                };
                Some(input.max(quantile(self.lower)).min(quantile(self.upper)))
            } else {
                None
            }
        }
    }

    fn lcg_bars(n: usize) -> Vec<f64> {
        let mut state = 0xB5297A4D94D8B4C9u64;
        (0..n)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                // Quantized so exact-value ties occur.
                ((state >> 33) % 223) as f64 * 0.03125 - 3.0
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
            for (lo, up) in [(0.05, 0.95), (0.0, 1.0), (0.25, 0.75), (0.1, 0.5)] {
                let mut old = OldRollingWinsorize::new(period, lo, up);
                let expected: Vec<Option<f64>> = bars.iter().map(|&v| old.append(v)).collect();
                let mut state = RollingWinsorize::new(period, lo, up).unwrap();
                for (i, &v) in bars.iter().enumerate() {
                    assert_bits(state.append(v), expected[i], i, period);
                }
                let batch = rolling_winsorize(&bars, period, lo, up).unwrap();
                for (i, &v) in batch.iter().enumerate() {
                    assert_bits(Some(v), expected[i], i, period);
                }
                for chunk in [1usize, 7, 97] {
                    let mut state = RollingWinsorize::new(period, lo, up).unwrap();
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
                let mut state = RollingWinsorize::new(period, lo, up).unwrap();
                for &v in head {
                    state.append(v);
                }
                for (j, &v) in tail.iter().enumerate() {
                    assert_bits(state.append(v), expected[4_000 + j], 4_000 + j, period);
                }
                state.reset();
                for (i, &v) in bars.iter().take(500).enumerate() {
                    assert_bits(state.append(v), expected[i], i, period);
                }
            }
        }
    }
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingWinsorize`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingWinsorize {
    window: super::sorted_ring::SortedRing,
    timeperiod: usize,
    lower: f64,
    upper: f64,
    value: Option<f64>,
}

impl RollingWinsorize {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize, lower: f64, upper: f64) -> TaResult<Self> {
        validate_period(timeperiod)?;
        validate_quantile(lower)?;
        validate_quantile(upper)?;
        if lower > upper {
            return Err(TaError::InvalidParameter {
                name: "lower/upper",
                value: format!("{lower}/{upper}"),
                reason: "lower must be <= upper",
            });
        }
        Ok(Self {
            window: super::sorted_ring::SortedRing::new(timeperiod),
            timeperiod,
            lower,
            upper,
            value: None,
        })
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    ///
    /// The window is a shared sorted ring; the quantile interpolation and
    /// `max`/`min` clamping are unchanged from the per-bar full-sort
    /// implementation, so outputs stay bit-identical.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.window.push(input);
        self.value = if self.window.is_full() {
            let sorted = self.window.sorted();
            let quantile = |q: f64| {
                let position = q * (sorted.len() - 1) as f64;
                let lower = position.floor() as usize;
                let upper = position.ceil() as usize;
                sorted[lower] + (sorted[upper] - sorted[lower]) * (position - lower as f64)
            };
            Some(input.max(quantile(self.lower)).min(quantile(self.upper)))
        } else {
            None
        };
        self.value
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
        self.window.clear();
        self.value = None;
    }
}
