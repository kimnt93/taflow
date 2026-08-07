//! Stateful Parabolic Moving Average Stop.

use std::collections::VecDeque;
use crate::error::TaResult;

/// EMA plus rolling-true-range stop with causal trend direction.
#[derive(Debug, Clone)]
pub struct ParabolicMovingAverageStop {
    period: usize,
    multiplier: f64,
    previous_close: Option<f64>,
    ema: Option<f64>,
    true_ranges: VecDeque<f64>,
    upper: Option<f64>,
    lower: Option<f64>,
    trend: i32,
    value: Option<(f64, i32)>,
}

impl ParabolicMovingAverageStop {
    /// Creates the stop with a positive EMA/ATR period.
    pub fn new(period: usize, multiplier: f64) -> TaResult<Self> {
        if period < 1 { return Err(super::invalid_period("length", period, 1)); }
        Ok(Self { period, multiplier, previous_close: None, ema: None, true_ranges: VecDeque::with_capacity(period), upper: None, lower: None, trend: 1, value: None })
    }

    /// Appends one high/low/close bar and returns stop and trend direction.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> (f64, i32) {
        let previous = self.previous_close.unwrap_or(close);
        let true_range = (high - low).max((high - previous).abs()).max((low - previous).abs());
        self.previous_close = Some(close);
        self.true_ranges.push_back(true_range);
        if self.true_ranges.len() > self.period { self.true_ranges.pop_front(); }
        let alpha = 2.0 / (self.period as f64 + 1.0);
        let ema = *self.ema.get_or_insert(close);
        self.ema = Some(ema + alpha * (close - ema));
        let average_true_range = self.true_ranges.iter().sum::<f64>() / self.true_ranges.len() as f64;
        let upper = self.ema.unwrap() + self.multiplier * average_true_range;
        let lower = self.ema.unwrap() - self.multiplier * average_true_range;
        self.upper = Some(match self.upper { None => upper, Some(previous) if self.trend < 0 => upper.min(previous), Some(_) => upper });
        self.lower = Some(match self.lower { None => lower, Some(previous) if self.trend > 0 => lower.max(previous), Some(_) => lower });
        if self.trend > 0 && close < self.lower.unwrap() { self.trend = -1; }
        else if self.trend < 0 && close > self.upper.unwrap() { self.trend = 1; }
        let result = (if self.trend > 0 { self.lower.unwrap() } else { self.upper.unwrap() }, self.trend);
        self.value = Some(result); result
    }

    /// Returns the latest stop and trend pair.
    pub fn value(&self) -> Option<(f64, i32)> { self.value }
    /// Clears EMA, range, and trend state.
    pub fn reset(&mut self) { self.previous_close = None; self.ema = None; self.true_ranges.clear(); self.upper = None; self.lower = None; self.trend = 1; self.value = None; }
}
