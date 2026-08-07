//! Causal delay-line state.

use std::collections::VecDeque;

use super::operator_states::validate_period;
use crate::TaResult;

/// Delays a scalar series by a fixed number of bars.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Lag`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Lag {
    values: VecDeque<f64>,
    timeperiod: usize,
    value: Option<f64>,
}

impl Lag {
    /// Creates an empty delay line.
    ///
    /// `timeperiod` is the number of bars before the first value is emitted.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self {
            values: VecDeque::with_capacity(timeperiod),
            timeperiod,
            value: None,
        })
    }

    /// Appends one observation and returns the value from `timeperiod` bars ago.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.value = if self.values.len() == self.timeperiod {
            let value = self.values.pop_front().expect("lag window is full");
            self.values.push_back(input);
            Some(value)
        } else {
            self.values.push_back(input);
            None
        };
        self.value
    }

    /// Returns the latest delayed value, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clears buffered observations and the latest value.
    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}

/// Compute the lag result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn lag(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_period(timeperiod)?;
    let mut output = vec![f64::NAN; input.len()];
    for index in timeperiod..input.len() {
        output[index] = input[index - timeperiod];
    }
    Ok(output)
}
