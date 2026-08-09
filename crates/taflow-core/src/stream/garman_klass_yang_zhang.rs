//! Batch implementation for `garman_klass_yang_zhang`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes the Garman-Klass/Yang-Zhang volatility estimate.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn garman_klass_yang_zhang(
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
    let mut state = GarmanKlassYangZhang::new(timeperiod)?;
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

/// Garman-Klass with the overnight term `ln(O/C_prev)²` added (GK-Yang-Zhang).
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `GarmanKlassYangZhang`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct GarmanKlassYangZhang {
    mean: RollingMean,
    previous_close: Option<f64>,
    value: Option<f64>,
}

impl GarmanKlassYangZhang {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            mean: RollingMean::new(timeperiod)?,
            previous_close: None,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<f64> {
        if let Some(previous_close) = self.previous_close.replace(close) {
            let term =
                if open > 0.0 && high > 0.0 && low > 0.0 && close > 0.0 && previous_close > 0.0 {
                    let gk = 0.5 * (high / low).ln().powi(2)
                        - (2.0 * 2.0f64.ln() - 1.0) * (close / open).ln().powi(2);
                    let overnight = (open / previous_close).ln().powi(2);
                    gk + overnight
                } else {
                    0.0
                };
            self.value = self.mean.append(term).map(|mean| mean.sqrt());
        }
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
        self.previous_close = None;
        self.value = None;
    }
}
