//! Stateful Even Better Sinewave-style detrended oscillator.

use std::collections::VecDeque;

use crate::error::TaResult;
use super::{invalid_period, StreamingIndicator};

/// Computes a causal detrended cycle value from close prices.
#[derive(Debug, Clone)]
pub struct EvenBetterSinewave {
    _period: usize,
    closes: VecDeque<f64>,
    previous_high_pass: f64,
    previous_value: f64,
    value: Option<f64>,
}

impl EvenBetterSinewave {
    /// Creates the oscillator with a positive nominal cycle period.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 1 { return Err(invalid_period("length", period, 1)); }
        Ok(Self { _period: period, closes: VecDeque::with_capacity(3), previous_high_pass: 0.0, previous_value: 0.0, value: None })
    }
}

impl StreamingIndicator for EvenBetterSinewave {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        self.closes.push_back(input);
        if self.closes.len() > 3 { self.closes.pop_front(); }
        let high_pass = if self.closes.len() < 3 {
            0.0
        } else {
            0.25 * (input - 2.0 * self.closes[1] + self.closes[0]) + self.previous_high_pass
        };
        let output = if self.value.is_none() { high_pass } else { 0.5 * high_pass + 0.5 * self.previous_value };
        self.previous_high_pass = high_pass;
        self.previous_value = output;
        self.value = Some(output);
        self.value
    }

    fn value(&self) -> Option<f64> { self.value }
    fn reset(&mut self) { self.closes.clear(); self.previous_high_pass = 0.0; self.previous_value = 0.0; self.value = None; }
}
