//! Batch implementation for `spread_zscore`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `spread_zscore` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn spread_zscore(x: &[f64], y: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if x.len() != y.len() {
        return Err(TaError::LengthMismatch {
            expected: x.len(),
            got: y.len(),
        });
    }
    let mut state = SpreadZScore::new(timeperiod)?;
    Ok(x.iter()
        .zip(y)
        .map(|(&x, &y)| state.append(x, y).unwrap_or(f64::NAN))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// The pre-optimisation `SpreadZScore::append` body, kept verbatim.
    struct Oracle {
        values: VecDeque<(f64, f64)>,
        timeperiod: usize,
    }

    impl Oracle {
        fn new(timeperiod: usize) -> Self {
            Self {
                values: VecDeque::with_capacity(timeperiod),
                timeperiod,
            }
        }

        fn append(&mut self, x: f64, y: f64) -> Option<f64> {
            if self.values.len() == self.timeperiod {
                self.values.pop_front();
            }
            self.values.push_back((x, y));
            if self.values.len() == self.timeperiod {
                let n = self.timeperiod as f64;
                let mean_x = self.values.iter().map(|&(x, _)| x).sum::<f64>() / n;
                let mean_y = self.values.iter().map(|&(_, y)| y).sum::<f64>() / n;
                let covariance = self
                    .values
                    .iter()
                    .map(|&(x, y)| (x - mean_x) * (y - mean_y))
                    .sum::<f64>();
                let variance = self
                    .values
                    .iter()
                    .map(|&(x, _)| (x - mean_x).powi(2))
                    .sum::<f64>();
                let beta = if variance > 0.0 {
                    covariance / variance
                } else {
                    0.0
                };
                let spread = y - beta * x;
                let mean_spread = self.values.iter().map(|&(x, y)| y - beta * x).sum::<f64>() / n;
                let std_spread = (self
                    .values
                    .iter()
                    .map(|&(x, y)| (y - beta * x - mean_spread).powi(2))
                    .sum::<f64>()
                    / n)
                    .sqrt();
                Some(if std_spread > 0.0 {
                    (spread - mean_spread) / std_spread
                } else {
                    0.0
                })
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

    #[test]
    fn streaming_matches_the_previous_scan_bitwise() {
        let x = lcg_series(5_000, 0x2C00_0001);
        let y = lcg_series(5_000, 0x2C00_0002);
        for period in [1usize, 2, 8, 20, 64, 251] {
            let mut state = SpreadZScore::new(period).unwrap();
            let mut oracle = Oracle::new(period);
            for bar in 0..x.len() {
                let actual = state.append(x[bar], y[bar]).unwrap_or(f64::NAN);
                let expected = oracle.append(x[bar], y[bar]).unwrap_or(f64::NAN);
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
        let x = lcg_series(5_000, 0x2C00_0003);
        let y = lcg_series(5_000, 0x2C00_0004);
        for period in [1usize, 20, 64] {
            let batch = spread_zscore(&x, &y, period).unwrap();
            let mut state = SpreadZScore::new(period).unwrap();
            for bar in 0..x.len() {
                let expected = state.append(x[bar], y[bar]).unwrap_or(f64::NAN);
                assert_eq!(
                    batch[bar].to_bits(),
                    expected.to_bits(),
                    "period {period} bar {bar}"
                );
            }
        }
    }

    #[test]
    fn degenerate_and_reset_paths_are_preserved() {
        // Constant x drives the `variance == 0` beta branch; constant y then
        // drives the `std_spread == 0` branch.
        let mut state = SpreadZScore::new(20).unwrap();
        let mut oracle = Oracle::new(20);
        for bar in 0..200 {
            let actual = state.append(100.0, 50.0).unwrap_or(f64::NAN);
            let expected = oracle.append(100.0, 50.0).unwrap_or(f64::NAN);
            assert_eq!(actual.to_bits(), expected.to_bits(), "bar {bar}");
        }
        state.reset();
        assert!(state.value().is_none());
        let mut fresh = SpreadZScore::new(20).unwrap();
        let x = lcg_series(500, 0x2C00_0005);
        let y = lcg_series(500, 0x2C00_0006);
        for bar in 0..x.len() {
            let after_reset = state.append(x[bar], y[bar]).unwrap_or(f64::NAN);
            let from_fresh = fresh.append(x[bar], y[bar]).unwrap_or(f64::NAN);
            assert_eq!(after_reset.to_bits(), from_fresh.to_bits());
        }
    }
}
