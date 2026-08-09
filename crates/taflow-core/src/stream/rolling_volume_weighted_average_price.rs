//! Batch implementation for `rolling_volume_weighted_average_price`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes the causal rolling volume weighted average price series.
/// Parameters: aligned input slices followed by indicator parameters.
/// Compute the rolling volume weighted average price result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `close` - Input series or configuration value.
/// * `volume` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_volume_weighted_average_price(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    volume: &[f64],
    timeperiod: usize,
) -> TaResult<Vec<f64>> {
    if high.len() != low.len() || high.len() != close.len() || high.len() != volume.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len(),
        });
    }
    let mut state = RollingVolumeWeightedAveragePrice::new(timeperiod)?;
    Ok(high
        .iter()
        .zip(low)
        .zip(close)
        .zip(volume)
        .map(|(((&h, &l), &c), &v)| state.append(h, l, c, v).unwrap_or(f64::NAN))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// Pre-optimization `RollingVolumeWeightedAveragePrice::append` oracle.
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

        fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> Option<f64> {
            if self.prices.len() == self.period {
                self.prices.pop_front();
                self.volumes.pop_front();
            }
            self.prices.push_back((high + low + close) / 3.0);
            self.volumes.push_back(volume);
            (self.prices.len() == self.period).then(|| {
                let total = self.volumes.iter().sum::<f64>();
                if total != 0.0 {
                    self.prices
                        .iter()
                        .zip(&self.volumes)
                        .map(|(&p, &v)| p * v)
                        .sum::<f64>()
                        / total
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
        let base = lcg_series(5_000, 0x60_5EED_31);
        let high: Vec<f64> = base.iter().map(|v| v + 0.4).collect();
        let low: Vec<f64> = base.iter().map(|v| v - 0.4).collect();
        let mut volume = lcg_series(5_000, 0x61_5EED_32);
        for slot in volume.iter_mut().step_by(89) {
            *slot = 0.0;
        }
        for period in [1usize, 2, 5, 20, 200] {
            let mut reference = Reference::new(period);
            let expected: Vec<f64> = (0..base.len())
                .map(|i| {
                    reference
                        .append(high[i], low[i], base[i], volume[i])
                        .unwrap_or(f64::NAN)
                })
                .collect();
            let mut state = RollingVolumeWeightedAveragePrice::new(period).unwrap();
            for (i, want) in expected.iter().enumerate() {
                let got = state
                    .append(high[i], low[i], base[i], volume[i])
                    .unwrap_or(f64::NAN);
                assert_eq!(want.to_bits(), got.to_bits(), "p={period} bar {i}");
            }
            state.reset();
            let mut fresh = Reference::new(period);
            for i in 0..512 {
                let want = fresh
                    .append(high[i], low[i], base[i], volume[i])
                    .unwrap_or(f64::NAN);
                let got = state
                    .append(high[i], low[i], base[i], volume[i])
                    .unwrap_or(f64::NAN);
                assert_eq!(want.to_bits(), got.to_bits(), "p={period} post-reset {i}");
            }
        }
    }
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingVolumeWeightedAveragePrice`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingVolumeWeightedAveragePrice {
    prices: ContiguousWindow,
    volumes: ContiguousWindow,
    value: Option<f64>,
}

impl RollingVolumeWeightedAveragePrice {
    /// Create a new empty state.
    ///
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            prices: ContiguousWindow::new(period),
            volumes: ContiguousWindow::new(period),
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    /// Like [`VolumeWeightedMovingAverage`], both window sums are contiguous
    /// per-bar rescans: sliding add/evict sums would reassociate them and
    /// perturb the low bits of the historical output.
    pub fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> Option<f64> {
        self.prices.push((high + low + close) / 3.0);
        self.volumes.push(volume);
        self.value = self.prices.is_full().then(|| {
            let prices = self.prices.window();
            let volumes = self.volumes.window();
            let total = volumes.iter().sum::<f64>();
            if total != 0.0 {
                prices
                    .iter()
                    .zip(volumes)
                    .map(|(&p, &v)| p * v)
                    .sum::<f64>()
                    / total
            } else {
                0.0
            }
        });
        self.value
    }
    /// Return the latest computed result, if warm-up is complete.
    ///
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Reset the state and clear its accumulated history.
    ///
    pub fn reset(&mut self) {
        self.prices.clear();
        self.volumes.clear();
        self.value = None;
    }
}
