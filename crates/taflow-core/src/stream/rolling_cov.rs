//! Batch implementation for `rolling_cov`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `rolling_cov` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the rolling cov result for the supplied aligned series.
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
pub fn rolling_cov(input0: &[f64], input1: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if input0.len() != input1.len() {
        return Err(TaError::LengthMismatch {
            expected: input0.len(),
            got: input1.len(),
        });
    }
    let mut state = RollingCov::new(timeperiod)?;
    Ok(input0
        .iter()
        .zip(input1)
        .map(|(&left, &right)| state.append(left, right).unwrap_or(f64::NAN))
        .collect())
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingCov`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingCov {
    values: VecDeque<(f64, f64)>,
    timeperiod: usize,
    value: Option<f64>,
}

impl RollingCov {
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
