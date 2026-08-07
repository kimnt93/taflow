//! Rolling median state.

use std::collections::VecDeque;

use crate::TaResult;
use super::operator_states::validate_period;

/// Computes the causal median over a fixed trailing window.
#[derive(Debug, Clone)]
pub struct RollingMedian {
    values: VecDeque<f64>,
    timeperiod: usize,
    value: Option<f64>,
}

impl RollingMedian {
    /// Creates an empty rolling-median state.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self { values: VecDeque::with_capacity(timeperiod), timeperiod, value: None })
    }

    /// Appends one observation and returns the median after warm-up.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod { self.values.pop_front(); }
        self.values.push_back(input);
        self.value = if self.values.len() == self.timeperiod {
            let mut sorted: Vec<f64> = self.values.iter().copied().collect();
            sorted.sort_by(f64::total_cmp);
            let middle = self.timeperiod / 2;
            Some(if self.timeperiod % 2 == 1 { sorted[middle] } else { (sorted[middle - 1] + sorted[middle]) * 0.5 })
        } else { None };
        self.value
    }

    /// Returns the latest median, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> { self.value }

    /// Clears the trailing window and latest output.
    pub fn reset(&mut self) { self.values.clear(); self.value = None; }
}
