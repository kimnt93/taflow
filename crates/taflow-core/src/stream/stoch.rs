//! Stateful Stochastic Oscillator.
//!
//! STOCH maintains rolling high/low extrema for fast %K, then feeds each
//! warmed value through independently selectable slow-%K and slow-%D moving
//! averages.

use crate::error::TaResult;
use crate::ma_type::MaType;

use super::{moving_average::MovingAverage, RollingMax, RollingMin, StreamingIndicator};

/// One aligned slow %K and slow %D observation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StochValue {
    pub slowk: f64,
    pub slowd: f64,
}

/// Incremental STOCH with amortized constant work per bar.
pub struct Stoch {
    highest: RollingMax,
    lowest: RollingMin,
    slowk: MovingAverage,
    slowd: MovingAverage,
    value: Option<StochValue>,
}

impl Stoch {
    /// Creates a STOCH state for the selected smoothing types.
    pub fn new(
        fastk_period: usize,
        slowk_period: usize,
        slowk_matype: MaType,
        slowd_period: usize,
        slowd_matype: MaType,
    ) -> TaResult<Self> {
        Ok(Self {
            highest: RollingMax::new(fastk_period)?,
            lowest: RollingMin::new(fastk_period)?,
            slowk: MovingAverage::new(slowk_period, slowk_matype)?,
            slowd: MovingAverage::new(slowd_period, slowd_matype)?,
            value: None,
        })
    }

    /// Appends one high, low, and close bar.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<StochValue> {
        let fastk =
            self.highest
                .append(high)
                .zip(self.lowest.append(low))
                .map(|(highest, lowest)| {
                    let divisor = (highest - lowest) / 100.0;
                    if divisor.abs() >= 1.0e-14 {
                        (close - lowest) / divisor
                    } else {
                        0.0
                    }
                });
        self.value = fastk
            .and_then(|fastk| self.slowk.append(fastk))
            .and_then(|slowk| {
                self.slowd
                    .append(slowk)
                    .map(|slowd| StochValue { slowk, slowd })
            });
        self.value
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<StochValue> {
        self.value
    }

    /// Restores the post-construction state while retaining allocated buffers.
    pub fn reset(&mut self) {
        self.highest.reset();
        self.lowest.reset();
        self.slowk.reset();
        self.slowd.reset();
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::momentum;

    #[test]
    fn matches_batch_for_all_moving_average_pairs() {
        let close: Vec<f64> = (0..500)
            .map(|index| 100.0 + (index as f64 * 0.17).sin() * 8.0 + index as f64 * 0.01)
            .collect();
        let high: Vec<f64> = close
            .iter()
            .enumerate()
            .map(|(index, close)| close + 1.0 + (index as f64 * 0.11).sin().abs())
            .collect();
        let low: Vec<f64> = close
            .iter()
            .enumerate()
            .map(|(index, close)| close - 1.0 - (index as f64 * 0.13).cos().abs())
            .collect();
        for slowk_code in 0..=8 {
            for slowd_code in 0..=8 {
                let slowk_type = MaType::try_from(slowk_code).unwrap();
                let slowd_type = MaType::try_from(slowd_code).unwrap();
                let expected =
                    momentum::stochastic_oscillator(&high, &low, &close, 5, 13, slowk_type, 11, slowd_type)
                        .unwrap();
                let mut state = Stoch::new(5, 13, slowk_type, 11, slowd_type).unwrap();
                for index in 0..close.len() {
                    match state.append(high[index], low[index], close[index]) {
                        Some(actual) => {
                            assert!((actual.slowk - expected.0[index]).abs() < 1e-8);
                            assert!((actual.slowd - expected.1[index]).abs() < 1e-8);
                        }
                        None => assert!(expected.0[index].is_nan()),
                    }
                }
                let final_value = state.value();
                state.reset();
                for index in 0..close.len() {
                    state.append(high[index], low[index], close[index]);
                }
                assert_eq!(state.value(), final_value);
            }
        }
    }
}
