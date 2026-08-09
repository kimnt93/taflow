//! Persistent `EntryExit` state.

use crate::error::{TaError, TaResult};
use crate::stream::operator_states::*;
use crate::stream::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
/// Stateful entry/exit signal helper with causal position transitions.
///
/// The state emits aligned signals and can be reset for replay.
pub struct EntryExit {
    position: f64,
    value: Option<f64>,
}

impl EntryExit {
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
    pub fn append(&mut self, entry: bool, exit: bool) -> f64 {
        if entry && !exit {
            self.position = 1.0
        } else if exit && !entry {
            self.position = -1.0
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

impl Default for EntryExit {
    fn default() -> Self {
        Self::new()
    }
}
