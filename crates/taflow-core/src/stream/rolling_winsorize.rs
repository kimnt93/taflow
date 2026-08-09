//! Batch implementation for `rolling_winsorize`.

use super::operator_states::*;
use super::operator_states::*;
use super::*;
use super::*;
use crate::error::{TaError, TaResult};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingWinsorize`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingWinsorize {
    window: super::sorted_ring::SortedRing,
    timeperiod: usize,
    lower: f64,
    upper: f64,
    value: Option<f64>,
}

impl RollingWinsorize {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize, lower: f64, upper: f64) -> TaResult<Self> {
        validate_period(timeperiod)?;
        validate_quantile(lower)?;
        validate_quantile(upper)?;
        if lower > upper {
            return Err(TaError::InvalidParameter {
                name: "lower/upper",
                value: format!("{lower}/{upper}"),
                reason: "lower must be <= upper",
            });
        }
        Ok(Self {
            window: super::sorted_ring::SortedRing::new(timeperiod),
            timeperiod,
            lower,
            upper,
            value: None,
        })
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    ///
    /// The window is a shared sorted ring; the quantile interpolation and
    /// `max`/`min` clamping are unchanged from the per-bar full-sort
    /// implementation, so outputs stay bit-identical.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.window.push(input);
        self.value = if self.window.is_full() {
            let sorted = self.window.sorted();
            let quantile = |q: f64| {
                let position = q * (sorted.len() - 1) as f64;
                let lower = position.floor() as usize;
                let upper = position.ceil() as usize;
                sorted[lower] + (sorted[upper] - sorted[lower]) * (position - lower as f64)
            };
            Some(input.max(quantile(self.lower)).min(quantile(self.upper)))
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
        self.window.clear();
        self.value = None;
    }
}
