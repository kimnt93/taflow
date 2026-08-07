//! Fixed-capacity storage shared by bounded-window indicators.

use std::collections::VecDeque;

use crate::error::TaResult;

use super::invalid_period;

/// A fixed-capacity FIFO that allocates only during construction.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Window`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Window {
    values: VecDeque<f64>,
    capacity: usize,
}

impl Window {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(capacity: usize) -> TaResult<Self> {
        if capacity == 0 {
            return Err(invalid_period("capacity", capacity, 1));
        }
        Ok(Self {
            values: VecDeque::with_capacity(capacity),
            capacity,
        })
    }

    /// Appends `value`, returning the value evicted from a full window.
    pub fn push(&mut self, value: f64) -> Option<f64> {
        let evicted = if self.values.len() == self.capacity {
            self.values.pop_front()
        } else {
            None
        };
        self.values.push_back(value);
        evicted
    }

    /// Computes or updates `len` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Computes or updates `is_full` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn is_full(&self) -> bool {
        self.len() == self.capacity
    }

    /// Computes or updates `iter` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &f64> + ExactSizeIterator {
        self.values.iter()
    }

    /// Computes or updates `clear` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn clear(&mut self) {
        self.values.clear();
    }
}
