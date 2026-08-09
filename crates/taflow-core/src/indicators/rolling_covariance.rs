//! Stateful implementation for `rolling_covariance`.

use crate::error::{TaError, TaResult};
use crate::stream::validate_period;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
/// Persistent Rust state for `RollingCovariance`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingCovariance {
    values: VecDeque<(f64, f64)>,
    timeperiod: usize,
    value: Option<f64>,
}

impl RollingCovariance {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self {
            values: VecDeque::with_capacity(timeperiod),
            timeperiod,
            value: None,
        })
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, left: f64, right: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod {
            self.values.pop_front();
        }
        self.values.push_back((left, right));
        self.value = if self.values.len() == self.timeperiod {
            let n = self.timeperiod as f64;
            let left_mean = self.values.iter().map(|&(left, _)| left).sum::<f64>() / n;
            let right_mean = self.values.iter().map(|&(_, right)| right).sum::<f64>() / n;
            Some(
                self.values
                    .iter()
                    .map(|&(left, right)| (left - left_mean) * (right - right_mean))
                    .sum::<f64>()
                    / n,
            )
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
        self.values.clear();
        self.value = None;
    }
}
