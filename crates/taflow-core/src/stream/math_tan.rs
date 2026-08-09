//! Persistent pointwise `tan` transform.

use super::StreamingIndicator;
use crate::error::TaResult;

/// Apply `tan` to each value without warm-up.
#[derive(Debug, Clone, Default)]
pub struct MathTan {
    value: Option<f64>,
}

impl MathTan {
    /// Create a fresh pointwise transform state.
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }

    /// Transform one chronological value.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.value = Some(input.tan());
        self.value
    }

    /// Transform a slice into `output` while preserving scalar replay order.
    pub fn extend_slice_into(&mut self, input: &[f64], output: &mut Vec<f64>) {
        output.reserve(input.len());
        output.extend(input.iter().map(|&input| {
            self.append(input)
                .expect("pointwise transforms have no warm-up")
        }));
    }

    /// Return the latest result, or `None` before the first value.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restore fresh-state behavior without reallocating.
    pub fn reset(&mut self) {
        self.value = None;
    }
}

impl StreamingIndicator for MathTan {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<Self::Output> {
        Self::append(self, input)
    }

    fn value(&self) -> Option<Self::Output> {
        Self::value(self)
    }

    fn reset(&mut self) {
        Self::reset(self);
    }
}
