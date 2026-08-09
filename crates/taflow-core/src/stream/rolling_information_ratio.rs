//! Batch implementation for `rolling_information_ratio`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `rolling_information_ratio` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the rolling information ratio result for the supplied aligned series.
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
pub fn rolling_information_ratio(
    input: &[f64],
    benchmark: &[f64],
    timeperiod: usize,
) -> TaResult<Vec<f64>> {
    if input.len() != benchmark.len() {
        return Err(TaError::LengthMismatch {
            expected: input.len(),
            got: benchmark.len(),
        });
    }
    let mut state = RollingInformationRatio::new(timeperiod)?;
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

    /// Pre-optimization `RollingInformationRatio::append` oracle.
    struct Reference {
        values: VecDeque<f64>,
        period: usize,
    }

    impl Reference {
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
            self.values.push_back(input - benchmark);
            (self.values.len() == self.period).then(|| {
                let n = self.period as f64;
                let mean = self.values.iter().sum::<f64>() / n;
                let variance = self
                    .values
                    .iter()
                    .map(|&value| (value - mean).powi(2))
                    .sum::<f64>()
                    / n;
                if variance > 0.0 {
                    mean / variance.sqrt()
                } else {
                    0.0
                }
            })
        }
    }

    fn lcg_series(len: usize, seed: u64) -> Vec<f64> {
        let mut state = seed;
        (0..len)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                ((state >> 11) as f64 / (1u64 << 53) as f64) * 0.05 - 0.025
            })
            .collect()
    }

    #[test]
    fn matches_reference_bitwise_and_survives_chunking() {
        let input = lcg_series(5_000, 0x90_5EED_61);
        let benchmark = lcg_series(5_000, 0x91_5EED_62);
        for period in [1usize, 2, 5, 30, 252] {
            let mut reference = Reference::new(period);
            let expected: Vec<f64> = (0..input.len())
                .map(|i| reference.append(input[i], benchmark[i]).unwrap_or(f64::NAN))
                .collect();
            let mut state = RollingInformationRatio::new(period).unwrap();
            for (i, want) in expected.iter().enumerate() {
                let got = state.append(input[i], benchmark[i]).unwrap_or(f64::NAN);
                assert_eq!(want.to_bits(), got.to_bits(), "p={period} bar {i}");
            }
            state.reset();
            let mut fresh = Reference::new(period);
            for i in 0..512 {
                let want = fresh.append(input[i], benchmark[i]).unwrap_or(f64::NAN);
                let got = state.append(input[i], benchmark[i]).unwrap_or(f64::NAN);
                assert_eq!(want.to_bits(), got.to_bits(), "p={period} post-reset {i}");
            }
        }
    }

    #[test]
    fn batch_matches_streaming() {
        let input = lcg_series(1_000, 0x92_5EED_63);
        let benchmark = lcg_series(1_000, 0x93_5EED_64);
        let batch = rolling_information_ratio(&input, &benchmark, 30).unwrap();
        let mut state = RollingInformationRatio::new(30).unwrap();
        for (i, value) in batch.iter().enumerate() {
            let got = state.append(input[i], benchmark[i]).unwrap_or(f64::NAN);
            assert_eq!(value.to_bits(), got.to_bits());
        }
    }
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingInformationRatio`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingInformationRatio {
    values: ContiguousWindow,
    period: usize,
    value: Option<f64>,
}

impl RollingInformationRatio {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            values: ContiguousWindow::new(period),
            period,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    /// The variance pass needs the window mean, so the two passes cannot be
    /// collapsed into sliding sums without changing the summation order (and
    /// therefore the low bits). Both passes now walk one contiguous ring
    /// slice, so the second pass reads cache-hot memory.
    pub fn append(&mut self, input: f64, benchmark: f64) -> Option<f64> {
        self.values.push(input - benchmark);
        self.value = self.values.is_full().then(|| {
            let window = self.values.window();
            let n = self.period as f64;
            let mean = window.iter().sum::<f64>() / n;
            let variance = window
                .iter()
                .map(|&value| (value - mean).powi(2))
                .sum::<f64>()
                / n;
            if variance > 0.0 {
                mean / variance.sqrt()
            } else {
                0.0
            }
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
