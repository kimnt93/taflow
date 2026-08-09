//! Persistent Accumulation/Distribution state.

use crate::error::{TaError, TaResult};
use crate::stream::accumulation_distribution_helper::money_flow_volume;

/// Accumulate close-location value multiplied by volume.
#[derive(Debug, Clone, Default)]
pub struct AccumulationDistribution {
    total: f64,
    value: Option<f64>,
}

impl AccumulationDistribution {
    /// Create a fresh Accumulation/Distribution state.
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }

    /// Append one chronological high/low/close/volume tuple.
    pub fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> f64 {
        self.total += money_flow_volume(high, low, close, volume);
        self.value = Some(self.total);
        self.total
    }

    /// Append aligned slices after validating every length before mutation.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        volume: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        let len = high.len();
        for actual in [low.len(), close.len(), volume.len()] {
            if actual != len {
                return Err(TaError::LengthMismatch {
                    expected: len,
                    got: actual,
                });
            }
        }
        output.reserve(len);
        for index in 0..len {
            output.push(self.append(high[index], low[index], close[index], volume[index]));
        }
        Ok(())
    }

    /// Return the latest result, or `None` before the first tuple.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restore fresh-state behavior without reallocating.
    pub fn reset(&mut self) {
        self.total = 0.0;
        self.value = None;
    }
}
