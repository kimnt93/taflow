//! Persistent cumulative sum state.

use crate::error::TaResult;
use crate::stream::StreamingIndicator;

/// Compute the cumulative sum of chronological scalar observations.
#[derive(Debug, Clone)]
pub struct CumulativeSum {
    total: f64,
    value: Option<f64>,
}

impl CumulativeSum {
    /// Create a fresh cumulative state.
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }

    /// Append one value and return the current cumulative result.
    pub fn append(&mut self, input: f64) -> f64 {
        self.total += input;
        self.value = Some(self.total);
        self.total
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
        self.total = 0.0;
        self.value = None;
    }
}

impl Default for CumulativeSum {
    fn default() -> Self {
        Self {
            total: 0.0,
            value: None,
        }
    }
}

impl StreamingIndicator for CumulativeSum {
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
