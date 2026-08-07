//! Stateful and vectorized percentage rate of change.

use crate::TaResult;

use super::{
    lagged_common::{validate_rate_of_change, LaggedValue},
    StreamingIndicator,
};

/// Compute the rate of change result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rate_of_change(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_rate_of_change(input, timeperiod)?;
    let mut output = vec![f64::NAN; timeperiod];
    output.extend(
        input[timeperiod..]
            .iter()
            .zip(&input[..input.len() - timeperiod])
            .map(|(&current, &previous)| {
                if previous != 0.0 {
                    (current - previous) / previous * 100.0
                } else {
                    0.0
                }
            }),
    );
    Ok(output)
}

/// Computes percentage rate of change incrementally.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RateOfChange`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
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
}

impl StreamingIndicator for RateOfChange {
    type Output = f64;
    fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self.lag.append(input).map(|(current, previous)| {
            if previous != 0.0 {
                (current - previous) / previous * 100.0
            } else {
                0.0
            }
        });
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
