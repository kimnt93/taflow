//! Stateful Average Directional Movement Index Rating.

use std::collections::VecDeque;

use crate::error::TaResult;

use super::AverageDirectionalIndex;

/// Incremental ADXR using the current and `period - 1` lagged ADX values.
/// Persistent Rust state or aligned output type for `AverageDirectionalIndexRating`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct AverageDirectionalIndexRating {
    period: usize,
    adx: AverageDirectionalIndex,
    values: VecDeque<f64>,
    value: Option<f64>,
}

impl AverageDirectionalIndexRating {
    /// Creates an ADXR state with a period of at least two bars.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            period,
            adx: AverageDirectionalIndex::new(period)?,
            values: VecDeque::with_capacity(period),
            value: None,
        })
    }

    /// Appends one high, low, and close observation.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let current = self.adx.append(high, low, close)?;
        self.values.push_back(current);
        self.value = if self.values.len() == self.period {
            let lagged = self.values.pop_front().expect("full ADXR lag window");
            Some((current + lagged) / 2.0)
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
        self.adx.reset();
        self.values.clear();
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_batch_and_reset_replay() {
        let close: Vec<f64> = (0..700)
            .map(|index| 100.0 + (index as f64 * 0.17).sin() * 8.0 + index as f64 * 0.01)
            .collect();
        let high: Vec<f64> = close.iter().map(|value| value + 1.3).collect();
        let low: Vec<f64> = close.iter().map(|value| value - 1.1).collect();
        for period in [2, 3, 14, 30] {
            let expected =
                crate::stream::average_directional_index_rating(&high, &low, &close, period)
                    .unwrap();
            let mut state = AverageDirectionalIndexRating::new(period).unwrap();
            for index in 0..close.len() {
                match state.append(high[index], low[index], close[index]) {
                    Some(actual) => assert!((actual - expected[index]).abs() < 1e-12),
                    None => assert!(expected[index].is_nan()),
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
// Batch Average Directional Movement Index Rating.

/// Compute the average directional index rating result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `close` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn average_directional_index_rating(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    timeperiod: usize,
) -> TaResult<Vec<f64>> {
    let adx_values = crate::stream::average_directional_index(high, low, close, timeperiod)?;
    let len = adx_values.len();
    let lookback = 3 * timeperiod - 2;
    let mut output = vec![f64::NAN; len];
    for index in lookback..len {
        output[index] = (adx_values[index] + adx_values[index - timeperiod + 1]) / 2.0;
    }
    Ok(output)
}
