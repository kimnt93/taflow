//! Batch implementation for `balance_of_power`.

use super::volume_states::*;
use crate::error::{TaError, TaResult};

/// Compute the balance of power result for the supplied aligned series.
///
/// # Parameters
///
/// * `open` - Input series or configuration value.
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `close` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn balance_of_power(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<f64>> {
    if open.len() != high.len() || open.len() != low.len() || open.len() != close.len() {
        return Err(crate::TaError::LengthMismatch {
            expected: open.len(),
            got: high.len().min(low.len()).min(close.len()),
        });
    }
    let mut state = BalanceOfPower::new();
    Ok(open
        .iter()
        .zip(high)
        .zip(low)
        .zip(close)
        .map(|(((&open, &high), &low), &close)| state.append(open, high, low, close))
        .collect())
}
use super::*;

/// Stateful balance of power.
#[derive(Debug, Clone, Default)]
/// Persistent Rust state or aligned output type for `BalanceOfPower`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct BalanceOfPower {
    value: Option<f64>,
}

impl BalanceOfPower {
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
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> f64 {
        let range = high - low;
        let value = if range > 0.0 {
            (close - open) / range
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

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.value = None;
    }
}
