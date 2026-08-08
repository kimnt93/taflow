//! Batch implementation for `hedge_ratio`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Compute the hedge ratio result for the supplied aligned series.
///
/// # Parameters
///
/// * `x` - Input series or configuration value.
/// * `y` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn hedge_ratio(x: &[f64], y: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if x.len() != y.len() {
        return Err(TaError::LengthMismatch {
            expected: x.len(),
            got: y.len(),
        });
    }
    let mut state = HedgeRatio::new(timeperiod)?;
    Ok(x.iter()
        .zip(y)
        .map(|(&x, &y)| state.append(x, y).unwrap_or(f64::NAN))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// The pre-optimisation `HedgeRatio::append` body, kept verbatim.
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

        fn append(&mut self, x: f64, y: f64) -> Option<f64> {
            if self.values.len() == self.period {
                self.values.pop_front();
            }
            self.values.push_back((x, y));
            if self.values.len() == self.period {
                let n = self.period as f64;
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
                Some(if variance > 0.0 {
                    covariance / variance
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
        let x = lcg_series(5_000, 0x4EDE_0001);
        let y = lcg_series(5_000, 0x4EDE_0002);
        for period in [1usize, 2, 8, 20, 64, 251] {
            let mut state = HedgeRatio::new(period).unwrap();
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
        let x = lcg_series(5_000, 0x4EDE_0003);
        let y = lcg_series(5_000, 0x4EDE_0004);
        for period in [1usize, 20, 64] {
            let batch = hedge_ratio(&x, &y, period).unwrap();
            let mut state = HedgeRatio::new(period).unwrap();
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
    fn zero_variance_and_reset_paths_are_preserved() {
        let mut state = HedgeRatio::new(20).unwrap();
        let mut oracle = Oracle::new(20);
        let y = lcg_series(200, 0x4EDE_0005);
        for bar in 0..y.len() {
            let actual = state.append(100.0, y[bar]).unwrap_or(f64::NAN);
            let expected = oracle.append(100.0, y[bar]).unwrap_or(f64::NAN);
            assert_eq!(actual.to_bits(), expected.to_bits(), "bar {bar}");
        }
        state.reset();
        assert!(state.value().is_none());
        let mut fresh = HedgeRatio::new(20).unwrap();
        let x = lcg_series(500, 0x4EDE_0006);
        let y = lcg_series(500, 0x4EDE_0007);
        for bar in 0..x.len() {
            let after_reset = state.append(x[bar], y[bar]).unwrap_or(f64::NAN);
            let from_fresh = fresh.append(x[bar], y[bar]).unwrap_or(f64::NAN);
            assert_eq!(after_reset.to_bits(), from_fresh.to_bits());
        }
    }
}
