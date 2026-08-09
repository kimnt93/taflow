//! Batch implementation for `rolling_rank`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `rolling_rank` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the rolling rank result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_rank(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = RollingRank::new(timeperiod)?;
    Ok(input
        .iter()
        .map(|&value| state.append(value).unwrap_or(f64::NAN))
        .collect())
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingRank`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingRank {
    values: VecDeque<f64>,
    timeperiod: usize,
    value: Option<f64>,
}

impl RollingRank {
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
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod {
            self.values.pop_front();
        }
        self.values.push_back(input);
        self.value = if self.values.len() == self.timeperiod {
            let less = self.values.iter().filter(|&&value| value < input).count();
            let equal = self.values.iter().filter(|&&value| value == input).count();
            Some((less as f64 + equal as f64) / self.timeperiod as f64)
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
