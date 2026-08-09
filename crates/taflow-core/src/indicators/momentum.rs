//! Persistent momentum state.

use crate::error::TaResult;

use crate::stream::lagged_common::LaggedValue;
use crate::stream::StreamingIndicator;

/// Computes the causal difference from the value `period` bars earlier.
#[derive(Debug, Clone)]
pub struct Momentum {
    lag: LaggedValue,
    value: Option<f64>,
}

impl Momentum {
    /// Creates momentum state for a positive lag period.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            lag: LaggedValue::new(period)?,
            value: None,
        })
    }

    /// Appends one chronological value and returns the current momentum.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self
            .lag
            .append(input)
            .map(|(current, previous)| current - previous);
        self.value
    }

    /// Appends a slice and NaN-fills its aligned warm-up positions.
    pub fn extend_slice_into(&mut self, input: &[f64], output: &mut Vec<f64>) {
        output.reserve(input.len());
        output.extend(
            input
                .iter()
                .map(|&value| self.append(value).unwrap_or(f64::NAN)),
        );
    }

    /// Returns the latest momentum, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restores fresh-state behavior while retaining the allocated ring.
    pub fn reset(&mut self) {
        self.lag.reset();
        self.value = None;
    }
}

impl StreamingIndicator for Momentum {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        Self::append(self, input)
    }

    fn value(&self) -> Option<f64> {
        Self::value(self)
    }

    fn reset(&mut self) {
        Self::reset(self);
    }
}
