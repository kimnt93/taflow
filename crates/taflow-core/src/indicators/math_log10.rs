//! Persistent pointwise `log10` transform.

use crate::error::TaResult;
use crate::stream::StreamingIndicator;

/// Apply `log10` to each value without warm-up.
#[derive(Debug, Clone, Default)]
pub struct MathLog10 {
    value: Option<f64>,
}

impl MathLog10 {
    /// Create a fresh pointwise transform state.
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }

    /// Transform one chronological value.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.value = Some(input.log10());
        self.value
    }

    /// Transform a slice into `output` while preserving scalar replay order.
    pub fn extend_slice_into(&mut self, input: &[f64], output: &mut Vec<f64>) {
        let start = output.len();
        output.resize(start + input.len(), 0.0);
        for (slot, &input) in output[start..].iter_mut().zip(input) {
            *slot = input.log10();
        }
        if !input.is_empty() {
            self.value = Some(output[start + input.len() - 1]);
        }
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

impl StreamingIndicator for MathLog10 {
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
