//! Batch implementation for `accumulation_distribution`.

use super::volume_states::*;
use crate::error::{TaError, TaResult};

/// Compute the accumulation distribution result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `close` - Input series or configuration value.
/// * `volume` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn accumulation_distribution(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    volume: &[f64],
) -> TaResult<Vec<f64>> {
    if high.len() != low.len() || high.len() != close.len() || high.len() != volume.len() {
        return Err(crate::TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().min(close.len()).min(volume.len()),
        });
    }
    let mut state = AccumulationDistribution::new();
    Ok(high
        .iter()
        .zip(low)
        .zip(close)
        .zip(volume)
        .map(|(((&high, &low), &close), &volume)| state.append(high, low, close, volume))
        .collect())
}
use super::*;

/// Stateful Chaikin accumulation/distribution line.
#[derive(Debug, Clone, Default)]
/// Persistent Rust state or aligned output type for `AccumulationDistribution`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct AccumulationDistribution {
    total: f64,
    value: Option<f64>,
}

impl AccumulationDistribution {
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
    pub fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> f64 {
        self.total += ad_increment(high, low, close, volume);
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
        self.total = 0.0;
        self.value = None;
    }
}
