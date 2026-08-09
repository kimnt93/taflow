//! Batch implementation for `amihud`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `amihud` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the amihud result for the supplied aligned series.
///
/// # Parameters
///
/// * `close` - Input series or configuration value.
/// * `volume` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn amihud(close: &[f64], volume: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if close.len() != volume.len() {
        return Err(TaError::LengthMismatch {
            expected: close.len(),
            got: volume.len(),
        });
    }
    let mut state = Amihud::new(timeperiod)?;
    Ok(close
        .iter()
        .zip(volume)
        .map(|(&close, &volume)| state.append(close, volume).unwrap_or(f64::NAN))
        .collect())
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// Amihud illiquidity: rolling mean of `|ret| / (close × volume)`.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Amihud`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Amihud {
    mean: RollingMean,
    previous_close: Option<f64>,
    value: Option<f64>,
}

impl Amihud {
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
    pub fn append(&mut self, close: f64, volume: f64) -> Option<f64> {
        if let Some(previous_close) = self.previous_close.replace(close) {
            let term = if close > 0.0 && previous_close > 0.0 && volume > 0.0 {
                ((close - previous_close) / previous_close).abs() / (close * volume)
            } else {
                0.0
            };
            self.value = self.mean.append(term);
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
