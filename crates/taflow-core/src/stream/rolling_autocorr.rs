//! Batch implementation for `rolling_autocorr`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `rolling_autocorr` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the rolling autocorr result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_autocorr(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = RollingAutocorr::new(timeperiod)?;
    Ok(input
        .iter()
        .map(|&value| state.append(value).unwrap_or(f64::NAN))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// The pre-optimisation `RollingAutocorr::append` body, kept verbatim.
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
                let left_n = (self.period - 1) as f64;
                let left_mean = self.values.iter().take(self.period - 1).sum::<f64>() / left_n;
                let right_mean = self.values.iter().skip(1).sum::<f64>() / left_n;
                let left_variance = self
                    .values
                    .iter()
                    .take(self.period - 1)
                    .map(|&value| (value - left_mean).powi(2))
                    .sum::<f64>();
                let right_variance = self
                    .values
                    .iter()
                    .skip(1)
                    .map(|&value| (value - right_mean).powi(2))
                    .sum::<f64>();
                if left_variance == 0.0 || right_variance == 0.0 {
                    return 0.0;
                }
                let covariance = self
                    .values
                    .iter()
                    .take(self.period - 1)
                    .zip(self.values.iter().skip(1))
                    .map(|(&left, &right)| (left - left_mean) * (right - right_mean))
                    .sum::<f64>();
                covariance / (left_variance * right_variance).sqrt()
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
        let input = lcg_series(5_000, 0xAC00_0001);
        for period in [2usize, 3, 8, 20, 64, 251] {
            let mut state = RollingAutocorr::new(period).unwrap();
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
        let input = lcg_series(5_000, 0xAC00_0002);
        for period in [2usize, 20, 64] {
            let batch = rolling_autocorr(&input, period).unwrap();
            let mut state = RollingAutocorr::new(period).unwrap();
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
    fn zero_variance_and_reset_paths_are_preserved() {
        let mut state = RollingAutocorr::new(20).unwrap();
        let mut oracle = Oracle::new(20);
        for &value in &vec![100.0; 200] {
            let actual = state.append(value).unwrap_or(f64::NAN);
            let expected = oracle.append(value).unwrap_or(f64::NAN);
            assert_eq!(actual.to_bits(), expected.to_bits());
        }
        state.reset();
        assert!(state.value().is_none());
        let mut fresh = RollingAutocorr::new(20).unwrap();
        for &value in &lcg_series(500, 0xAC00_0003) {
            let after_reset = state.append(value).unwrap_or(f64::NAN);
            let from_fresh = fresh.append(value).unwrap_or(f64::NAN);
            assert_eq!(after_reset.to_bits(), from_fresh.to_bits());
        }
    }
}
