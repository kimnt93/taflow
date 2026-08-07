//! Causal delay-line state.

use std::collections::VecDeque;

use crate::TaResult;
use super::operators::validate_period;

/// Delays a scalar series by a fixed number of bars.
#[derive(Debug, Clone)]
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
