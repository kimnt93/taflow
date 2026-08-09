//! Batch implementation for `rolling_alpha`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `rolling_alpha` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the rolling alpha result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `benchmark` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_alpha(input: &[f64], benchmark: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if input.len() != benchmark.len() {
        return Err(TaError::LengthMismatch {
            expected: input.len(),
            got: benchmark.len(),
        });
    }
    let mut state = RollingAlpha::new(timeperiod)?;
    Ok(input
        .iter()
        .zip(benchmark)
        .map(|(&input, &benchmark)| state.append(input, benchmark).unwrap_or(f64::NAN))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// The pre-optimisation `RollingAlpha::append` body, kept verbatim.
    struct Oracle {
        values: VecDeque<(f64, f64)>,
        period: usize,
    }

    impl Oracle {
        fn new(period: usize) -> Self {
            Self {
                values: VecDeque::with_capacity(period),
                period,
            }
        }

        fn append(&mut self, input: f64, benchmark: f64) -> Option<f64> {
            if self.values.len() == self.period {
                self.values.pop_front();
            }
            self.values.push_back((input, benchmark));
            (self.values.len() == self.period).then(|| {
                let n = self.period as f64;
                let mean_input = self.values.iter().map(|&(input, _)| input).sum::<f64>() / n;
                let mean_benchmark = self
                    .values
                    .iter()
                    .map(|&(_, benchmark)| benchmark)
                    .sum::<f64>()
                    / n;
                let covariance = self
                    .values
                    .iter()
                    .map(|&(input, benchmark)| (input - mean_input) * (benchmark - mean_benchmark))
                    .sum::<f64>();
                let variance = self
                    .values
                    .iter()
                    .map(|&(_, benchmark)| (benchmark - mean_benchmark).powi(2))
                    .sum::<f64>();
                let beta = if variance > 0.0 {
                    covariance / variance
                } else {
                    0.0
                };
                mean_input - beta * mean_benchmark
            })
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
    fn streaming_matches_the_previous_scan_bitwise() {
        let input = lcg_series(5_000, 0xA1FA_0001);
        let benchmark = lcg_series(5_000, 0xA1FA_0002);
        for period in [1usize, 2, 8, 20, 64, 251] {
            let mut state = RollingAlpha::new(period).unwrap();
            let mut oracle = Oracle::new(period);
            for bar in 0..input.len() {
                let actual = state.append(input[bar], benchmark[bar]).unwrap_or(f64::NAN);
                let expected = oracle
                    .append(input[bar], benchmark[bar])
                    .unwrap_or(f64::NAN);
                assert_eq!(
                    actual.to_bits(),
                    expected.to_bits(),
                    "period {period} bar {bar}"
                );
            }
        }
    }

    #[test]
    fn batch_matches_per_bar_streaming_bitwise() {
        let input = lcg_series(5_000, 0xA1FA_0003);
        let benchmark = lcg_series(5_000, 0xA1FA_0004);
        for period in [1usize, 20, 64] {
            let batch = rolling_alpha(&input, &benchmark, period).unwrap();
            let mut state = RollingAlpha::new(period).unwrap();
            for bar in 0..input.len() {
                let expected = state.append(input[bar], benchmark[bar]).unwrap_or(f64::NAN);
                assert_eq!(
                    batch[bar].to_bits(),
                    expected.to_bits(),
                    "period {period} bar {bar}"
                );
            }
        }
    }

    #[test]
    fn zero_variance_and_reset_paths_are_preserved() {
        let mut state = RollingAlpha::new(20).unwrap();
        let mut oracle = Oracle::new(20);
        let input = lcg_series(200, 0xA1FA_0005);
        for bar in 0..input.len() {
            let actual = state.append(input[bar], 100.0).unwrap_or(f64::NAN);
            let expected = oracle.append(input[bar], 100.0).unwrap_or(f64::NAN);
            assert_eq!(actual.to_bits(), expected.to_bits(), "bar {bar}");
        }
        state.reset();
        assert!(state.value().is_none());
        let mut fresh = RollingAlpha::new(20).unwrap();
        let a = lcg_series(500, 0xA1FA_0006);
        let b = lcg_series(500, 0xA1FA_0007);
        for bar in 0..a.len() {
            let after_reset = state.append(a[bar], b[bar]).unwrap_or(f64::NAN);
            let from_fresh = fresh.append(a[bar], b[bar]).unwrap_or(f64::NAN);
            assert_eq!(after_reset.to_bits(), from_fresh.to_bits());
        }
    }
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingAlpha`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingAlpha {
    values: VecDeque<(f64, f64)>,
    period: usize,
    value: Option<f64>,
}

impl RollingAlpha {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            values: VecDeque::with_capacity(period),
            period,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, input: f64, benchmark: f64) -> Option<f64> {
        if self.values.len() == self.period {
            self.values.pop_front();
        }
        self.values.push_back((input, benchmark));
        self.value = (self.values.len() == self.period).then(|| {
            let n = self.period as f64;
            // Contiguous two-slice scans with fused accumulators: each
            // accumulator adds the same terms in the same order as the
            // original per-quantity passes, so results are bit-identical.
            let (front, back) = self.values.as_slices();
            let mut sum_input = 0.0;
            let mut sum_benchmark = 0.0;
            for &(input, benchmark) in front {
                sum_input += input;
                sum_benchmark += benchmark;
            }
            for &(input, benchmark) in back {
                sum_input += input;
                sum_benchmark += benchmark;
            }
            let mean_input = sum_input / n;
            let mean_benchmark = sum_benchmark / n;
            let mut covariance = 0.0;
            let mut variance = 0.0;
            for &(input, benchmark) in front {
                let delta_benchmark = benchmark - mean_benchmark;
                covariance += (input - mean_input) * delta_benchmark;
                variance += delta_benchmark * delta_benchmark;
            }
            for &(input, benchmark) in back {
                let delta_benchmark = benchmark - mean_benchmark;
                covariance += (input - mean_input) * delta_benchmark;
                variance += delta_benchmark * delta_benchmark;
            }
            let beta = if variance > 0.0 {
                covariance / variance
            } else {
                0.0
            };
            mean_input - beta * mean_benchmark
        });
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
        self.values.clear();
        self.value = None;
    }
}
