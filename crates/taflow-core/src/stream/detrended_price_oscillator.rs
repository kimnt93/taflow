use super::operator_states::*;
use super::StreamingIndicator;
use super::{SimpleMovingAverage, Window};
use crate::error::TaResult;

pub struct DetrendedPriceOscillator {
    sma: SimpleMovingAverage,
    delay: Window,
    value: Option<f64>,
}

impl DetrendedPriceOscillator {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            sma: SimpleMovingAverage::new(period)?,
            delay: Window::new(period / 2 + 1)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, close: f64) -> Option<f64> {
        self.value = self
            .sma
            .append(close)
            .and_then(|mean| self.delay.push(mean).map(|delayed| close - delayed));
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
        self.sma.reset();
        self.delay.clear();
        self.value = None;
    }
}
