use crate::error::TaResult;
use crate::stream::operator_states::*;

pub struct Amihud {
    mean: RollingMean,
    previous_close: Option<f64>,
    value: Option<f64>,
}

impl Amihud {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            mean: RollingMean::new(timeperiod)?,
            previous_close: None,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, close: f64, volume: f64) -> Option<f64> {
        if let Some(previous_close) = self.previous_close.replace(close) {
            let term = if close > 0.0 && previous_close > 0.0 && volume > 0.0 {
                ((close - previous_close) / previous_close).abs() / (close * volume)
            } else {
                0.0
            };
            self.value = self.mean.append(term);
        }
        self.value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.mean.reset();
        self.previous_close = None;
        self.value = None;
    }
}
