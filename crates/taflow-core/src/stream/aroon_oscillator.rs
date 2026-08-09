//! Persistent Aroon Oscillator state.

use super::Aroon;
use crate::error::TaResult;

/// Compute Aroon Up minus Aroon Down from one persistent extrema state.
#[derive(Debug, Clone)]
pub struct AroonOscillator {
    aroon: Aroon,
    value: Option<f64>,
}

impl AroonOscillator {
    /// Create an oscillator with a lookback of at least two bars.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            aroon: Aroon::new(period)?,
            value: None,
        })
    }

    /// Append one chronological high/low pair.
    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        self.value = self
            .aroon
            .append(high, low)
            .map(|value| value.up - value.down);
        self.value
    }

    /// Append aligned slices, writing NaN during warm-up.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        self.aroon.extend_oscillator_into(high, low, output)?;
        self.value = self.aroon.value().map(|value| value.up - value.down);
        Ok(())
    }

    /// Return the latest result, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restore fresh-state behavior without reallocating.
    pub fn reset(&mut self) {
        self.aroon.reset();
        self.value = None;
    }
}
