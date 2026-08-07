//! Stateful Stochastic Relative Strength Index.
//!
//! STOCHRSI pipelines each warmed Wilder RSI value through the persistent
//! Fast Stochastic state, preserving adaptive-MA seed and rounding semantics.

use crate::error::TaResult;
use crate::ma_type::MaType;

use super::{RelativeStrengthIndex, FastStochasticOscillator, StreamingIndicator};

/// One aligned stochastic-RSI fast %K and fast %D observation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StochasticRelativeStrengthIndexValue {
    pub fastk: f64,
    pub fastd: f64,
}

/// Incremental STOCHRSI state.
pub struct StochasticRelativeStrengthIndex {
    rsi: RelativeStrengthIndex,
    stochastic: FastStochasticOscillator,
    value: Option<StochasticRelativeStrengthIndexValue>,
}

impl StochasticRelativeStrengthIndex {
    /// Creates STOCHRSI with a selectable fast-%D moving-average type.
    pub fn new(
        timeperiod: usize,
        fastk_period: usize,
        fastd_period: usize,
        fastd_matype: MaType,
    ) -> TaResult<Self> {
        Ok(Self {
            rsi: RelativeStrengthIndex::new(timeperiod)?,
            stochastic: FastStochasticOscillator::new(fastk_period, fastd_period, fastd_matype)?,
            value: None,
        })
    }

    /// Appends one close value.
    pub fn append(&mut self, input: f64) -> Option<StochasticRelativeStrengthIndexValue> {
        self.value = self.rsi.append(input).and_then(|rsi| {
            self.stochastic
                .append(rsi, rsi, rsi)
                .map(|value| StochasticRelativeStrengthIndexValue {
                    fastk: value.fastk,
                    fastd: value.fastd,
                })
        });
        self.value
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<StochasticRelativeStrengthIndexValue> {
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


    #[test]
    fn matches_batch_for_all_moving_average_types() {
        let input: Vec<f64> = (0..500)
            .map(|index| 100.0 + (index as f64 * 0.17).sin() * 8.0 + index as f64 * 0.01)
            .collect();
        for code in 0..=8 {
            let ma_type = MaType::try_from(code).unwrap();
            let expected = crate::stream::stochastic_relative_strength_index(&input, 14, 5, 13, ma_type).unwrap();
            let mut state = StochasticRelativeStrengthIndex::new(14, 5, 13, ma_type).unwrap();
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
// Batch Stochastic Relative Strength Index.
//
// STOCHRSI applies a rolling stochastic range to RSI values, then smooths
// fast %K with a selectable TA-Lib moving average to produce fast %D.

/// Computes aligned stochastic-RSI fast %K and fast %D arrays.
///
/// # Parameters
///
/// * `input` - Chronological close-price series.
/// * Period and moving-average parameters configure RSI and stochastic windows.
///
/// # Returns
///
/// A pair of same-length fast %K and fast %D arrays with warm-up NaNs.
pub fn stochastic_relative_strength_index(
    input: &[f64],
    timeperiod: usize,
    fastk_period: usize,
    fastd_period: usize,
    fastd_matype: MaType,
) -> TaResult<(Vec<f64>, Vec<f64>)> {
    let rsi_values = crate::stream::relative_strength_index(input, timeperiod)?;
    let rsi_valid = &rsi_values[timeperiod..];
    let (stochastic_k, stochastic_d) = crate::stream::fast_stochastic_oscillator(
        rsi_valid,
        rsi_valid,
        rsi_valid,
        fastk_period,
        fastd_period,
        fastd_matype,
    )?;
    let len = input.len();
    let mut fastk_out = vec![f64::NAN; len];
    let mut fastd_out = vec![f64::NAN; len];
    fastk_out[timeperiod..].copy_from_slice(&stochastic_k);
    fastd_out[timeperiod..].copy_from_slice(&stochastic_d);
    Ok((fastk_out, fastd_out))
}
