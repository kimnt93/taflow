//! Stateful Relative Strength Index.
//!
//! RSI retains Wilder's average gain and loss recurrence and uses TA-Lib's
//! operation order and epsilon rule for deterministic composite indicators.

use crate::error::TaResult;

use super::{invalid_period, StreamingIndicator};

/// Incremental Wilder RSI with TA-Lib-compatible warm-up and rounding.
#[derive(Debug, Clone)]
pub struct RelativeStrengthIndex {
    period: usize,
    previous_input: Option<f64>,
    changes: usize,
    gain_sum: f64,
    loss_sum: f64,
    average_gain: f64,
    average_loss: f64,
    value: Option<f64>,
}

impl RelativeStrengthIndex {
    /// Creates an RSI state with a period of at least two bars.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            period,
            previous_input: None,
            changes: 0,
            gain_sum: 0.0,
            loss_sum: 0.0,
            average_gain: 0.0,
            average_loss: 0.0,
            value: None,
        })
    }

    fn rsi_value(&self) -> f64 {
        let sum = self.average_gain + self.average_loss;
        if sum.abs() < 1.0e-14 {
            0.0
        } else {
            100.0 * (self.average_gain / sum)
        }
    }
}

impl StreamingIndicator for RelativeStrengthIndex {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        let Some(previous) = self.previous_input.replace(input) else {
            return None;
        };
        let change = input - previous;
        let (gain, loss) = if change > 0.0 {
            (change, 0.0)
        } else {
            (0.0, -change)
        };
        self.changes += 1;
        if self.changes < self.period {
            self.gain_sum += gain;
            self.loss_sum += loss;
            return None;
        }
        if self.changes == self.period {
            self.average_gain = (self.gain_sum + gain) / self.period as f64;
            self.average_loss = (self.loss_sum + loss) / self.period as f64;
        } else {
            let period = self.period as f64;
            self.average_gain = (self.average_gain * (period - 1.0) + gain) / period;
            self.average_loss = (self.average_loss * (period - 1.0) + loss) / period;
        }
        self.value = Some(self.rsi_value());
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.previous_input = None;
        self.changes = 0;
        self.gain_sum = 0.0;
        self.loss_sum = 0.0;
        self.average_gain = 0.0;
        self.average_loss = 0.0;
        self.value = None;
    }
}
