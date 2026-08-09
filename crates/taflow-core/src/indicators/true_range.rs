//! Persistent True Range state.

use crate::error::{TaError, TaResult};

/// Compute true range from high, low, and the previous close.
#[derive(Debug, Clone, Default)]
pub struct TrueRange {
    previous_close: Option<f64>,
    value: Option<f64>,
}

impl TrueRange {
    /// Create a fresh True Range state.
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }

    /// Append one chronological high/low/close tuple.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let previous = self.previous_close.replace(close)?;
        self.value = Some(
            (high - low)
                .max((high - previous).abs())
                .max((low - previous).abs()),
        );
        self.value
    }

    /// Append aligned slices in scalar replay order, NaN-filling the first bar.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        let len = high.len();
        for actual in [low.len(), close.len()] {
            if actual != len {
                return Err(TaError::LengthMismatch {
                    expected: len,
                    got: actual,
                });
            }
        }
        output.reserve(len);
        for index in 0..len {
            output.push(
                self.append(high[index], low[index], close[index])
                    .unwrap_or(f64::NAN),
            );
        }
        Ok(())
    }

    /// Return the latest result, or `None` before two bars are present.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restore fresh-state behavior without reallocating.
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.value = None;
    }
}
