use crate::error::TaResult;
use crate::stream::operator_states::*;
use crate::stream::operator_states::*;
use crate::stream::*;
use crate::stream::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// Rolling standard deviation of log returns (close-to-close volatility).
/// Warm-up values are `NaN`.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `CloseToCloseSigma`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CloseToCloseSigma {
    mean: RollingMean,
    squares: RollingMean,
    previous_close: Option<f64>,
    value: Option<f64>,
}

impl CloseToCloseSigma {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            mean: RollingMean::new(timeperiod)?,
            squares: RollingMean::new(timeperiod)?,
            previous_close: None,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, close: f64) -> Option<f64> {
        if let Some(previous_close) = self.previous_close.replace(close) {
            if close > 0.0 && previous_close > 0.0 {
                let log_return = (close / previous_close).ln();
                let _ = self.mean.append(log_return);
                let _ = self.squares.append(log_return * log_return);
                self.value = match (self.mean.value(), self.squares.value()) {
                    (Some(mean), Some(squares)) => {
                        Some((squares - mean * mean).max(0.0).sqrt() * 252.0_f64.sqrt() * 100.0)
                    }
                    _ => None,
                };
            }
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
        self.squares.reset();
        self.previous_close = None;
        self.value = None;
    }
}
