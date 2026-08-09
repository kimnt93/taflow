//! Stateful exponentially weighted variance.

use super::operator_states::ewm_alpha;
use crate::error::TaResult;

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `ExponentiallyWeightedVariance`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct ExponentiallyWeightedVariance {
    alpha: f64,
    mean: Option<f64>,
    variance: f64,
    value: Option<f64>,
}

impl ExponentiallyWeightedVariance {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            alpha: ewm_alpha(timeperiod)?,
            mean: None,
            variance: 0.0,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, input: f64) -> f64 {
        let variance = match self.mean {
            None => {
                self.mean = Some(input);
                0.0
            }
            Some(previous) => {
                let delta = input - previous;
                self.mean = Some(previous + self.alpha * delta);
                (1.0 - self.alpha) * (self.variance + self.alpha * delta * delta)
            }
        };
        self.variance = variance;
        self.value = Some(variance);
        variance
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
        self.mean = None;
        self.variance = 0.0;
        self.value = None;
    }
}
