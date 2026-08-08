//! Batch implementation for `fisher_transform`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Compute the Fisher transform from aligned high and low prices.
///
/// `timeperiod` controls the trailing normalization window; warm-up output
/// Compute the fisher transform result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn fisher_transform(high: &[f64], low: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if high.len() != low.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len(),
        });
    }
    let mut state = FisherTransform::new(timeperiod)?;
    Ok(high
        .iter()
        .zip(low)
        .map(|(&h, &l)| state.append(h, l).unwrap_or(f64::NAN))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// Pre-optimization `FisherTransform::append` oracle (deque rescans).
    struct Reference {
        period: usize,
        values: VecDeque<f64>,
        previous: f64,
    }

    impl Reference {
        fn new(period: usize) -> Self {
            Self {
                period,
                values: VecDeque::with_capacity(period),
                previous: 0.0,
            }
        }

        fn append(&mut self, high: f64, low: f64) -> Option<f64> {
            if self.values.len() == self.period {
                self.values.pop_front();
            }
            self.values.push_back((high + low) * 0.5);
            if self.values.len() != self.period {
                return None;
            }
            let maximum = self
                .values
                .iter()
                .copied()
                .fold(f64::NEG_INFINITY, f64::max);
            let minimum = self.values.iter().copied().fold(f64::INFINITY, f64::min);
            let normalized = if maximum != minimum {
                2.0 * ((self.values.back().copied().unwrap() - minimum) / (maximum - minimum) - 0.5)
            } else {
                0.0
            };
            let x = (0.66 * normalized + 0.67 * self.previous).clamp(-0.999, 0.999);
            self.previous = x;
            Some(0.5 * ((1.0 + x) / (1.0 - x)).ln())
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
        let base = lcg_series(5_000, 0xB1_5EED_81);
        let high: Vec<f64> = base.iter().map(|v| v + 0.6).collect();
        let low: Vec<f64> = base.iter().map(|v| v - 0.6).collect();
        // A flat stretch exercises the `high == low` branch.
        let mut high = high;
        let mut low = low;
        for i in 1_000..1_050 {
            high[i] = 100.0;
            low[i] = 100.0;
        }
        for period in [1usize, 2, 5, 9, 200] {
            let mut reference = Reference::new(period);
            let expected: Vec<f64> = (0..base.len())
                .map(|i| reference.append(high[i], low[i]).unwrap_or(f64::NAN))
                .collect();
            let mut state = FisherTransform::new(period).unwrap();
            for (i, want) in expected.iter().enumerate() {
                let got = state.append(high[i], low[i]).unwrap_or(f64::NAN);
                assert_eq!(want.to_bits(), got.to_bits(), "p={period} bar {i}");
            }
            state.reset();
            let mut fresh = Reference::new(period);
            for i in 0..512 {
                let want = fresh.append(high[i], low[i]).unwrap_or(f64::NAN);
                let got = state.append(high[i], low[i]).unwrap_or(f64::NAN);
                assert_eq!(want.to_bits(), got.to_bits(), "p={period} post-reset {i}");
            }
        }
    }

    #[test]
    fn batch_matches_streaming() {
        let base = lcg_series(1_000, 0xB2_5EED_82);
        let high: Vec<f64> = base.iter().map(|v| v + 0.6).collect();
        let low: Vec<f64> = base.iter().map(|v| v - 0.6).collect();
        let batch = fisher_transform(&high, &low, 9).unwrap();
        let mut state = FisherTransform::new(9).unwrap();
        for (i, value) in batch.iter().enumerate() {
            let got = state.append(high[i], low[i]).unwrap_or(f64::NAN);
            assert_eq!(value.to_bits(), got.to_bits());
        }
    }
}
