//! Stateful Relative Strength Index.

use crate::error::TaResult;
use crate::stream::{invalid_period, StreamingIndicator};

/// Incremental Wilder Relative Strength Index with TA-Lib-compatible warm-up.
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
    /// Creates a state with a smoothing period of at least two changes.
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

    #[inline]
    fn current_value(&self) -> f64 {
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
        let Some(previous_input) = self.previous_input.replace(input) else {
            return None;
        };
        let change = input - previous_input;
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
        self.value = Some(self.current_value());
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

    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        output.reserve(inputs.len());
        for &input in inputs {
            output.push(self.append(input).unwrap_or(f64::NAN));
        }
    }
}
