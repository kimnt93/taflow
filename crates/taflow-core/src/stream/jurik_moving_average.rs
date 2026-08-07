//! Stateful Jurik-like adaptive moving average reconstruction.

use std::collections::VecDeque;
use crate::error::TaResult;
use super::{invalid_period, StreamingIndicator};

/// Computes the documented adaptive Jurik-like moving-average recurrence.
#[derive(Debug, Clone)]
pub struct JurikMovingAverage {
    period: usize,
    _phase: f64,
    closes: VecDeque<f64>,
    value: Option<f64>,
}

impl JurikMovingAverage {
    /// Creates the adaptive average from a positive length and phase value.
    pub fn new(period: usize, phase: f64) -> TaResult<Self> {
        if period < 1 { return Err(invalid_period("length", period, 1)); }
        Ok(Self { period, _phase: phase, closes: VecDeque::with_capacity(period + 1), value: None })
    }
}

impl StreamingIndicator for JurikMovingAverage {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        self.closes.push_back(input);
        if self.closes.len() > self.period + 1 { self.closes.pop_front(); }
        if self.value.is_none() {
            self.value = Some(input);
            return self.value;
        }
        let mut volatility = 0.0;
        for (previous_close, current_close) in self.closes.iter().zip(self.closes.iter().skip(1)) {
            volatility += (current_close - previous_close).abs();
        }
        let count = self.closes.len().saturating_sub(1).max(1) as f64;
        volatility /= count;
        let previous = self.value.expect("initialized above");
        let base = 2.0 / (self.period as f64 + 1.0);
        let deviation = (input - previous).abs();
        let adaptive = (base * (1.0 + (deviation / (volatility + 1.0e-12)).min(1.0))).min(1.0);
        self.value = Some(previous + adaptive * (input - previous));
        self.value
    }

    fn value(&self) -> Option<f64> { self.value }
    fn reset(&mut self) { self.closes.clear(); self.value = None; }
}
