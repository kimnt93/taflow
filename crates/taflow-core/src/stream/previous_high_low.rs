//! Batch implementation for `previous_high_low`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `previous_high_low` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn previous_high_low(
    new_session: &[bool],
    high: &[f64],
    low: &[f64],
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> {
    if new_session.len() != high.len() || high.len() != low.len() {
        return Err(TaError::LengthMismatch {
            expected: new_session.len(),
            got: high.len().max(low.len()),
        });
    }
    let mut state = PreviousHighLow::new();
    let mut prev_high = Vec::with_capacity(high.len());
    let mut prev_low = Vec::with_capacity(high.len());
    let mut broken_high = Vec::with_capacity(high.len());
    let mut broken_low = Vec::with_capacity(high.len());
    for ((&new_session, &high), &low) in new_session.iter().zip(high).zip(low) {
        let value = state.append(new_session, high, low);
        prev_high.push(value.prev_high);
        prev_low.push(value.prev_low);
        broken_high.push(value.broken_high);
        broken_low.push(value.broken_low);
    }
    Ok((prev_high, prev_low, broken_high, broken_low))
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `PreviousHighLowValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct PreviousHighLowValue {
    pub prev_high: f64,
    pub prev_low: f64,
    pub broken_high: f64,
    pub broken_low: f64,
}

/// Causal prior-higher-timeframe high/low tracking with break flags. Given a
/// HTF boundary flag series, running extrema are snapshotted into
/// `prev_high`/`prev_low` at each boundary; breaks are flagged when the
/// current bar trades beyond the previous HTF bar's extrema.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `PreviousHighLow`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct PreviousHighLow {
    running_high: Option<f64>,
    running_low: Option<f64>,
    previous_high: Option<f64>,
    previous_low: Option<f64>,
    value: Option<PreviousHighLowValue>,
}

impl PreviousHighLow {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            running_high: None,
            running_low: None,
            previous_high: None,
            previous_low: None,
            value: None,
        }
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, new_session: bool, high: f64, low: f64) -> PreviousHighLowValue {
        if new_session {
            if self.running_high.is_some() {
                self.previous_high = self.running_high;
                self.previous_low = self.running_low;
            }
            self.running_high = Some(high);
            self.running_low = Some(low);
        } else {
            self.running_high = Some(self.running_high.map_or(high, |running| running.max(high)));
            self.running_low = Some(self.running_low.map_or(low, |running| running.min(low)));
        }

        let broken_high =
            self.previous_high.map_or(
                f64::NAN,
                |previous| if high > previous { 1.0 } else { f64::NAN },
            );
        let broken_low =
            self.previous_low.map_or(
                f64::NAN,
                |previous| if low < previous { 1.0 } else { f64::NAN },
            );

        let value = PreviousHighLowValue {
            prev_high: self.previous_high.unwrap_or(f64::NAN),
            prev_low: self.previous_low.unwrap_or(f64::NAN),
            broken_high,
            broken_low,
        };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<PreviousHighLowValue> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.running_high = None;
        self.running_low = None;
        self.previous_high = None;
        self.previous_low = None;
        self.value = None;
    }
}

impl Default for PreviousHighLow {
    fn default() -> Self {
        Self::new()
    }
}
