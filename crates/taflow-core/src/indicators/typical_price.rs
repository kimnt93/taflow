//! Persistent pointwise `typical price` transform.

use crate::error::{TaError, TaResult};

/// Compute typical price for aligned chronological prices without warm-up.
#[derive(Debug, Clone, Default)]
pub struct TypicalPrice {
    value: Option<f64>,
}

impl TypicalPrice {
    /// Create a fresh price-transform state.
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }

    /// Transform one chronological price tuple.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> f64 {
        let value = (high + low + close) * (1.0 / 3.0);
        self.value = Some(value);
        value
    }

    /// Transform aligned slices after validating every length before mutation.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        let len = high.len();
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
            high.iter()
                .zip(low)
                .zip(close)
                .map(|((&high, &low), &close)| (high + low + close) * (1.0 / 3.0)),
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
