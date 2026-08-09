//! Persistent zero-lag exponential moving average state.

use super::operator_states::validate_period;
use crate::error::TaResult;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `ZeroLagExponentialMovingAverage`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct ZeroLagExponentialMovingAverage {
    values: VecDeque<f64>,
    period: usize,
    lag: usize,
    alpha: f64,
    adjusted_count: usize,
    adjusted_sum: f64,
    ema: Option<f64>,
    value: Option<f64>,
}

impl ZeroLagExponentialMovingAverage {
    /// Create a new empty state.
    ///
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            values: VecDeque::with_capacity((period / 2).max(1) + 1),
            period,
            lag: (period - 1) / 2,
            alpha: 2.0 / (period as f64 + 1.0),
            adjusted_count: 0,
            adjusted_sum: 0.0,
            ema: None,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.values.push_back(input);
        if self.values.len() > self.lag + 1 {
            self.values.pop_front();
        }
        if self.values.len() <= self.lag {
            self.value = None
        } else {
            let lagged = self.values.front().copied().unwrap_or(input);
            let adjusted = 2.0 * input - lagged;
            self.adjusted_count += 1;
            self.ema = if self.adjusted_count < self.period {
                self.adjusted_sum += adjusted;
                None
            } else if self.adjusted_count == self.period {
                self.adjusted_sum += adjusted;
                Some(self.adjusted_sum / self.period as f64)
            } else {
                self.ema
                    .map(|previous| previous + self.alpha * (adjusted - previous))
            };
            self.value = self.ema;
        }
        self.value
    }
    /// Return the latest computed result, if warm-up is complete.
    ///
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Reset the state and clear its accumulated history.
    ///
    pub fn reset(&mut self) {
        self.values.clear();
        self.adjusted_count = 0;
        self.adjusted_sum = 0.0;
        self.ema = None;
        self.value = None;
    }
}
