//! Batch implementation for `on_balance_volume`.

use super::volume_states::*;
use crate::error::{TaError, TaResult};

/// Compute the on balance volume result for the supplied aligned series.
///
/// # Parameters
///
/// * `close` - Input series or configuration value.
/// * `volume` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn on_balance_volume(close: &[f64], volume: &[f64]) -> TaResult<Vec<f64>> {
    if close.len() != volume.len() {
        return Err(crate::TaError::LengthMismatch {
            expected: close.len(),
            got: volume.len(),
        });
    }
    let mut state = OnBalanceVolume::new();
    Ok(close
        .iter()
        .zip(volume)
        .map(|(&close, &volume)| state.append(close, volume))
        .collect())
}
use super::*;

/// Stateful on-balance volume.
#[derive(Debug, Clone, Default)]
/// Persistent Rust state or aligned output type for `OnBalanceVolume`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct OnBalanceVolume {
    previous_close: Option<f64>,
    total: f64,
    value: Option<f64>,
}

impl OnBalanceVolume {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self::default()
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, close: f64, volume: f64) -> f64 {
        match self.previous_close.replace(close) {
            None => self.total = volume,
            Some(previous) if close > previous => self.total += volume,
            Some(previous) if close < previous => self.total -= volume,
            Some(_) => {}
        }
        self.value = Some(self.total);
        self.total
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.total = 0.0;
        self.value = None;
    }
}
