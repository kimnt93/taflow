//! Causal logarithmic-return state.

use super::operator_states::validate_period;
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

/// Compute the log return result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn log_return(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_period(timeperiod)?;
    let mut output = vec![f64::NAN; input.len()];
    for index in timeperiod..input.len() {
        output[index] = (input[index] / input[index - timeperiod]).ln();
    }
    Ok(output)
}
