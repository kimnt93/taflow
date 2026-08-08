//! Batch implementation for `hull_moving_average`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes the causal hull moving average series.
/// Parameters: aligned input slices followed by indicator parameters.
/// Compute the hull moving average result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn hull_moving_average(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = HullMovingAverage::new(timeperiod)?;
    Ok(input
        .iter()
        .map(|&v| state.append(v).unwrap_or(f64::NAN))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    fn weighted_mean(values: &VecDeque<f64>) -> f64 {
        let denominator = (values.len() * (values.len() + 1) / 2) as f64;
        values
            .iter()
            .enumerate()
            .map(|(i, &v)| v * (i + 1) as f64)
            .sum::<f64>()
            / denominator
    }

    /// Pre-optimization `HullMovingAverage::append`, kept verbatim as oracle.
    struct Reference {
        raw: VecDeque<f64>,
        intermediate: VecDeque<f64>,
        period: usize,
        half: usize,
        smooth: usize,
    }

    impl Reference {
        fn new(period: usize) -> Self {
            let half = (period / 2).max(1);
            let smooth = (period as f64).sqrt().floor() as usize;
            Self {
                raw: VecDeque::with_capacity(period),
                intermediate: VecDeque::with_capacity(smooth.max(1)),
                period,
                half,
                smooth: smooth.max(1),
            }
        }

        fn append(&mut self, input: f64) -> Option<f64> {
            if self.raw.len() == self.period {
                self.raw.pop_front();
            }
            self.raw.push_back(input);
            if self.raw.len() >= self.half && self.raw.len() >= self.period {
                let half = weighted_mean(
                    &self
                        .raw
                        .iter()
                        .skip(self.period - self.half)
                        .copied()
                        .collect(),
                );
                let full = weighted_mean(&self.raw);
                if self.intermediate.len() == self.smooth {
                    self.intermediate.pop_front();
                }
                self.intermediate.push_back(2.0 * half - full);
                (self.intermediate.len() == self.smooth).then(|| weighted_mean(&self.intermediate))
            } else {
                None
            }
        }
    }

    fn lcg_series(len: usize, seed: u64) -> Vec<f64> {
        let mut state = seed;
        (0..len)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                90.0 + ((state >> 11) as f64 / (1u64 << 53) as f64) * 20.0
            })
            .collect()
    }

    #[test]
    fn matches_reference_bitwise_and_survives_chunking() {
        let input = lcg_series(5_000, 0x40_5EED_11);
        for period in [1usize, 2, 4, 9, 16, 20, 55, 200] {
            let mut reference = Reference::new(period);
            let expected: Vec<f64> = input
                .iter()
                .map(|&v| reference.append(v).unwrap_or(f64::NAN))
                .collect();
            let mut state = HullMovingAverage::new(period).unwrap();
            for (i, want) in expected.iter().enumerate() {
                let got = state.append(input[i]).unwrap_or(f64::NAN);
                assert_eq!(want.to_bits(), got.to_bits(), "p={period} bar {i}");
            }
            state.reset();
            let mut fresh = Reference::new(period);
            for &v in input.iter().take(512) {
                let want = fresh.append(v).unwrap_or(f64::NAN);
                let got = state.append(v).unwrap_or(f64::NAN);
                assert_eq!(want.to_bits(), got.to_bits(), "p={period} post-reset");
            }
        }
    }

    #[test]
    fn batch_matches_streaming() {
        let input = lcg_series(1_000, 0x41_5EED_12);
        let batch = hull_moving_average(&input, 16).unwrap();
        let mut state = HullMovingAverage::new(16).unwrap();
        for (i, value) in batch.iter().enumerate() {
            let got = state.append(input[i]).unwrap_or(f64::NAN);
            assert_eq!(value.to_bits(), got.to_bits());
        }
    }
}
