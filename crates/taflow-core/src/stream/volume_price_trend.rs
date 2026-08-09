//! Batch implementation for `volume_price_trend`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes the causal volume price trend series.
/// Parameters: aligned input slices followed by indicator parameters.
/// Compute the volume price trend result for the supplied aligned series.
///
/// # Parameters
///
/// * `close` - Input series or configuration value.
/// * `volume` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn volume_price_trend(close: &[f64], volume: &[f64]) -> TaResult<Vec<f64>> {
    if close.len() != volume.len() {
        return Err(TaError::LengthMismatch {
            expected: close.len(),
            got: volume.len(),
        });
    }
    let mut state = VolumePriceTrend::new();
    Ok(close
        .iter()
        .zip(volume)
        .map(|(&close, &volume)| state.append(close, volume).unwrap_or(f64::NAN))
        .collect())
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// Stateful Volume-price Trend, aligned to `ta.volume.VolumePriceTrendIndicator`.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `VolumePriceTrend`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct VolumePriceTrend {
    previous_close: Option<f64>,
    total: f64,
    value: Option<f64>,
}

impl VolumePriceTrend {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            previous_close: None,
            total: 0.0,
            value: None,
        }
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, close: f64, volume: f64) -> Option<f64> {
        let previous = self.previous_close.replace(close);
        self.value = previous.map(|previous| {
            if previous != 0.0 {
                self.total += volume * (close - previous) / previous;
            }
            self.total
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
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.total = 0.0;
        self.value = None;
    }
}
