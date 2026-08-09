//! Batch implementation for `ewm_cov`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `ewm_cov` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the ewm cov result for the supplied aligned series.
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
pub fn ewm_cov(input0: &[f64], input1: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if input0.len() != input1.len() {
        return Err(TaError::LengthMismatch {
            expected: input0.len(),
            got: input1.len(),
        });
    }
    let mut state = ExponentiallyWeightedCovariance::new(timeperiod)?;
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
/// Persistent Rust state or aligned output type for `ExponentiallyWeightedCovariance`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct ExponentiallyWeightedCovariance {
    alpha: f64,
    mean0: Option<f64>,
    mean1: Option<f64>,
    pub(crate) var0: f64,
    pub(crate) var1: f64,
    pub(crate) covariance: f64,
    value: Option<f64>,
}

impl ExponentiallyWeightedCovariance {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            alpha: ewm_alpha(timeperiod)?,
            mean0: None,
            mean1: None,
            var0: 0.0,
            var1: 0.0,
            covariance: 0.0,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, left: f64, right: f64) -> f64 {
        let covariance = match (self.mean0, self.mean1) {
            (Some(previous0), Some(previous1)) => {
                let delta0 = left - previous0;
                let delta1 = right - previous1;
                self.mean0 = Some(previous0 + self.alpha * delta0);
                self.mean1 = Some(previous1 + self.alpha * delta1);
                self.var0 = (1.0 - self.alpha) * (self.var0 + self.alpha * delta0 * delta0);
                self.var1 = (1.0 - self.alpha) * (self.var1 + self.alpha * delta1 * delta1);
                (1.0 - self.alpha) * (self.covariance + self.alpha * delta0 * delta1)
            }
            _ => {
                self.mean0 = Some(left);
                self.mean1 = Some(right);
                0.0
            }
        };
        self.covariance = covariance;
        self.value = Some(covariance);
        covariance
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
        self.mean0 = None;
        self.mean1 = None;
        self.var0 = 0.0;
        self.var1 = 0.0;
        self.covariance = 0.0;
        self.value = None;
    }
}
