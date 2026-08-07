//! Stateful moving average with variable period.
//!
//! A newly requested period is initialized by replaying retained input once;
//! initialized periods then advance incrementally. This preserves recursive
//! MA history without allocating every possible period state up front.

use std::collections::HashMap;

use crate::error::{TaError, TaResult};
use crate::ma_type::MaType;

use super::moving_average::MovingAverageDispatcher;

/// Computes an aligned variable-period moving-average vector.
pub fn moving_average_variable_period(
    input: &[f64],
    periods: &[f64],
    minperiod: usize,
    maxperiod: usize,
    matype: MaType,
) -> TaResult<Vec<f64>> {
    if input.len() != periods.len() {
        return Err(crate::TaError::LengthMismatch { expected: input.len(), got: periods.len() });
    }
    let mut state = VariablePeriodMovingAverage::new(minperiod, maxperiod, matype)?;
    Ok(input.iter().zip(periods).map(|(&value, &period)| state.append(value, period).unwrap_or(f64::NAN)).collect())
}

/// Incremental MAVP with TA-Lib-compatible truncation and clamping.
pub struct VariablePeriodMovingAverage {
    minperiod: usize,
    maxperiod: usize,
    matype: MaType,
    lookback: usize,
    history: Vec<f64>,
    states: HashMap<usize, MovingAverageDispatcher>,
    value: Option<f64>,
}

impl VariablePeriodMovingAverage {
    /// Creates a variable-period moving-average state.
    pub fn new(minperiod: usize, maxperiod: usize, matype: MaType) -> TaResult<Self> {
        if minperiod == 0 || maxperiod < minperiod {
            return Err(TaError::InvalidParameter {
                name: "minperiod/maxperiod",
                value: format!("{minperiod}/{maxperiod}"),
                reason: "minperiod >= 1 and maxperiod >= minperiod required",
            });
        }
        Ok(Self {
            minperiod,
            maxperiod,
            matype,
            lookback: matype.lookback(maxperiod),
            history: Vec::new(),
            states: HashMap::new(),
            value: None,
        })
    }

    /// Appends one value and its requested moving-average period.
    pub fn append(&mut self, input: f64, period: f64) -> Option<f64> {
        self.history.push(input);
        let selected = (period as usize).clamp(self.minperiod, self.maxperiod);

        let mut selected_value = None;
        for (state_period, state) in &mut self.states {
            let current = state.append(input);
            if *state_period == selected {
                selected_value = current;
            }
        }

        if !self.states.contains_key(&selected) {
            let source_start = self.lookback - self.matype.lookback(selected);
            if self.history.len() > source_start {
                let mut state = MovingAverageDispatcher::new(selected, self.matype)
                    .expect("MAVP constructor validates the complete period range");
                selected_value = self.history[source_start..]
                    .iter()
                    .copied()
                    .fold(None, |_, value| state.append(value));
                self.states.insert(selected, state);
            }
        }

        self.value = if self.history.len() > self.lookback {
            selected_value
        } else {
            None
        };
        self.value
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restores the post-construction state.
    pub fn reset(&mut self) {
        self.history.clear();
        self.states.clear();
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::overlap;

    #[test]
    fn matches_batch_for_every_moving_average_type() {
        let input: Vec<f64> = (0..700)
            .map(|index| 100.0 + (index as f64 * 0.17).sin() * 8.0 + index as f64 * 0.01)
            .collect();
        let requested = [1.9, 3.8, 7.2, 11.9, 50.0];
        let periods: Vec<f64> = (0..input.len())
            .map(|index| requested[index % requested.len()])
            .collect();
        for code in 0..=8 {
            let ma_type = MaType::try_from(code).unwrap();
            let expected = overlap::moving_average_variable_period(&input, &periods, 2, 12, ma_type).unwrap();
            let mut state = VariablePeriodMovingAverage::new(2, 12, ma_type).unwrap();
            for index in 0..input.len() {
                match state.append(input[index], periods[index]) {
                    Some(actual) => assert!((actual - expected[index]).abs() < 1e-8),
                    None => assert!(expected[index].is_nan()),
                }
            }
            state.reset();
            assert!(state.value().is_none());
        }
    }
}
