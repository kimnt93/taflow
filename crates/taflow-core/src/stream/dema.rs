//! Incremental Double Exponential Moving Average (DEMA).

use crate::error::TaResult;

use super::{invalid_period, ExponentialMovingAverage, StreamingIndicator};

/// Computes an aligned Double Exponential Moving Average vector.
pub fn double_exponential_moving_average(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = DoubleExponentialMovingAverage::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value).unwrap_or(f64::NAN)).collect())
}

/// Stateful double EMA composed from the shared EMA primitive.
#[derive(Debug, Clone)]
pub struct DoubleExponentialMovingAverage {
    ema1: ExponentialMovingAverage,
    ema2: ExponentialMovingAverage,
    value: Option<f64>,
}

impl DoubleExponentialMovingAverage {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            ema1: ExponentialMovingAverage::new(period)?,
            ema2: ExponentialMovingAverage::new(period)?,
            value: None,
        })
    }
}

impl StreamingIndicator for DoubleExponentialMovingAverage {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self
            .ema1
            .append(input)
            .and_then(|ema1| self.ema2.append(ema1).map(|ema2| 2.0 * ema1 - ema2));
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.ema1.reset();
        self.ema2.reset();
        self.value = None;
    }
}
