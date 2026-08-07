//! Incremental Minus Directional Indicator (-DI).
use super::directional::DirectionalMovement;
use crate::error::TaResult;
pub struct MinusDirectionalIndicator {
    directional: DirectionalMovement,
    value: Option<f64>,
}
impl MinusDirectionalIndicator {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            directional: DirectionalMovement::new(period)?,
            value: None,
        })
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        self.value = self
            .directional
            .append(high, low, close)
            .map(|v| v.minus_di);
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
        self.directional.reset();
        self.value = None;
    }
}
