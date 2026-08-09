use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `McGinleyDynamic`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct McGinleyDynamic {
    length: usize,
    c: f64,
    value: Option<f64>,
}

impl McGinleyDynamic {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(length: usize, c: f64) -> TaResult<Self> {
        validate_period(length)?;
        if !(0.0 < c && c <= 1.0) {
            return Err(TaError::InvalidParameter {
                name: "c",
                value: c.to_string(),
                reason: "must be in (0, 1]",
            });
        }
        Ok(Self {
            length,
            c,
            value: None,
        })
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, close: f64) -> Option<f64> {
        self.value = Some(match self.value {
            None => close,
            Some(previous) if previous != 0.0 => {
                let mut denominator = self.c * self.length as f64 * (close / previous).powi(4);
                if denominator < 1e-10 {
                    denominator = 1e-10;
                }
                previous + (close - previous) / denominator
            }
            Some(_) => close,
        });
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
        self.value = None;
    }
}
