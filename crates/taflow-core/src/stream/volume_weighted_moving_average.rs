//! Batch implementation for `volume_weighted_moving_average`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes the causal volume weighted moving average series.
/// Parameters: aligned input slices followed by indicator parameters.
/// Compute the volume weighted moving average result for the supplied aligned series.
///
/// # Parameters
///
/// * `price` - Input series or configuration value.
/// * `volume` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn volume_weighted_moving_average(
    price: &[f64],
    volume: &[f64],
    timeperiod: usize,
) -> TaResult<Vec<f64>> {
    if price.len() != volume.len() {
        return Err(TaError::LengthMismatch {
            expected: price.len(),
            got: volume.len(),
        });
    }
    let mut state = VolumeWeightedMovingAverage::new(timeperiod)?;
    Ok(price
        .iter()
        .zip(volume)
        .map(|(&p, &v)| state.append(p, v).unwrap_or(f64::NAN))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// Pre-optimization `VolumeWeightedMovingAverage::append` oracle.
    struct Reference {
        prices: VecDeque<f64>,
        volumes: VecDeque<f64>,
        period: usize,
    }

    impl Reference {
        fn new(period: usize) -> Self {
            Self {
                prices: VecDeque::with_capacity(period),
                volumes: VecDeque::with_capacity(period),
                period,
            }
        }

        fn append(&mut self, price: f64, volume: f64) -> Option<f64> {
            if self.prices.len() == self.period {
                self.prices.pop_front();
                self.volumes.pop_front();
            }
            self.prices.push_back(price);
            self.volumes.push_back(volume);
            (self.prices.len() == self.period).then(|| {
                let volume = self.volumes.iter().sum::<f64>();
                if volume != 0.0 {
                    self.prices
                        .iter()
                        .zip(&self.volumes)
                        .map(|(&p, &v)| p * v)
                        .sum::<f64>()
                        / volume
                } else {
                    0.0
                }
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
                ((state >> 11) as f64 / (1u64 << 53) as f64) * 100.0
            })
            .collect()
    }

    #[test]
    fn matches_reference_bitwise_and_survives_chunking() {
        let price = lcg_series(5_000, 0x50_5EED_21);
        let mut volume = lcg_series(5_000, 0x51_5EED_22);
        // Exercise the zero-volume branch too.
        for slot in volume.iter_mut().step_by(97) {
            *slot = 0.0;
        }
        for period in [1usize, 2, 5, 20, 200] {
            let mut reference = Reference::new(period);
            let expected: Vec<f64> = (0..price.len())
                .map(|i| reference.append(price[i], volume[i]).unwrap_or(f64::NAN))
                .collect();
            let mut state = VolumeWeightedMovingAverage::new(period).unwrap();
            for (i, want) in expected.iter().enumerate() {
                let got = state.append(price[i], volume[i]).unwrap_or(f64::NAN);
                assert_eq!(want.to_bits(), got.to_bits(), "p={period} bar {i}");
            }
            state.reset();
            let mut fresh = Reference::new(period);
            for i in 0..512 {
                let want = fresh.append(price[i], volume[i]).unwrap_or(f64::NAN);
                let got = state.append(price[i], volume[i]).unwrap_or(f64::NAN);
                assert_eq!(want.to_bits(), got.to_bits(), "p={period} post-reset {i}");
            }
        }
    }

    #[test]
    fn batch_matches_streaming() {
        let price = lcg_series(1_000, 0x52_5EED_23);
        let volume = lcg_series(1_000, 0x53_5EED_24);
        let batch = volume_weighted_moving_average(&price, &volume, 20).unwrap();
        let mut state = VolumeWeightedMovingAverage::new(20).unwrap();
        for (i, value) in batch.iter().enumerate() {
            let got = state.append(price[i], volume[i]).unwrap_or(f64::NAN);
            assert_eq!(value.to_bits(), got.to_bits());
        }
    }
}
