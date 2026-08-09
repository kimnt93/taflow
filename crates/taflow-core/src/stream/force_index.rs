//! Batch implementation for `force_index`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Compute the force index from aligned close and volume series.
///
/// The result is input-aligned and uses `NaN` while the previous close is
/// Compute the force index result for the supplied aligned series.
///
/// # Parameters
///
/// * `close` - Input series or configuration value.
/// * `volume` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn force_index(close: &[f64], volume: &[f64]) -> TaResult<Vec<f64>> {
    if close.len() != volume.len() {
        return Err(TaError::LengthMismatch {
            expected: close.len(),
            got: volume.len(),
        });
    }
    let mut state = ForceIndex::new();
    Ok(close
        .iter()
        .zip(volume)
        .map(|(&c, &v)| state.append(c, v).unwrap_or(f64::NAN))
        .collect())
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
/// Stateful Force Index derived from close-to-close change and volume.
///
/// The state preserves warm-up behavior and supports append/reset updates.
pub struct ForceIndex {
    previous: Option<f64>,
    value: Option<f64>,
}

impl ForceIndex {
    /// Create a new empty state.
    ///
    pub fn new() -> Self {
        Self {
            previous: None,
            value: None,
        }
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, close: f64, volume: f64) -> Option<f64> {
        let previous = self.previous.replace(close)?;
        self.value = Some((close - previous) * volume);
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
        self.previous = None;
        self.value = None;
    }
}

impl Default for ForceIndex {
    fn default() -> Self {
        Self::new()
    }
}
