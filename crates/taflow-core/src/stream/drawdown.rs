//! Batch implementation for `drawdown`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `drawdown` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the drawdown result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn drawdown(input: &[f64]) -> Vec<f64> {
    let mut maximum = f64::NEG_INFINITY;
    input
        .iter()
        .map(|&value| {
            maximum = maximum.max(value);
            if maximum != 0.0 {
                value / maximum - 1.0
            } else {
                0.0
            }
        })
        .collect()
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Drawdown`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Drawdown {
    maximum: CumulativeMaximum,
    value: Option<f64>,
}

impl Drawdown {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            maximum: CumulativeMaximum::default(),
            value: None,
        }
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, input: f64) -> f64 {
        let maximum = self.maximum.append(input);
        let value = if maximum != 0.0 {
            input / maximum - 1.0
        } else {
            0.0
        };
        self.value = Some(value);
        value
    }
    /// Return the latest computed result, if warm-up is complete.
    ///
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.maximum.reset();
        self.value = None;
    }
}

impl Default for Drawdown {
    fn default() -> Self {
        Self::new()
    }
}
