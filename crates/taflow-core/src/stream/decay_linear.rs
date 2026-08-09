//! Persistent WorldQuant Alpha101 linear-decay moving average state.

use super::{StreamingIndicator, WeightedMovingAverage};
use crate::error::TaResult;

/// Linear-decay weighted moving average with a persistent bounded state.
#[derive(Debug, Clone)]
pub struct DecayLinear {
    inner: WeightedMovingAverage,
}

impl DecayLinear {
    /// Construct a validated linear-decay state.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            inner: WeightedMovingAverage::new(timeperiod)?,
        })
    }

    /// Extend the state with chronological values and aligned warm-up output.
    pub fn extend_slice_into(&mut self, input: &[f64], output: &mut Vec<f64>) {
        output.extend(
            input
                .iter()
                .copied()
                .map(|value| self.append(value).unwrap_or(f64::NAN)),
        );
    }
}

impl StreamingIndicator for DecayLinear {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        self.inner.append(input)
    }

    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}
