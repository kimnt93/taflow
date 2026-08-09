//! Persistent pointwise division.

use crate::error::{TaError, TaResult};

/// Divide aligned left values by right values without warm-up.
#[derive(Debug, Clone, Default)]
pub struct MathDivide {
    value: Option<f64>,
}

impl MathDivide {
    /// Create a fresh division state.
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }

    /// Divide one aligned pair and return its IEEE-754 result.
    pub fn append(&mut self, left: f64, right: f64) -> f64 {
        let value = left / right;
        self.value = Some(value);
        value
    }

    /// Divide aligned slices into `output`, validating lengths first.
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

    /// Return the latest quotient, or `None` before the first pair.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restore fresh-state behavior without reallocating.
    pub fn reset(&mut self) {
        self.value = None;
    }
}
