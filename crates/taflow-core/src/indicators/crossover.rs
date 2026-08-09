//! Persistent `Crossover` state.

use super::*;
use crate::error::{TaError, TaResult};
use crate::stream::operator_states::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Crossover`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Crossover {
    previous_left: Option<f64>,
    previous_right: Option<f64>,
    value: Option<f64>,
}

impl Crossover {
    /// Create a new empty state.
    ///
    pub fn new() -> Self {
        Self {
            previous_left: None,
            previous_right: None,
            value: None,
        }
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, left: f64, right: f64) -> f64 {
        let value = match (self.previous_left, self.previous_right) {
            (Some(pl), Some(pr)) if pl <= pr && left > right => 1.0,
            _ => 0.0,
        };
        self.previous_left = Some(left);
        self.previous_right = Some(right);
        self.value = Some(value);
        value
    }
    /// Return the latest computed result, if warm-up is complete.
    ///
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Reset the state and clear its accumulated history.
    ///
    pub fn reset(&mut self) {
        self.previous_left = None;
        self.previous_right = None;
        self.value = None;
    }
}

impl Default for Crossover {
    fn default() -> Self {
        Self::new()
    }
}
