//! Persistent cumulative maximum state.

use crate::error::TaResult;
use crate::stream::StreamingIndicator;

/// Compute the cumulative maximum of chronological scalar observations.
#[derive(Debug, Clone)]
pub struct CumulativeMaximum {
    extreme: f64,
    value: Option<f64>,
}

impl CumulativeMaximum {
    /// Create a fresh cumulative state.
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }

    /// Append one value and return the current cumulative result.
    pub fn append(&mut self, input: f64) -> f64 {
        self.extreme = self.extreme.max(input);
        self.value = Some(self.extreme);
        self.extreme
    }

    /// Append a slice into `output` in scalar replay order.
    pub fn extend_slice_into(&mut self, input: &[f64], output: &mut Vec<f64>) {
        output.reserve(input.len());
        output.extend(input.iter().map(|&input| self.append(input)));
    }

    /// Return the latest result, or `None` before the first observation.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restore fresh-state behavior without reallocating.
    pub fn reset(&mut self) {
        self.extreme = f64::NEG_INFINITY;
        self.value = None;
    }
}

impl Default for CumulativeMaximum {
    fn default() -> Self {
        Self {
            extreme: f64::NEG_INFINITY,
            value: None,
        }
    }
}

impl StreamingIndicator for CumulativeMaximum {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<Self::Output> {
        Some(Self::append(self, input))
    }

    fn value(&self) -> Option<Self::Output> {
        Self::value(self)
    }

    fn reset(&mut self) {
        Self::reset(self);
    }
}
