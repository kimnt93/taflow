//! Batch implementation for `parkinson`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `parkinson` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn parkinson(high: &[f64], low: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if high.len() != low.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len(),
        });
    }
    let mut state = Parkinson::new(timeperiod)?;
    Ok(high
        .iter()
        .zip(low)
        .map(|(&high, &low)| state.append(high, low).unwrap_or(f64::NAN))
        .collect())
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// Rolling mean of `ln(H/L)² / (4 ln 2)` (Parkinson volatility).
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Parkinson`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Parkinson {
    mean: RollingMean,
    value: Option<f64>,
}

impl Parkinson {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            mean: RollingMean::new(timeperiod)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        let term = if high > low && high > 0.0 && low > 0.0 {
            (high / low).ln().powi(2) / (4.0 * 2.0f64.ln())
        } else {
            0.0
        };
        self.value = self.mean.append(term).map(|mean| mean.sqrt());
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
        self.value = None;
    }
}
