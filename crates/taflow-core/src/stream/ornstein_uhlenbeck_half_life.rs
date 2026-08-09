//! Batch implementation for `ornstein_uhlenbeck_half_life`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes the Ornstein-Uhlenbeck mean-reversion half-life.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn ornstein_uhlenbeck_half_life(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = OrnsteinUhlenbeckHalfLife::new(timeperiod)?;
    Ok(input
        .iter()
        .map(|&price| state.append(price).unwrap_or(f64::NAN))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// The pre-optimisation `OrnsteinUhlenbeckHalfLife` + `RollingPairMoments`
    /// bodies, kept verbatim as the oracle (two parallel deques, `var_y`
    /// rescanned in the consumer).
    struct Oracle {
        x: VecDeque<f64>,
        y: VecDeque<f64>,
        timeperiod: usize,
        covariance: Option<f64>,
        previous_price: Option<f64>,
    }

    impl Oracle {
        fn new(timeperiod: usize) -> Self {
            Self {
                x: VecDeque::with_capacity(timeperiod),
                y: VecDeque::with_capacity(timeperiod),
                timeperiod,
                covariance: None,
                previous_price: None,
            }
        }

        fn push_moment(&mut self, x: f64, y: f64) {
            if self.x.len() == self.timeperiod {
                self.x.pop_front();
                self.y.pop_front();
            }
            self.x.push_back(x);
            self.y.push_back(y);
            self.covariance = if self.x.len() == self.timeperiod {
                let n = self.timeperiod as f64;
                let mean_x = self.x.iter().sum::<f64>() / n;
                let mean_y = self.y.iter().sum::<f64>() / n;
                let mut cov = 0.0;
                for (x, y) in self.x.iter().zip(self.y.iter()) {
                    cov += (x - mean_x) * (y - mean_y);
                }
                Some(cov / (n - 1.0))
            } else {
                None
            };
        }

        fn append(&mut self, price: f64) -> Option<f64> {
            if let Some(previous_price) = self.previous_price.replace(price) {
                let delta = price - previous_price;
                self.push_moment(delta, previous_price);
            }
            if let Some(cov) = self.covariance {
                let n = self.timeperiod as f64;
                let mean_y = self.y.iter().sum::<f64>() / n;
                let var_y = self
                    .y
                    .iter()
                    .map(|&y| (y - mean_y) * (y - mean_y))
                    .sum::<f64>()
                    / (n - 1.0);
                if var_y > 0.0 {
                    let lambda = -cov / var_y;
                    (lambda > 0.0).then_some(2.0f64.ln() / lambda)
                } else {
                    None
                }
            } else {
                None
            }
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

    /// Mean-reverting series so the `lambda > 0` branch is exercised too.
    fn mean_reverting(n: usize, seed: u64) -> Vec<f64> {
        let noise = lcg_series(n, seed);
        let mut level = 100.0;
        noise
            .iter()
            .map(|&sample| {
                level += 0.25 * (100.0 - level) + (sample - 100.0) * 0.1;
                level
            })
            .collect()
    }

    #[test]
    fn streaming_matches_the_previous_scan_bitwise() {
        for input in [
            lcg_series(5_000, 0x00A0_0001),
            mean_reverting(5_000, 0x00A0_0002),
        ] {
            for period in [2usize, 3, 8, 20, 64, 251] {
                let mut state = OrnsteinUhlenbeckHalfLife::new(period).unwrap();
                let mut oracle = Oracle::new(period);
                for (bar, &price) in input.iter().enumerate() {
                    let actual = state.append(price).unwrap_or(f64::NAN);
                    let expected = oracle.append(price).unwrap_or(f64::NAN);
                    assert_eq!(
                        actual.to_bits(),
                        expected.to_bits(),
                        "period {period} bar {bar}"
                    );
                }
            }
        }
    }

    #[test]
    fn batch_matches_per_bar_streaming_bitwise() {
        let input = mean_reverting(5_000, 0x00A0_0003);
        for period in [2usize, 20, 64] {
            let batch = ornstein_uhlenbeck_half_life(&input, period).unwrap();
            let mut state = OrnsteinUhlenbeckHalfLife::new(period).unwrap();
            for (bar, &price) in input.iter().enumerate() {
                let expected = state.append(price).unwrap_or(f64::NAN);
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
        // Constant prices give `var_y == 0`, the `None` branch.
        let mut state = OrnsteinUhlenbeckHalfLife::new(20).unwrap();
        let mut oracle = Oracle::new(20);
        for bar in 0..200 {
            let actual = state.append(100.0).unwrap_or(f64::NAN);
            let expected = oracle.append(100.0).unwrap_or(f64::NAN);
            assert_eq!(actual.to_bits(), expected.to_bits(), "bar {bar}");
        }
        state.reset();
        assert!(state.value().is_none());
        let mut fresh = OrnsteinUhlenbeckHalfLife::new(20).unwrap();
        for &price in &mean_reverting(500, 0x00A0_0004) {
            let after_reset = state.append(price).unwrap_or(f64::NAN);
            let from_fresh = fresh.append(price).unwrap_or(f64::NAN);
            assert_eq!(after_reset.to_bits(), from_fresh.to_bits());
        }
    }
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// OU half-life: `−ln(2)/λ` where `λ` is the slope of `Δp` on lagged `p`.
/// `λ ≥ 0` yields `NaN`.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `OrnsteinUhlenbeckHalfLife`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct OrnsteinUhlenbeckHalfLife {
    moments: RollingPairMoments,
    previous_price: Option<f64>,
    value: Option<f64>,
}

impl OrnsteinUhlenbeckHalfLife {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            moments: RollingPairMoments::new(timeperiod)?,
            previous_price: None,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, price: f64) -> Option<f64> {
        if let Some(previous_price) = self.previous_price.replace(price) {
            let delta = price - previous_price;
            let _ = self.moments.append(delta, previous_price);
        }
        self.value = if let Some(cov) = self.moments.value() {
            // `var_y` is computed inside `RollingPairMoments::append` from the
            // same window with the same summation order as the scans this
            // replaced, so the result is bit-identical.
            let var_y = self.moments.var_y;
            if var_y > 0.0 {
                let lambda = -cov / var_y;
                (lambda > 0.0).then_some(2.0f64.ln() / lambda)
            } else {
                None
            }
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
        self.moments.reset();
        self.previous_price = None;
        self.value = None;
    }
}
