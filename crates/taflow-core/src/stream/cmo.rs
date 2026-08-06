//! Stateful Chande Momentum Oscillator.
//!
//! CMO separates positive and negative price changes, seeds both Wilder sums
//! from the first `timeperiod` changes, and then updates them in constant time.

use crate::error::TaResult;

use super::{invalid_period, StreamingIndicator};

/// Incremental Chande Momentum Oscillator with TA-Lib-compatible warm-up.
#[derive(Debug, Clone)]
pub struct Cmo {
    period: usize,
    previous_input: Option<f64>,
    changes: usize,
    sum_up: f64,
    sum_down: f64,
    value: Option<f64>,
}

impl Cmo {
    /// Creates a CMO state with a period of at least two changes.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            period,
            previous_input: None,
            changes: 0,
            sum_up: 0.0,
            sum_down: 0.0,
            value: None,
        })
    }

    fn oscillator(&self) -> f64 {
        let total = self.sum_up + self.sum_down;
        if total > 0.0 {
            100.0 * (self.sum_up - self.sum_down) / total
        } else {
            0.0
        }
    }
}

impl StreamingIndicator for Cmo {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        let Some(previous) = self.previous_input.replace(input) else {
            return None;
        };
        let change = input - previous;
        let (up, down) = if change > 0.0 {
            (change, 0.0)
        } else {
            (0.0, -change)
        };
        self.changes += 1;
        if self.changes <= self.period {
            self.sum_up += up;
            self.sum_down += down;
            if self.changes < self.period {
                return None;
            }
        } else {
            let period = self.period as f64;
            self.sum_up = self.sum_up - self.sum_up / period + up;
            self.sum_down = self.sum_down - self.sum_down / period + down;
        }
        self.value = Some(self.oscillator());
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.previous_input = None;
        self.changes = 0;
        self.sum_up = 0.0;
        self.sum_down = 0.0;
        self.value = None;
    }
}
