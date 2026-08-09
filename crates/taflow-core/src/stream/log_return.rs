//! Causal logarithmic-return state.

use super::StreamingIndicator;
use crate::{stream::Lag, TaResult};

/// Computes `ln(x_t / x_(t-n))` with causal warm-up.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `LogReturn`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
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
        self.value = self
            .lag
            .append(input)
            .map(|previous| (input / previous).ln());
        self.value
    }

    /// Append a slice into `output` with `NaN` at warm-up positions.
    pub fn extend_slice_into(&mut self, input: &[f64], output: &mut Vec<f64>) {
        output.reserve(input.len());
        output.extend(
            input
                .iter()
                .map(|&input| self.append(input).unwrap_or(f64::NAN)),
        );
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

impl StreamingIndicator for LogReturn {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<Self::Output> {
        Self::append(self, input)
    }

    fn value(&self) -> Option<Self::Output> {
        Self::value(self)
    }

    fn reset(&mut self) {
        Self::reset(self);
    }
}
