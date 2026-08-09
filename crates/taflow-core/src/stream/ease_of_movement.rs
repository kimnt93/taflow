//! Batch implementation for `ease_of_movement`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Compute ease of movement from aligned high, low, and volume series.
///
/// The returned series preserves input length and reports `NaN` during its
/// Compute the ease of movement result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `volume` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn ease_of_movement(high: &[f64], low: &[f64], volume: &[f64]) -> TaResult<Vec<f64>> {
    if high.len() != low.len() || high.len() != volume.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len(),
        });
    }
    let mut state = EaseOfMovement::new();
    Ok(high
        .iter()
        .zip(low)
        .zip(volume)
        .map(|((&h, &l), &v)| state.append(h, l, v).unwrap_or(f64::NAN))
        .collect())
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
/// Stateful Ease of Movement oscillator using aligned high, low, and volume.
///
/// The result is causal and retains the latest value between updates.
pub struct EaseOfMovement {
    previous_midpoint: Option<f64>,
    value: Option<f64>,
}

impl EaseOfMovement {
    /// Create a new empty state.
    ///
    pub fn new() -> Self {
        Self {
            previous_midpoint: None,
            value: None,
        }
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, high: f64, low: f64, volume: f64) -> Option<f64> {
        let midpoint = (high + low) * 0.5;
        let previous = self.previous_midpoint.replace(midpoint)?;
        self.value = Some(if volume != 0.0 {
            (midpoint - previous) * (high - low) / volume
        } else {
            0.0
        });
        self.value
    }
    /// Return the latest computed result, if warm-up is complete.
    ///
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Reset the state and clear its accumulated history.
    ///
    pub fn reset(&mut self) {
        self.previous_midpoint = None;
        self.value = None;
    }
}

impl Default for EaseOfMovement {
    fn default() -> Self {
        Self::new()
    }
}
