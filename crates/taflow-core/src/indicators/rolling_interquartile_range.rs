//! Batch implementation for `rolling_iqr`.

use super::RollingQuantile;
use crate::error::{TaError, TaResult};

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingInterquartileRange`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingInterquartileRange {
    quantile: RollingQuantile,
    value: Option<f64>,
}

impl RollingInterquartileRange {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            quantile: RollingQuantile::new(timeperiod, 0.25)?,
            value: None,
        })
    }
    /// Append one value and return the current interquartile range.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.quantile.append(input);
        self.value = if self.quantile.window.is_full() {
            let sorted = self.quantile.window.sorted();
            let quantile = |q: f64| {
                let position = q * (sorted.len() - 1) as f64;
                let lower = position.floor() as usize;
                let upper = position.ceil() as usize;
                sorted[lower] + (sorted[upper] - sorted[lower]) * (position - lower as f64)
            };
            Some(quantile(0.75) - quantile(0.25))
        } else {
            None
        };
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
        self.quantile.reset();
        self.value = None;
    }
}
