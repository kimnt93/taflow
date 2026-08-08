//! Batch implementation for `ulcer_index`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Compute the causal ulcer index for an aligned price series.
///
/// Parameters are the input prices and rolling period; the returned vector
/// Compute the ulcer index result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn ulcer_index(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = UlcerIndex::new(timeperiod)?;
    Ok(input
        .iter()
        .map(|&value| state.append(value).unwrap_or(f64::NAN))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// Pre-optimization `UlcerIndex::append` oracle.
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

        fn append(&mut self, input: f64) -> Option<f64> {
            if self.values.len() == self.period {
                self.values.pop_front();
            }
            self.values.push_back(input);
            (self.values.len() == self.period).then(|| {
                let mut peak = f64::NEG_INFINITY;
                let sum = self
                    .values
                    .iter()
                    .map(|&v| {
                        peak = peak.max(v);
                        let drawdown = if peak != 0.0 {
                            100.0 * (v - peak) / peak
                        } else {
                            0.0
                        };
                        drawdown * drawdown
                    })
                    .sum::<f64>();
                (sum / self.period as f64).sqrt()
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
                90.0 + ((state >> 11) as f64 / (1u64 << 53) as f64) * 20.0
            })
            .collect()
    }

    #[test]
    fn matches_reference_bitwise_and_survives_chunking() {
        let input = lcg_series(5_000, 0x70_5EED_41);
        for period in [1usize, 2, 5, 14, 200] {
            let mut reference = Reference::new(period);
            let expected: Vec<f64> = input
                .iter()
                .map(|&v| reference.append(v).unwrap_or(f64::NAN))
                .collect();
            let mut state = UlcerIndex::new(period).unwrap();
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
        let input = lcg_series(1_000, 0x71_5EED_42);
        let batch = ulcer_index(&input, 14).unwrap();
        let mut state = UlcerIndex::new(14).unwrap();
        for (i, value) in batch.iter().enumerate() {
            let got = state.append(input[i]).unwrap_or(f64::NAN);
            assert_eq!(value.to_bits(), got.to_bits());
        }
    }
}
