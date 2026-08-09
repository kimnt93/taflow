//! Persistent `PositionHold` state.

use crate::error::{TaError, TaResult};
use crate::stream::operator_states::*;
use crate::stream::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `PositionHold`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct PositionHold {
    position: f64,
    value: Option<f64>,
}

impl PositionHold {
    /// Create a new empty state.
    ///
    pub fn new() -> Self {
        Self {
            position: 0.0,
            value: None,
        }
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, input: f64) -> f64 {
        if input != 0.0 {
            self.position = input;
        }
        self.value = Some(self.position);
        self.position
    }
    /// Return the latest computed result, if warm-up is complete.
    ///
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Reset the state and clear its accumulated history.
    ///
    pub fn reset(&mut self) {
        self.position = 0.0;
        self.value = None;
    }
}

impl Default for PositionHold {
    fn default() -> Self {
        Self::new()
    }
}
