//! Stateful Acceleration Bands.
//!
//! ACCBANDS applies TA-Lib's high/low acceleration transform and advances
//! three aligned simple moving averages for upper, middle, and lower bands.

use crate::error::TaResult;

use super::{invalid_period, Sma, StreamingIndicator};

/// One aligned upper, middle, and lower Acceleration Bands observation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AccbandsValue {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}

/// Incremental Acceleration Bands with constant per-bar work.
#[derive(Debug, Clone)]
pub struct Accbands {
    upper: Sma,
    middle: Sma,
    lower: Sma,
    value: Option<AccbandsValue>,
}

impl Accbands {
    /// Creates an ACCBANDS state for a period of at least two bars.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            upper: Sma::new(period)?,
            middle: Sma::new(period)?,
            lower: Sma::new(period)?,
            value: None,
        })
    }

    /// Appends one high, low, and close bar.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<AccbandsValue> {
        let denominator = high + low;
        let (upper_input, lower_input) = if denominator == 0.0 {
            (high, low)
        } else {
            let adjustment = 4.0 * (high - low) / denominator;
            (high * (1.0 + adjustment), low * (1.0 - adjustment))
        };
        let upper = self.upper.append(upper_input);
        let middle = self.middle.append(close);
        let lower = self.lower.append(lower_input);
        self.value = upper
            .zip(middle)
            .zip(lower)
            .map(|((upper, middle), lower)| AccbandsValue {
                upper,
                middle,
                lower,
            });
        self.value
    }

    pub fn value(&self) -> Option<AccbandsValue> {
        self.value
    }

    pub fn reset(&mut self) {
        self.upper.reset();
        self.middle.reset();
        self.lower.reset();
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::overlap;

    #[test]
    fn matches_batch_and_reset_replay() {
        let close: Vec<f64> = (0..200)
            .map(|index| 100.0 + (index as f64 * 0.31).sin() * 8.0 + index as f64 * 0.02)
            .collect();
        let high: Vec<f64> = close
            .iter()
            .enumerate()
            .map(|(index, close)| close + 1.0 + (index as f64 * 0.17).sin().abs())
            .collect();
        let low: Vec<f64> = close
            .iter()
            .enumerate()
            .map(|(index, close)| close - 1.0 - (index as f64 * 0.13).cos().abs())
            .collect();
        let (upper, middle, lower) = overlap::acceleration_bands(&high, &low, &close, 13).unwrap();
        let mut state = Accbands::new(13).unwrap();
        for index in 0..close.len() {
            match state.append(high[index], low[index], close[index]) {
                Some(actual) => {
                    assert!((actual.upper - upper[index]).abs() < 1e-10);
                    assert!((actual.middle - middle[index]).abs() < 1e-10);
                    assert!((actual.lower - lower[index]).abs() < 1e-10);
                }
                None => {
                    assert!(upper[index].is_nan());
                    assert!(middle[index].is_nan());
                    assert!(lower[index].is_nan());
                }
            }
        }
        let expected_final = state.value();
        state.reset();
        for index in 0..close.len() {
            state.append(high[index], low[index], close[index]);
        }
        assert_eq!(state.value(), expected_final);
    }
}
