//! Batch implementation for `rolling_quantile`.

use super::operator_states::*;
use super::operator_states::*;
use super::*;
use super::*;
use crate::error::{TaError, TaResult};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingQuantile`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingQuantile {
    pub(crate) window: super::sorted_ring::SortedRing,
    timeperiod: usize,
    quantile: f64,
    value: Option<f64>,
}

impl RollingQuantile {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize, quantile: f64) -> TaResult<Self> {
        validate_period(timeperiod)?;
        validate_quantile(quantile)?;
        Ok(Self {
            window: super::sorted_ring::SortedRing::new(timeperiod),
            timeperiod,
            quantile,
            value: None,
        })
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    ///
    /// The window is a shared sorted ring; the interpolation arithmetic is
    /// unchanged from the per-bar full-sort implementation, so outputs stay
    /// bit-identical.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.window.push(input);
        self.value = if self.window.is_full() {
            let sorted = self.window.sorted();
            let position = self.quantile * (self.timeperiod - 1) as f64;
            let lower = position.floor() as usize;
            let upper = position.ceil() as usize;
            Some(sorted[lower] + (sorted[upper] - sorted[lower]) * (position - lower as f64))
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
