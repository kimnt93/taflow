//! Batch implementation for `hurst`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `hurst` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the hurst result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn hurst(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = Hurst::new(timeperiod)?;
    Ok(input
        .iter()
        .map(|&value| state.append(value).unwrap_or(f64::NAN))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// The pre-optimisation `Hurst::append` body, kept verbatim as the oracle.
    struct Oracle {
        values: VecDeque<f64>,
        period: usize,
    }

    impl Oracle {
        fn new(period: usize) -> Self {
            Self {
                values: VecDeque::with_capacity(period),
                period,
            }
        }

        fn append(&mut self, input: f64) -> Option<f64> {
            if self.values.len() == self.period {
                self.values.pop_front();
            }
            self.values.push_back(input);
            (self.values.len() == self.period).then(|| {
                let n = self.period as f64;
                let mean = self.values.iter().sum::<f64>() / n;
                let mut cumulative = 0.0;
                let mut minimum = f64::INFINITY;
                let mut maximum = f64::NEG_INFINITY;
                for &value in &self.values {
                    cumulative += value - mean;
                    minimum = minimum.min(cumulative);
                    maximum = maximum.max(cumulative);
                }
                let standard_deviation = (self
                    .values
                    .iter()
                    .map(|&value| (value - mean).powi(2))
                    .sum::<f64>()
                    / n)
                    .sqrt();
                let rescaled_range = (maximum - minimum) / standard_deviation;
                if rescaled_range > 0.0 {
                    (rescaled_range.ln() / n.ln()).clamp(0.0, 1.0)
                } else {
                    0.5
                }
            })
        }
    }

    pub(crate) fn lcg_series(n: usize, mut state: u64) -> Vec<f64> {
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
        let input = lcg_series(5_000, 0x4057_0001);
        for period in [2usize, 3, 8, 20, 64, 251] {
            let mut state = Hurst::new(period).unwrap();
            let mut oracle = Oracle::new(period);
            for (bar, &value) in input.iter().enumerate() {
                let actual = state.append(value).unwrap_or(f64::NAN);
                let expected = oracle.append(value).unwrap_or(f64::NAN);
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
        let input = lcg_series(5_000, 0x4057_0002);
        for period in [2usize, 20, 64] {
            let batch = hurst(&input, period).unwrap();
            let mut state = Hurst::new(period).unwrap();
            for (bar, &value) in input.iter().enumerate() {
                let expected = state.append(value).unwrap_or(f64::NAN);
                assert_eq!(
                    batch[bar].to_bits(),
                    expected.to_bits(),
                    "period {period} bar {bar}"
                );
            }
        }
    }

    #[test]
    fn constant_and_reset_paths_are_preserved() {
        let flat = vec![100.0; 200];
        let mut state = Hurst::new(20).unwrap();
        let mut oracle = Oracle::new(20);
        for &value in &flat {
            let actual = state.append(value).unwrap_or(f64::NAN);
            let expected = oracle.append(value).unwrap_or(f64::NAN);
            assert_eq!(actual.to_bits(), expected.to_bits());
        }
        state.reset();
        assert!(state.value().is_none());
        let mut fresh = Hurst::new(20).unwrap();
        for &value in &lcg_series(500, 0x4057_0003) {
            let after_reset = state.append(value).unwrap_or(f64::NAN);
            let from_fresh = fresh.append(value).unwrap_or(f64::NAN);
            assert_eq!(after_reset.to_bits(), from_fresh.to_bits());
        }
    }
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Hurst`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Hurst {
    values: VecDeque<f64>,
    period: usize,
    /// `ln(period)`, invariant for the lifetime of the state; computing it
    /// once at construction removes one `ln` call per bar.
    log_period: f64,
    value: Option<f64>,
}

impl Hurst {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: period.to_string(),
                reason: "must be >= 2",
            });
        }
        Ok(Self {
            values: VecDeque::with_capacity(period),
            period,
            log_period: (period as f64).ln(),
            value: None,
        })
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    /// Compute the append result for the supplied aligned series.
    ///
    /// # Parameters
    ///
    /// * `&mut self` - Input series or configuration value.
    /// * `input` - Input series or configuration value.
    ///
    /// # Returns
    ///
    /// An aligned result with TA-Lib-compatible validation and warm-up values.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.period {
            self.values.pop_front();
        }
        self.values.push_back(input);
        let log_period = self.log_period;
        self.value = (self.values.len() == self.period).then(|| {
            let n = self.period as f64;
            // Contiguous two-slice scans; the R/S walk and the squared-deviation
            // sum are fused into one pass with independent accumulators, each
            // adding the same terms in the same order as before (bit-identical).
            let (front, back) = self.values.as_slices();
            let mut sum = 0.0;
            for &value in front {
                sum += value;
            }
            for &value in back {
                sum += value;
            }
            let mean = sum / n;
            let mut cumulative = 0.0;
            let mut minimum = f64::INFINITY;
            let mut maximum = f64::NEG_INFINITY;
            let mut squared = 0.0;
            // Plain comparisons instead of `f64::min`/`f64::max`: the
            // accumulators start at `±INFINITY` and `f64::min`/`max` never
            // return NaN when one operand is non-NaN, so they can never hold
            // NaN. For a non-NaN accumulator the two forms agree on every
            // input including NaN (`NaN < minimum` is false, and
            // `f64::min(minimum, NaN) == minimum`), so this is bit-identical
            // while dropping the NaN fix-up from the dependency chain.
            for &value in front {
                let deviation = value - mean;
                cumulative += deviation;
                if cumulative < minimum {
                    minimum = cumulative;
                }
                if cumulative > maximum {
                    maximum = cumulative;
                }
                squared += deviation * deviation;
            }
            for &value in back {
                let deviation = value - mean;
                cumulative += deviation;
                if cumulative < minimum {
                    minimum = cumulative;
                }
                if cumulative > maximum {
                    maximum = cumulative;
                }
                squared += deviation * deviation;
            }
            let standard_deviation = (squared / n).sqrt();
            let rescaled_range = (maximum - minimum) / standard_deviation;
            if rescaled_range > 0.0 {
                // `log_period` is `(period as f64).ln()` computed once at
                // construction — the same value `n.ln()` produced per bar.
                (rescaled_range.ln() / log_period).clamp(0.0, 1.0)
            } else {
                0.5
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
