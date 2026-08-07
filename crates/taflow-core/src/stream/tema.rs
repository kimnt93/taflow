//! Incremental Triple Exponential Moving Average (TEMA).

use crate::error::TaResult;

use super::{invalid_period, ExponentialMovingAverage, StreamingIndicator};

/// Compute the triple exponential moving average result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn triple_exponential_moving_average(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = TripleExponentialMovingAverage::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value).unwrap_or(f64::NAN)).collect())
}

/// Stateful triple EMA composed from the shared EMA primitive.
#[derive(Debug, Clone)]
pub struct TripleExponentialMovingAverage {
    ema1: ExponentialMovingAverage,
    ema2: ExponentialMovingAverage,
    ema3: ExponentialMovingAverage,
    value: Option<f64>,
}

impl TripleExponentialMovingAverage {
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
            ema3: ExponentialMovingAverage::new(period)?,
            value: None,
        })
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
