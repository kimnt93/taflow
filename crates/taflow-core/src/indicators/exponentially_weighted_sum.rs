//! Causal exponentially weighted moving sum.

use crate::error::TaResult;
use crate::stream::ewm_alpha;

/// Persistent exponentially weighted sum with recurrence
/// `sum_t = x_t + (1 - alpha) * sum_(t-1)`.
#[derive(Debug, Clone)]
pub struct ExponentiallyWeightedSum {
    decay: f64,
    value: Option<f64>,
}

impl ExponentiallyWeightedSum {
    /// Create a state using `alpha = 2 / (timeperiod + 1)`.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            decay: 1.0 - ewm_alpha(timeperiod)?,
            value: None,
        })
    }

    /// Append one observation and return the updated weighted sum.
    pub fn append(&mut self, input: f64) -> f64 {
        let value = input + self.decay * self.value.unwrap_or(0.0);
        self.value = Some(value);
        value
    }

    /// Return the latest weighted sum.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clear accumulated weight while retaining the configured decay.
    pub fn reset(&mut self) {
        self.value = None;
    }
}
