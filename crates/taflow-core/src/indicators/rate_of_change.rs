//! Persistent percentage rate-of-change state.

use crate::TaResult;

use crate::stream::lagged_common::LaggedValue;
use crate::stream::StreamingIndicator;

/// Computes percentage rate of change incrementally.
#[derive(Debug, Clone)]
pub struct RateOfChange {
    lag: LaggedValue,
    value: Option<f64>,
}

impl RateOfChange {
    /// Creates rate-of-change state for a positive lag period.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            lag: LaggedValue::new(period)?,
            value: None,
        })
    }

    /// Appends one value and returns `100 * (current - previous) / previous`.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self.lag.append(input).map(|(current, previous)| {
            if previous != 0.0 {
                (current - previous) / previous * 100.0
            } else {
                0.0
            }
        });
        self.value
    }

    /// Appends a slice and NaN-fills its aligned warm-up positions.
    pub fn extend_slice_into(&mut self, input: &[f64], output: &mut Vec<f64>) {
        if self
            .lag
            .extend_from_empty_into(input, output, |current, previous| {
                if previous != 0.0 {
                    (current - previous) / previous * 100.0
                } else {
                    0.0
                }
            })
        {
            self.value = output.last().copied();
            return;
        }
        output.reserve(input.len());
        output.extend(
            input
                .iter()
                .map(|&value| self.append(value).unwrap_or(f64::NAN)),
        );
    }

    /// Returns the latest percentage rate of change.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restores fresh-state behavior while retaining the allocated ring.
    pub fn reset(&mut self) {
        self.lag.reset();
        self.value = None;
    }
}

impl StreamingIndicator for RateOfChange {
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
