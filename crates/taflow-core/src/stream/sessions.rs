//! Batch implementation for `sessions`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `sessions` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn sessions(
    new_session: &[bool],
    high: &[f64],
    low: &[f64],
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if new_session.len() != high.len() || high.len() != low.len() {
        return Err(TaError::LengthMismatch {
            expected: new_session.len(),
            got: high.len().max(low.len()),
        });
    }
    let mut state = Sessions::new();
    let mut active = Vec::with_capacity(high.len());
    let mut session_high = Vec::with_capacity(high.len());
    let mut session_low = Vec::with_capacity(high.len());
    for ((&new_session, &high), &low) in new_session.iter().zip(high).zip(low) {
        let value = state.append(new_session, high, low);
        active.push(value.active);
        session_high.push(value.session_high);
        session_low.push(value.session_low);
    }
    Ok((active, session_high, session_low))
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `SessionsValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SessionsValue {
    pub active: f64,
    pub session_high: f64,
    pub session_low: f64,
}

/// Causal session-scoped extrema. Given a session-boundary flag series,
/// emits a constant `active` marker and the running high/low since the last
/// boundary — matching the package's causal running extrema.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Sessions`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Sessions {
    session_high: Option<f64>,
    session_low: Option<f64>,
    started: bool,
    value: Option<SessionsValue>,
}

impl Sessions {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            session_high: None,
            session_low: None,
            started: false,
            value: None,
        }
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, new_session: bool, high: f64, low: f64) -> SessionsValue {
        if new_session || !self.started {
            self.session_high = Some(high);
            self.session_low = Some(low);
            self.started = true;
        } else {
            self.session_high = Some(self.session_high.map_or(high, |running| running.max(high)));
            self.session_low = Some(self.session_low.map_or(low, |running| running.min(low)));
        }
        let value = SessionsValue {
            active: 1.0,
            session_high: self.session_high.unwrap_or(f64::NAN),
            session_low: self.session_low.unwrap_or(f64::NAN),
        };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<SessionsValue> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.session_high = None;
        self.session_low = None;
        self.started = false;
        self.value = None;
    }
}

impl Default for Sessions {
    fn default() -> Self {
        Self::new()
    }
}
