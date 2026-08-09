//! Batch implementation for `rogers_satchell`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `rogers_satchell` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn rogers_satchell(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
    timeperiod: usize,
) -> TaResult<Vec<f64>> {
    if open.len() != high.len() || high.len() != low.len() || low.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: open.len(),
            got: high.len().max(low.len()).max(close.len()),
        });
    }
    let mut state = RogersSatchell::new(timeperiod)?;
    Ok(open
        .iter()
        .zip(high)
        .zip(low)
        .zip(close)
        .map(|(((&open, &high), &low), &close)| {
            state.append(open, high, low, close).unwrap_or(f64::NAN)
        })
        .collect())
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// Rolling mean of `ln(H/C)ln(H/O) + ln(L/C)ln(L/O)` (Rogers-Satchell).
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RogersSatchell`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RogersSatchell {
    mean: RollingMean,
    value: Option<f64>,
}

impl RogersSatchell {
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
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<f64> {
        let term = if open > 0.0 && high > 0.0 && low > 0.0 && close > 0.0 {
            (high / close).ln() * (high / open).ln() + (low / close).ln() * (low / open).ln()
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
