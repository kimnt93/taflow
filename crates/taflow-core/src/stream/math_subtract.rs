//! Persistent pointwise subtraction.

use crate::error::{TaError, TaResult};

/// Subtract aligned right values from left values without warm-up.
#[derive(Debug, Clone, Default)]
pub struct MathSubtract {
    value: Option<f64>,
}

impl MathSubtract {
    /// Create a fresh subtraction state.
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }

    /// Subtract one aligned pair and return its result.
    pub fn append(&mut self, left: f64, right: f64) -> f64 {
        let value = left - right;
        self.value = Some(value);
        value
    }

    /// Subtract aligned slices into `output`, validating lengths first.
    pub fn extend_slices_into(
        &mut self,
        left: &[f64],
        right: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        if left.len() != right.len() {
            return Err(TaError::LengthMismatch {
                expected: left.len(),
                got: right.len(),
            });
        }
        output.reserve(left.len());
        output.extend(
            left.iter()
                .zip(right)
                .map(|(&left, &right)| self.append(left, right)),
        );
        Ok(())
    }

    /// Return the latest difference, or `None` before the first pair.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restore fresh-state behavior without reallocating.
    pub fn reset(&mut self) {
        self.value = None;
    }
}
