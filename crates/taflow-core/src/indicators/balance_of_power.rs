//! Persistent Balance of Power state.

use crate::error::{TaError, TaResult};

/// Measure close-to-open movement relative to the high-low range.
#[derive(Debug, Clone, Default)]
pub struct BalanceOfPower {
    value: Option<f64>,
}

impl BalanceOfPower {
    /// Create a fresh Balance of Power state.
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }

    /// Append one chronological open/high/low/close tuple.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> f64 {
        let range = high - low;
        let value = if range > 0.0 {
            (close - open) / range
        } else {
            0.0
        };
        self.value = Some(value);
        value
    }

    /// Append aligned slices after validating every length before mutation.
    pub fn extend_slices_into(
        &mut self,
        open: &[f64],
        high: &[f64],
        low: &[f64],
        close: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        let len = open.len();
        for actual in [high.len(), low.len(), close.len()] {
            if actual != len {
                return Err(TaError::LengthMismatch {
                    expected: len,
                    got: actual,
                });
            }
        }
        output.reserve(len);
        for index in 0..len {
            output.push(self.append(open[index], high[index], low[index], close[index]));
        }
        Ok(())
    }

    /// Return the latest result, or `None` before the first tuple.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restore fresh-state behavior without reallocating.
    pub fn reset(&mut self) {
        self.value = None;
    }
}
