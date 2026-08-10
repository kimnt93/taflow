//! Stateful Variable Index Dynamic Average.

use crate::error::{TaError, TaResult};
use crate::stream::StreamingIndicator;

/// Chande Momentum Oscillator-modulated exponential moving average.
#[derive(Debug, Clone)]
pub struct VariableIndexDynamicAverage {
    cmo_period: usize,
    alpha: f64,
    changes: Box<[f64]>,
    head: usize,
    change_count: usize,
    upward_change_sum: f64,
    downward_change_sum: f64,
    previous_close: Option<f64>,
    value: Option<f64>,
}

impl VariableIndexDynamicAverage {
    /// Creates a state from positive average and CMO periods.
    pub fn new(length: usize, cmo_period: usize, alpha: f64) -> TaResult<Self> {
        if length < 1 {
            return Err(TaError::InvalidParameter {
                name: "length",
                value: length.to_string(),
                reason: "must be positive",
            });
        }
        if cmo_period < 1 {
            return Err(TaError::InvalidParameter {
                name: "cmo_period",
                value: cmo_period.to_string(),
                reason: "must be positive",
            });
        }
        if !(0.0..=1.0).contains(&alpha) || alpha == 0.0 {
            return Err(TaError::InvalidParameter {
                name: "alpha",
                value: alpha.to_string(),
                reason: "must be in (0, 1]",
            });
        }
        Ok(Self {
            cmo_period,
            alpha,
            changes: vec![0.0; cmo_period].into_boxed_slice(),
            head: 0,
            change_count: 0,
            upward_change_sum: 0.0,
            downward_change_sum: 0.0,
            previous_close: None,
            value: None,
        })
    }

    #[inline]
    fn push_change(&mut self, change: f64) {
        let capacity = self.changes.len();
        if self.change_count < capacity {
            self.changes[self.change_count] = change;
            self.change_count += 1;
        } else {
            let previous_change = self.changes[self.head];
            if previous_change > 0.0 {
                self.upward_change_sum -= previous_change;
            } else {
                self.downward_change_sum += previous_change;
            }
            self.changes[self.head] = change;
            self.head += 1;
            if self.head == capacity {
                self.head = 0;
            }
        }
        if change > 0.0 {
            self.upward_change_sum += change;
        } else {
            self.downward_change_sum -= change;
        }
    }

    #[inline]
    fn momentum_weight(upward_change_sum: f64, downward_change_sum: f64) -> f64 {
        let total = upward_change_sum + downward_change_sum;
        if total == 0.0 {
            0.0
        } else {
            (upward_change_sum - downward_change_sum).abs() / total
        }
    }
}

impl StreamingIndicator for VariableIndexDynamicAverage {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        let Some(previous_close) = self.previous_close.replace(input) else {
            return None;
        };
        self.push_change(input - previous_close);
        if self.change_count < self.cmo_period {
            return None;
        }

        let momentum_weight =
            Self::momentum_weight(self.upward_change_sum, self.downward_change_sum);
        let smoothing_weight = self.alpha * momentum_weight;
        let previous_value = self.value.unwrap_or(input);
        self.value = Some(smoothing_weight * input + (1.0 - smoothing_weight) * previous_value);
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.changes.fill(0.0);
        self.head = 0;
        self.change_count = 0;
        self.upward_change_sum = 0.0;
        self.downward_change_sum = 0.0;
        self.previous_close = None;
        self.value = None;
    }

    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        output.reserve(inputs.len());
        for &input in inputs {
            output.push(self.append(input).unwrap_or(f64::NAN));
        }
    }
}
