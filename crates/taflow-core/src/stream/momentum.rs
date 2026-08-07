//! Stateful and vectorized momentum.

use crate::error::{TaError, TaResult};

use super::{lagged_common::LaggedValue, StreamingIndicator};

/// Compute the momentum result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn momentum(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if timeperiod == 0 {
        return Err(TaError::InvalidParameter {
            name: "timeperiod",
            value: "0".to_string(),
            reason: "must be >= 1",
        });
    }
    let mut state = Momentum::new(timeperiod)?;
    Ok(input
        .iter()
        .map(|&value| state.append(value).unwrap_or(f64::NAN))
        .collect())
}

/// Computes the causal difference from the value `period` bars earlier.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Momentum`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
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
}

impl StreamingIndicator for Momentum {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self
            .lag
            .append(input)
            .map(|(current, previous)| current - previous);
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.lag.reset();
        self.value = None;
    }
}
