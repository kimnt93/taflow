//! Stateful Tillson TripleExponentialAverage moving average.
//!
//! TripleExponentialAverage cascades six TA-Lib-seeded exponential moving averages and combines the
//! final four layers with coefficients derived from the volume factor.

use crate::error::{TaError, TaResult};

use super::{ExponentialMovingAverage, StreamingIndicator};

/// Incremental TripleExponentialAverage with constant work and storage per appended bar.
#[derive(Debug, Clone)]
pub struct TripleExponentialAverage {
    ema1: ExponentialMovingAverage,
    ema2: ExponentialMovingAverage,
    ema3: ExponentialMovingAverage,
    ema4: ExponentialMovingAverage,
    ema5: ExponentialMovingAverage,
    ema6: ExponentialMovingAverage,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    value: Option<f64>,
}

impl TripleExponentialAverage {
    /// Creates a TripleExponentialAverage state with a period of at least two bars.
    pub fn new(period: usize, v_factor: f64) -> TaResult<Self> {
        if period < 2 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: period.to_string(),
                reason: "must be >= 2 for TripleExponentialAverage",
            });
        }
        let v2 = v_factor * v_factor;
        let v3 = v2 * v_factor;
        Ok(Self {
            ema1: ExponentialMovingAverage::new(period)?,
            ema2: ExponentialMovingAverage::new(period)?,
            ema3: ExponentialMovingAverage::new(period)?,
            ema4: ExponentialMovingAverage::new(period)?,
            ema5: ExponentialMovingAverage::new(period)?,
            ema6: ExponentialMovingAverage::new(period)?,
            c1: -v3,
            c2: 3.0 * v2 + 3.0 * v3,
            c3: -6.0 * v2 - 3.0 * v_factor - 3.0 * v3,
            c4: 1.0 + 3.0 * v_factor + v3 + 3.0 * v2,
            value: None,
        })
    }
}

impl StreamingIndicator for TripleExponentialAverage {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        let Some(e1) = self.ema1.append(input) else {
            return None;
        };
        let Some(e2) = self.ema2.append(e1) else {
            return None;
        };
        let Some(e3) = self.ema3.append(e2) else {
            return None;
        };
        let Some(e4) = self.ema4.append(e3) else {
            return None;
        };
        let Some(e5) = self.ema5.append(e4) else {
            return None;
        };
        let Some(e6) = self.ema6.append(e5) else {
            return None;
        };
        self.value = Some(self.c1 * e6 + self.c2 * e5 + self.c3 * e4 + self.c4 * e3);
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.ema1.reset();
        self.ema2.reset();
        self.ema3.reset();
        self.ema4.reset();
        self.ema5.reset();
        self.ema6.reset();
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::overlap;

    #[test]
    fn matches_batch_and_reset_replay() {
        let input: Vec<f64> = (0..200)
            .map(|index| 100.0 + (index as f64 * 0.23).sin() * 9.0 + index as f64 * 0.04)
            .collect();
        let expected = overlap::triple_exponential_average(&input, 7, 0.7).unwrap();
        let mut state = TripleExponentialAverage::new(7, 0.7).unwrap();
        for (&input, expected) in input.iter().zip(expected) {
            let actual = state.append(input);
            if expected.is_nan() {
                assert_eq!(actual, None);
            } else {
                assert!((actual.unwrap() - expected).abs() < 1e-12);
            }
        }
        let final_value = state.value();
        state.reset();
        for input in input {
            state.append(input);
        }
        assert_eq!(state.value(), final_value);
    }
}
