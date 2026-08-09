//! Incremental Triple Exponential Moving Average (TEMA).

use crate::error::TaResult;

use super::{invalid_period, ExponentialMovingAverage, StreamingIndicator};

/// Stateful triple EMA composed from the shared EMA primitive.
#[derive(Debug, Clone)]
pub struct TripleExponentialMovingAverage {
    ema1: ExponentialMovingAverage,
    ema2: ExponentialMovingAverage,
    ema3: ExponentialMovingAverage,
    value: Option<f64>,
}

impl TripleExponentialMovingAverage {
    /// Creates a TEMA state with the requested EMA period.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            ema1: ExponentialMovingAverage::new(period)?,
            ema2: ExponentialMovingAverage::new(period)?,
            ema3: ExponentialMovingAverage::new(period)?,
            value: None,
        })
    }

    /// Extends the state with a borrowed slice and appends aligned output.
    pub fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        output.reserve(inputs.len());
        output.extend(
            inputs
                .iter()
                .copied()
                .map(|input| self.append(input).unwrap_or(f64::NAN)),
        );
    }
}

impl StreamingIndicator for TripleExponentialMovingAverage {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self.ema1.append(input).and_then(|ema1| {
            self.ema2.append(ema1).and_then(|ema2| {
                self.ema3
                    .append(ema2)
                    .map(|ema3| 3.0 * ema1 - 3.0 * ema2 + ema3)
            })
        });
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.ema1.reset();
        self.ema2.reset();
        self.ema3.reset();
        self.value = None;
    }
}
