//! Batch implementation for `normalized_average_true_range`.

use super::aroon_true_range::*;
use crate::error::{TaError, TaResult};

/// Compute the normalized average true range result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `close` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn normalized_average_true_range(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    timeperiod: usize,
) -> TaResult<Vec<f64>> {
    if high.len() != low.len() || high.len() != close.len() {
        return Err(crate::TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().min(close.len()),
        });
    }
    let mut state = NormalizedAverageTrueRange::new(timeperiod)?;
    Ok(high
        .iter()
        .zip(low)
        .zip(close)
        .map(|((high, low), close)| state.append(*high, *low, *close).unwrap_or(f64::NAN))
        .collect())
}
use super::*;

/// Stateful normalized ATR, matching `NATR = ATR / close * 100`.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `NormalizedAverageTrueRange`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct NormalizedAverageTrueRange {
    atr: AverageTrueRange,
    value: Option<f64>,
}

impl NormalizedAverageTrueRange {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            atr: AverageTrueRange::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        self.value = self.atr.append(high, low, close).map(|atr| {
            if close == 0.0 {
                0.0
            } else {
                atr / close * 100.0
            }
        });
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

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.atr.reset();
        self.value = None;
    }
}
