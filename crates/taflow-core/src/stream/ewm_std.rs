//! Batch implementation for `ewm_std`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `ewm_std` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the ewm std result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn ewm_std(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = ExponentiallyWeightedStandardDeviation::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value)).collect())
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `ExponentiallyWeightedStandardDeviation`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct ExponentiallyWeightedStandardDeviation {
    variance: ExponentiallyWeightedVariance,
    value: Option<f64>,
}

impl ExponentiallyWeightedStandardDeviation {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            variance: ExponentiallyWeightedVariance::new(timeperiod)?,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, input: f64) -> f64 {
        let value = self.variance.append(input).sqrt();
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
        self.variance.reset();
        self.value = None;
    }
}
