//! Persistent pointwise `median price` transform.

use crate::error::{TaError, TaResult};

/// Compute median price for aligned chronological prices without warm-up.
#[derive(Debug, Clone, Default)]
pub struct MedianPrice {
    value: Option<f64>,
}

impl MedianPrice {
    /// Create a fresh price-transform state.
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }

    /// Transform one chronological price tuple.
    pub fn append(&mut self, high: f64, low: f64) -> f64 {
        let value = (high + low) * 0.5;
        self.value = Some(value);
        value
    }

    /// Transform aligned slices after validating every length before mutation.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        let len = high.len();
        if low.len() != len {
            return Err(TaError::LengthMismatch {
                expected: len,
                got: low.len(),
            });
        }
        output.reserve(len);
        for index in 0..len {
            output.push(self.append(high[index], low[index]));
        }
        Ok(())
    }

    /// Return the latest result, or `None` before the first price tuple.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restore fresh-state behavior without reallocating.
    pub fn reset(&mut self) {
        self.value = None;
    }
}
