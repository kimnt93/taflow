//! Rolling mode state.

use std::collections::VecDeque;

use crate::TaResult;
use super::operators::validate_period;

/// Computes the causal most-frequent value over a fixed trailing window.
#[derive(Debug, Clone)]
pub struct RollingMode {
    values: VecDeque<f64>,
    timeperiod: usize,
    value: Option<f64>,
}

impl RollingMode {
    /// Creates an empty rolling-mode state.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self { values: VecDeque::with_capacity(timeperiod), timeperiod, value: None })
    }

    /// Appends one observation and returns the mode after warm-up.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod { self.values.pop_front(); }
        self.values.push_back(input);
        self.value = if self.values.len() == self.timeperiod {
            let mut best = self.values[0];
            let mut best_count = 0;
            for &candidate in &self.values {
                let count = self.values.iter().filter(|&&value| value == candidate).count();
                if count > best_count { best = candidate; best_count = count; }
            }
            Some(best)
        } else { None };
        self.value
    }

    /// Returns the latest mode, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> { self.value }

    /// Clears the trailing window and latest output.
    pub fn reset(&mut self) { self.values.clear(); self.value = None; }
}
