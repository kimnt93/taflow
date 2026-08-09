//! Batch implementation for `session_extrema`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Running high and low values reset by an explicit session boundary.
///
/// The boundary is supplied as an aligned boolean input. The first bar is
/// treated as the beginning of a session when `new_session` is false.
pub fn session_extrema(
    new_session: &[bool],
    high: &[f64],
    low: &[f64],
) -> TaResult<(Vec<f64>, Vec<f64>)> {
    if new_session.len() != high.len() || high.len() != low.len() {
        return Err(TaError::LengthMismatch {
            expected: new_session.len(),
            got: high.len().max(low.len()),
        });
    }
    let mut state = SessionExtrema::new();
    let mut session_high = Vec::with_capacity(high.len());
    let mut session_low = Vec::with_capacity(low.len());
    for ((&new_session, &high), &low) in new_session.iter().zip(high).zip(low) {
        let value = state.append(new_session, high, low);
        session_high.push(value.high);
        session_low.push(value.low);
    }
    Ok((session_high, session_low))
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `SessionExtremaValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SessionExtremaValue {
    pub high: f64,
    pub low: f64,
}

#[derive(Debug, Clone, Default)]
/// Persistent Rust state or aligned output type for `SessionExtrema`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SessionExtrema {
    high: Option<f64>,
    low: Option<f64>,
    value: Option<SessionExtremaValue>,
}

impl SessionExtrema {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, new_session: bool, high: f64, low: f64) -> SessionExtremaValue {
        if new_session || self.high.is_none() {
            self.high = Some(high);
            self.low = Some(low);
        } else {
            self.high = Some(self.high.expect("session high is initialized").max(high));
            self.low = Some(self.low.expect("session low is initialized").min(low));
        }
        let value = SessionExtremaValue {
            high: self.high.expect("session high is initialized"),
            low: self.low.expect("session low is initialized"),
        };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<SessionExtremaValue> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.high = None;
        self.low = None;
        self.value = None;
    }
}
