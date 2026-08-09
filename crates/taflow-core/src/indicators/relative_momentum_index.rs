//! Stateful Relative Momentum Index.

use std::collections::VecDeque;

use crate::error::TaResult;
use crate::stream::{invalid_period, StreamingIndicator};

/// Computes Relative Momentum Index using Wilder-smoothed momentum gains.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RelativeMomentumIndex`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RelativeMomentumIndex {
    period: usize,
    momentum: usize,
    prices: VecDeque<f64>,
    count: usize,
    up: f64,
    down: f64,
    value: Option<f64>,
}

impl RelativeMomentumIndex {
    /// Creates an RMI with a positive smoothing period and momentum lag.
    pub fn new(period: usize, momentum: usize) -> TaResult<Self> {
        if period < 1 {
            return Err(invalid_period("timeperiod", period, 1));
        }
        if momentum < 1 {
            return Err(invalid_period("momentum", momentum, 1));
        }
        Ok(Self {
            period,
            momentum,
            prices: VecDeque::with_capacity(momentum + 1),
            count: 0,
            up: 0.0,
            down: 0.0,
            value: None,
        })
    }
}

impl StreamingIndicator for RelativeMomentumIndex {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        self.prices.push_back(input);
        if self.prices.len() <= self.momentum {
            return None;
        }
        let previous = self.prices.pop_front().expect("momentum history exists");
        let change = input - previous;
        let gain = change.max(0.0);
        let loss = (-change).max(0.0);
        self.count += 1;
        if self.count <= self.period {
            self.up += gain;
            self.down += loss;
            if self.count < self.period {
                return None;
            }
            let period = self.period as f64;
            self.up /= period;
            self.down /= period;
        } else {
            let period = self.period as f64;
            self.up = (self.up * (period - 1.0) + gain) / period;
            self.down = (self.down * (period - 1.0) + loss) / period;
        }
        let total = self.up + self.down;
        self.value = Some(if total == 0.0 {
            50.0
        } else {
            100.0 * self.up / total
        });
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.prices.clear();
        self.count = 0;
        self.up = 0.0;
        self.down = 0.0;
        self.value = None;
    }

    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        output.reserve(inputs.len());
        for &input in inputs {
            output.push(self.append(input).unwrap_or(f64::NAN));
        }
    }
}
