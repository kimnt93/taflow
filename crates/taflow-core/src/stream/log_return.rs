//! Causal logarithmic-return state.

use crate::{TaResult, stream::Lag};

/// Computes `ln(x_t / x_(t-n))` with causal warm-up.
#[derive(Debug, Clone)]
pub struct LogReturn {
    lag: Lag,
    value: Option<f64>,
}

impl LogReturn {
    /// Creates a logarithmic-return state with the requested lag.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            lag: Lag::new(timeperiod)?,
            value: None,
        })
    }

    /// Appends one price and returns its logarithmic return when warmed up.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self.lag.append(input).map(|previous| (input / previous).ln());
        self.value
    }

    /// Returns the latest logarithmic return, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clears the lag and latest return.
    pub fn reset(&mut self) {
        self.lag.reset();
        self.value = None;
    }
}
