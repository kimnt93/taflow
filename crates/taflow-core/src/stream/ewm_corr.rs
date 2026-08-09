//! Batch implementation for `ewm_corr`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `ewm_corr` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the ewm corr result for the supplied aligned series.
///
/// # Parameters
///
/// * `input0` - Input series or configuration value.
/// * `input1` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn ewm_corr(input0: &[f64], input1: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if input0.len() != input1.len() {
        return Err(TaError::LengthMismatch {
            expected: input0.len(),
            got: input1.len(),
        });
    }
    let mut state = ExponentiallyWeightedCorrelation::new(timeperiod)?;
    Ok(input0
        .iter()
        .zip(input1)
        .map(|(&left, &right)| state.append(left, right))
        .collect())
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

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
