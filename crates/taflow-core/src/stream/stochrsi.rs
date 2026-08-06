//! Stateful Stochastic Relative Strength Index.
//!
//! STOCHRSI pipelines each warmed Wilder RSI value through the persistent
//! Fast Stochastic state, preserving adaptive-MA seed and rounding semantics.

use crate::error::TaResult;
use crate::ma_type::MaType;

use super::{Rsi, Stochf, StreamingIndicator};

/// One aligned stochastic-RSI fast %K and fast %D observation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StochrsiValue {
    pub fastk: f64,
    pub fastd: f64,
}

/// Incremental STOCHRSI state.
pub struct Stochrsi {
    rsi: Rsi,
    stochastic: Stochf,
    value: Option<StochrsiValue>,
}

impl Stochrsi {
    /// Creates STOCHRSI with a selectable fast-%D moving-average type.
    pub fn new(
        timeperiod: usize,
        fastk_period: usize,
        fastd_period: usize,
        fastd_matype: MaType,
    ) -> TaResult<Self> {
        Ok(Self {
            rsi: Rsi::new(timeperiod)?,
            stochastic: Stochf::new(fastk_period, fastd_period, fastd_matype)?,
            value: None,
        })
    }

    /// Appends one close value.
    pub fn append(&mut self, input: f64) -> Option<StochrsiValue> {
        self.value = self.rsi.append(input).and_then(|rsi| {
            self.stochastic
                .append(rsi, rsi, rsi)
                .map(|value| StochrsiValue {
                    fastk: value.fastk,
                    fastd: value.fastd,
                })
        });
        self.value
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<StochrsiValue> {
        self.value
    }

    /// Restores the post-construction state.
    pub fn reset(&mut self) {
        self.rsi.reset();
        self.stochastic.reset();
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::momentum;

    #[test]
    fn matches_batch_for_all_moving_average_types() {
        let input: Vec<f64> = (0..500)
            .map(|index| 100.0 + (index as f64 * 0.17).sin() * 8.0 + index as f64 * 0.01)
            .collect();
        for code in 0..=8 {
            let ma_type = MaType::try_from(code).unwrap();
            let expected = momentum::stochrsi(&input, 14, 5, 13, ma_type).unwrap();
            let mut state = Stochrsi::new(14, 5, 13, ma_type).unwrap();
            for (index, input) in input.iter().copied().enumerate() {
                match state.append(input) {
                    Some(actual) => {
                        assert!((actual.fastk - expected.0[index]).abs() < 1e-8);
                        assert!((actual.fastd - expected.1[index]).abs() < 1e-8);
                    }
                    None => assert!(expected.0[index].is_nan()),
                }
            }
            let final_value = state.value();
            state.reset();
            for input in input.iter().copied() {
                state.append(input);
            }
            assert_eq!(state.value(), final_value);
        }
    }
}
