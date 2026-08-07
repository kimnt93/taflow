//! Stateful rolling Fibonacci retracement levels.

use std::collections::VecDeque;
use crate::error::TaResult;

/// Rolling high/low range converted to seven Fibonacci levels.
#[derive(Debug, Clone)]
pub struct FibonacciRetracement {
    period: usize,
    closes: VecDeque<f64>,
    value: Option<[f64; 7]>,
}

impl FibonacciRetracement {
    /// Creates the retracement calculator with a positive rolling window.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 1 { return Err(super::invalid_period("window", period, 1)); }
        Ok(Self { period, closes: VecDeque::with_capacity(period), value: None })
    }

    /// Appends one close and returns levels from 0% through 100%.
    pub fn append(&mut self, close: f64) -> [f64; 7] {
        self.closes.push_back(close);
        if self.closes.len() > self.period { self.closes.pop_front(); }
        let low = self.closes.iter().copied().fold(f64::INFINITY, f64::min);
        let high = self.closes.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let span = high - low;
        let levels = [0.0, 0.236, 0.382, 0.5, 0.618, 0.786, 1.0].map(|ratio| high - span * ratio);
        self.value = Some(levels);
        levels
    }

    /// Returns the latest seven retracement levels.
    pub fn value(&self) -> Option<[f64; 7]> { self.value }

    /// Clears rolling history and levels.
    pub fn reset(&mut self) { self.closes.clear(); self.value = None; }
}
