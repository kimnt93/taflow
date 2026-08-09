//! Persistent On-Balance Volume state.

use crate::error::{TaError, TaResult};

/// Accumulate volume according to consecutive closing-price direction.
#[derive(Debug, Clone, Default)]
pub struct OnBalanceVolume {
    previous_close: Option<f64>,
    total: f64,
    value: Option<f64>,
}

impl OnBalanceVolume {
    /// Create a fresh On-Balance Volume state.
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }

    /// Append one chronological close/volume pair.
    pub fn append(&mut self, close: f64, volume: f64) -> f64 {
        match self.previous_close.replace(close) {
            None => self.total = volume,
            Some(previous) if close > previous => self.total += volume,
            Some(previous) if close < previous => self.total -= volume,
            Some(_) => {}
        }
        self.value = Some(self.total);
        self.total
    }

    /// Append aligned slices after validating lengths before mutation.
    pub fn extend_slices_into(
        &mut self,
        close: &[f64],
        volume: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        if close.len() != volume.len() {
            return Err(TaError::LengthMismatch {
                expected: close.len(),
                got: volume.len(),
            });
        }
        output.reserve(close.len());
        for index in 0..close.len() {
            output.push(self.append(close[index], volume[index]));
        }
        Ok(())
    }

    /// Return the latest result, or `None` before the first pair.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restore fresh-state behavior without reallocating.
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.total = 0.0;
        self.value = None;
    }
}
