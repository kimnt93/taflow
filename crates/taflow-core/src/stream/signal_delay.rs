//! Persistent `SignalDelay` state.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `SignalDelay`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SignalDelay {
    values: VecDeque<f64>,
    period: usize,
    value: Option<f64>,
}

impl SignalDelay {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            values: VecDeque::with_capacity(period),
            period,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.value = if self.values.len() == self.period {
            let value = self.values.pop_front();
            self.values.push_back(input);
            value
        } else {
            self.values.push_back(input);
            None
        };
        self.value
    }
    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}
