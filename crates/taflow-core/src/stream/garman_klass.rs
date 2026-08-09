//! Batch implementation for `garman_klass`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `garman_klass` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn garman_klass(
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
    let mut state = GarmanKlass::new(timeperiod)?;
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

/// Rolling mean of `0.5·ln(H/L)² − (2ln2−1)·ln(C/O)²` (Garman-Klass).
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `GarmanKlass`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct GarmanKlass {
    mean: RollingMean,
    value: Option<f64>,
}

impl GarmanKlass {
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
        let term = if high > 0.0 && low > 0.0 && open > 0.0 && close > 0.0 {
            0.5 * (high / low).ln().powi(2)
                - (2.0 * 2.0f64.ln() - 1.0) * (close / open).ln().powi(2)
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
