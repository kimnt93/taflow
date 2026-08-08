//! Batch implementation for `roll_spread`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `roll_spread` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn roll_spread(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = RollSpread::new(timeperiod)?;
    Ok(input
        .iter()
        .map(|&price| state.append(price).unwrap_or(f64::NAN))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// The pre-optimisation `RollSpread` + `RollingPairMoments` bodies, kept
    /// verbatim as the oracle (two parallel deques).
    struct Oracle {
        x: VecDeque<f64>,
        y: VecDeque<f64>,
        timeperiod: usize,
        covariance: Option<f64>,
        previous_price: Option<f64>,
        delta_previous: Option<f64>,
    }

    impl Oracle {
        fn new(timeperiod: usize) -> Self {
            Self {
                x: VecDeque::with_capacity(timeperiod),
                y: VecDeque::with_capacity(timeperiod),
                timeperiod,
                covariance: None,
                previous_price: None,
                delta_previous: None,
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
            let delta = if let Some(previous_price) = self.previous_price.replace(price) {
                price - previous_price
            } else {
                0.0
            };
            if let Some(delta_previous) = self.delta_previous {
                self.push_moment(delta, delta_previous);
            }
            self.delta_previous = Some(delta);
            self.covariance
                .map(|cov| 2.0 * (0.0f64 - cov).max(0.0).sqrt())
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
        let input = lcg_series(5_000, 0x8011_0001);
        for period in [2usize, 3, 8, 20, 64, 251] {
            let mut state = RollSpread::new(period).unwrap();
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

    #[test]
    fn batch_matches_per_bar_streaming_bitwise() {
        let input = lcg_series(5_000, 0x8011_0002);
        for period in [2usize, 20, 64] {
            let batch = roll_spread(&input, period).unwrap();
            let mut state = RollSpread::new(period).unwrap();
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
        let mut state = RollSpread::new(20).unwrap();
        let mut oracle = Oracle::new(20);
        for bar in 0..200 {
            let actual = state.append(100.0).unwrap_or(f64::NAN);
            let expected = oracle.append(100.0).unwrap_or(f64::NAN);
            assert_eq!(actual.to_bits(), expected.to_bits(), "bar {bar}");
        }
        state.reset();
        assert!(state.value().is_none());
        let mut fresh = RollSpread::new(20).unwrap();
        for &price in &lcg_series(500, 0x8011_0003) {
            let after_reset = state.append(price).unwrap_or(f64::NAN);
            let from_fresh = fresh.append(price).unwrap_or(f64::NAN);
            assert_eq!(after_reset.to_bits(), from_fresh.to_bits());
        }
    }
}
