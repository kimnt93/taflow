//! Stateful trailing percentile indicator.

use crate::error::{TaError, TaResult};

use super::{RollingQuantile, StreamingIndicator};

/// Persistent trailing percentile, expressed on a 0–100 scale.
#[derive(Debug, Clone)]
pub struct RollingPercentile {
    quantile: RollingQuantile,
    percentile: f64,
    value: Option<f64>,
}

impl RollingPercentile {
    /// Create a percentile state with a positive period and percentile in `[0, 100]`.
    pub fn new(timeperiod: usize, percentile: f64) -> TaResult<Self> {
        if !(0.0..=100.0).contains(&percentile) {
            return Err(TaError::InvalidParameter {
                name: "percentile",
                value: percentile.to_string(),
                reason: "must be between 0 and 100",
            });
        }
        Ok(Self {
            quantile: RollingQuantile::new(timeperiod, percentile / 100.0)?,
            percentile,
            value: None,
        })
    }

    /// Append one observation and return the trailing percentile after warm-up.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self.quantile.append(input);
        self.value
    }

    /// Return the latest percentile, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Reset the state without changing its configuration.
    pub fn reset(&mut self) {
        self.quantile.reset();
        self.value = None;
    }

    /// Return the configured percentile for diagnostics.
    pub fn percentile(&self) -> f64 {
        self.percentile
    }
}

impl StreamingIndicator for RollingPercentile {
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

    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        output.extend(
            inputs
                .iter()
                .copied()
                .map(|input| self.append(input).unwrap_or(f64::NAN)),
        );
    }
}
