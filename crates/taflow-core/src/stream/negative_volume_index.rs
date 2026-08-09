//! Batch implementation for `negative_volume_index`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes the causal negative volume index series.
/// Parameters: aligned input slices followed by indicator parameters.
/// Compute the negative volume index result for the supplied aligned series.
///
/// # Parameters
///
/// * `close` - Input series or configuration value.
/// * `volume` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn negative_volume_index(close: &[f64], volume: &[f64]) -> TaResult<Vec<f64>> {
    if close.len() != volume.len() {
        return Err(TaError::LengthMismatch {
            expected: close.len(),
            got: volume.len(),
        });
    }
    let mut state = VolumeIndex::new(VolumeIndexMode::Negative);
    Ok(close
        .iter()
        .zip(volume)
        .map(|(&c, &v)| state.append(c, v))
        .collect())
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// Persistent Rust state or aligned output type for `NegativeVolumeIndex`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct NegativeVolumeIndex(VolumeIndex);

impl NegativeVolumeIndex {
    /// Create a new empty state.
    ///
    pub fn new() -> Self {
        Self(VolumeIndex::new(VolumeIndexMode::Negative))
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, close: f64, volume: f64) -> f64 {
        self.0.append(close, volume)
    }
    /// Return the latest computed result, if warm-up is complete.
    ///
    pub fn value(&self) -> f64 {
        self.0.value
    }
    /// Reset the state and clear its accumulated history.
    ///
    pub fn reset(&mut self) {
        self.0.reset();
    }
}
