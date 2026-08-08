//! Batch implementation for `hurst`.

use super::operator_states::*;
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
