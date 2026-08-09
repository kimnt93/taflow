//! Persistent pointwise `cbrt` transform.

use crate::error::TaResult;
use crate::stream::StreamingIndicator;

/// Apply `cbrt` to each value without warm-up.
#[derive(Debug, Clone, Default)]
pub struct MathCbrt {
    value: Option<f64>,
}

impl MathCbrt {
    /// Create a fresh pointwise transform state.
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }

    /// Transform one chronological value.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.value = Some(input.cbrt());
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

impl StreamingIndicator for MathCbrt {
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
