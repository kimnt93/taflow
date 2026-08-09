//! Persistent rate-of-change-ratio state.

use crate::stream::lagged_common::LaggedValue;
use crate::stream::StreamingIndicator;
use crate::TaResult;

/// Computes the lagged value ratio incrementally.
#[derive(Debug, Clone)]
pub struct RateOfChangeRatio {
    lag: LaggedValue,
    value: Option<f64>,
}
impl RateOfChangeRatio {
    /// Creates ratio state for a positive lag period.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            lag: LaggedValue::new(period)?,
            value: None,
        })
    }

    /// Appends one value and returns `current / previous`.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self.lag.append(input).map(|(current, previous)| {
            if previous != 0.0 {
                current / previous
            } else {
                0.0
            }
        });
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

    /// Returns the latest rate-of-change ratio.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restores fresh-state behavior while retaining the allocated ring.
    pub fn reset(&mut self) {
        self.lag.reset();
        self.value = None;
    }
}
impl StreamingIndicator for RateOfChangeRatio {
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
