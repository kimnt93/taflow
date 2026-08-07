//! Stateful Fast Stochastic Oscillator.
//!
//! STOCHF maintains rolling high/low extrema for fast %K and feeds that value
//! into the selected incremental moving average for fast %D.

use crate::error::TaResult;
use crate::ma_type::MaType;

use super::{moving_average::MovingAverageDispatcher, RollingMax, RollingMin, StreamingIndicator};

/// One aligned fast %K and fast %D observation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FastStochasticOscillatorValue {
    pub fastk: f64,
    pub fastd: f64,
}

/// Incremental STOCHF with amortized constant work per bar.
pub struct FastStochasticOscillator {
    highest: RollingMax,
    lowest: RollingMin,
    fastd: MovingAverageDispatcher,
    value: Option<FastStochasticOscillatorValue>,
}

impl FastStochasticOscillator {
    /// Creates a STOCHF state for the selected fast %D moving-average type.
    pub fn new(fastk_period: usize, fastd_period: usize, fastd_matype: MaType) -> TaResult<Self> {
        Ok(Self {
            highest: RollingMax::new(fastk_period)?,
            lowest: RollingMin::new(fastk_period)?,
            fastd: MovingAverageDispatcher::new(fastd_period, fastd_matype)?,
            value: None,
        })
    }

    /// Appends one high, low, and close bar.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<FastStochasticOscillatorValue> {
        let highest = self.highest.append(high);
        let lowest = self.lowest.append(low);
        let fastk = highest.zip(lowest).map(|(highest, lowest)| {
            let divisor = (highest - lowest) / 100.0;
            if divisor.abs() >= 1.0e-14 {
                (close - lowest) / divisor
            } else {
                0.0
            }
        });
        self.value = fastk.and_then(|fastk| {
            self.fastd
                .append(fastk)
                .map(|fastd| FastStochasticOscillatorValue { fastk, fastd })
        });
        self.value
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<FastStochasticOscillatorValue> {
        self.value
    }

    /// Restores the post-construction state while retaining allocated buffers.
    pub fn reset(&mut self) {
        self.highest.reset();
        self.lowest.reset();
        self.fastd.reset();
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::momentum;

    #[test]
    fn matches_batch_for_all_moving_average_types() {
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
        for code in 0..=8 {
            let ma_type = MaType::try_from(code).unwrap();
            let expected = momentum::fast_stochastic_oscillator(&high, &low, &close, 5, 13, ma_type).unwrap();
            let mut state = FastStochasticOscillator::new(5, 13, ma_type).unwrap();
            for index in 0..close.len() {
                match state.append(high[index], low[index], close[index]) {
                    Some(actual) => {
                        assert!(
                            (actual.fastk - expected.0[index]).abs() < 1e-8,
                            "type {code}"
                        );
                        assert!(
                            (actual.fastd - expected.1[index]).abs() < 1e-8,
                            "type {code}"
                        );
                    }
                    None => assert!(expected.0[index].is_nan(), "type {code}"),
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
