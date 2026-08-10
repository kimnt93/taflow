use std::collections::VecDeque;

use crate::error::TaResult;
use crate::stream::{invalid_period, StreamingIndicator};

/// Standard error of the least-squares trend fitted to each rolling window.
#[derive(Debug, Clone)]
pub struct RollingStandardError {
    period: usize,
    values: VecDeque<f64>,
    sum_x: f64,
    denominator: f64,
    sum_y: f64,
    sum_xy: f64,
    sum_y_squared: f64,
    value: Option<f64>,
}

impl RollingStandardError {
    /// Create a regression standard-error state; `period` must be at least three.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 3 {
            return Err(invalid_period("period", period, 3));
        }
        let n = period as f64;
        let sum_x = n * (n - 1.0) * 0.5;
        let sum_x_squared = (n - 1.0) * n * (2.0 * n - 1.0) / 6.0;
        Ok(Self {
            period,
            values: VecDeque::with_capacity(period),
            sum_x,
            denominator: n * sum_x_squared - sum_x * sum_x,
            sum_y: 0.0,
            sum_xy: 0.0,
            sum_y_squared: 0.0,
            value: None,
        })
    }

    /// Append one dependent observation and return regression residual error.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.period {
            let oldest = self.values.pop_front().expect("full regression window");
            self.sum_xy = self.sum_xy - self.sum_y + oldest;
            self.sum_y -= oldest;
            self.sum_y_squared -= oldest * oldest;
        }
        let x = self.values.len() as f64;
        self.values.push_back(input);
        self.sum_y += input;
        self.sum_xy += x * input;
        self.sum_y_squared += input * input;
        self.value = (self.values.len() == self.period).then(|| {
            let n = self.period as f64;
            let slope = (n * self.sum_xy - self.sum_x * self.sum_y) / self.denominator;
            let mean = self.sum_y / n;
            let total = self.sum_y_squared - n * mean * mean;
            let residual = (total - slope * slope * self.denominator / n).max(0.0);
            (residual / (n - 2.0)).sqrt()
        });
        self.value
    }

    /// Return the latest regression standard error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clear observations and running sums.
    pub fn reset(&mut self) {
        self.values.clear();
        self.sum_y = 0.0;
        self.sum_xy = 0.0;
        self.sum_y_squared = 0.0;
        self.value = None;
    }
}

impl StreamingIndicator for RollingStandardError {
    type Output = f64;
    fn append(&mut self, input: f64) -> Option<f64> {
        Self::append(self, input)
    }
    fn value(&self) -> Option<f64> {
        self.value
    }
    fn reset(&mut self) {
        Self::reset(self)
    }
}
