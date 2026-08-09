//! Stateful exponentially weighted correlation.

use super::exponentially_weighted_covariance::ExponentiallyWeightedCovariance;
use crate::error::TaResult;

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `ExponentiallyWeightedCorrelation`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct ExponentiallyWeightedCorrelation {
    covariance: ExponentiallyWeightedCovariance,
    value: Option<f64>,
}

impl ExponentiallyWeightedCorrelation {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            covariance: ExponentiallyWeightedCovariance::new(timeperiod)?,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, left: f64, right: f64) -> f64 {
        self.covariance.append(left, right);
        let denominator = (self.covariance.var0 * self.covariance.var1).sqrt();
        let value = if denominator > 0.0 {
            self.covariance.covariance / denominator
        } else {
            0.0
        };
        self.value = Some(value);
        value
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
        self.covariance.reset();
        self.value = None;
    }
}
