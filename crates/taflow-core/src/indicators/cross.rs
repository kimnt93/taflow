//! Persistent `Cross` state.

use super::*;
use crate::error::{TaError, TaResult};
use crate::stream::operator_states::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Cross`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Cross {
    crossover: Crossover,
    crossunder: Crossunder,
    value: Option<f64>,
}

impl Cross {
    /// Create a new empty state.
    ///
    pub fn new() -> Self {
        Self {
            crossover: Crossover::new(),
            crossunder: Crossunder::new(),
            value: None,
        }
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, left: f64, right: f64) -> f64 {
        let value =
            (self.crossover.append(left, right) + self.crossunder.append(left, right)).min(1.0);
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
        self.crossover.reset();
        self.crossunder.reset();
        self.value = None;
    }
}

impl Default for Cross {
    fn default() -> Self {
        Self::new()
    }
}
