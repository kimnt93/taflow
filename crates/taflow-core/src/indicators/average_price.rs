//! Persistent pointwise `average price` transform.

use crate::error::{TaError, TaResult};

/// Compute average price for aligned chronological prices without warm-up.
#[derive(Debug, Clone, Default)]
pub struct AveragePrice {
    value: Option<f64>,
}

impl AveragePrice {
    /// Create a fresh price-transform state.
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }

    /// Transform one chronological price tuple.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> f64 {
        let value = (open + high + low + close) * 0.25;
        self.value = Some(value);
        value
    }

    /// Transform aligned slices after validating every length before mutation.
    pub fn extend_slices_into(
        &mut self,
        open: &[f64],
        high: &[f64],
        low: &[f64],
        close: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        let len = open.len();
        if high.len() != len {
            return Err(TaError::LengthMismatch {
                expected: len,
                got: high.len(),
            });
        }
        if low.len() != len {
            return Err(TaError::LengthMismatch {
                expected: len,
                got: low.len(),
            });
        }
        if close.len() != len {
            return Err(TaError::LengthMismatch {
                expected: len,
                got: close.len(),
            });
        }
        output.reserve(len);
        output.extend(
            open.iter()
                .zip(high)
                .zip(low)
                .zip(close)
                .map(|(((&open, &high), &low), &close)| (open + high + low + close) * 0.25),
        );
        if let Some(&value) = output.last().filter(|_| len != 0) {
            self.value = Some(value);
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
