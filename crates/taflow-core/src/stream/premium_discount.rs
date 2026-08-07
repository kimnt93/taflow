//! Stateful premium/discount zones relative to a rolling midpoint.

use std::collections::VecDeque;
use crate::error::TaResult;

/// Rolling midpoint and signed premium/discount zone.
#[derive(Debug, Clone)]
pub struct PremiumDiscount {
    period: usize,
    closes: VecDeque<f64>,
    value: Option<(i32, f64)>,
}

impl PremiumDiscount {
    /// Creates the indicator with a positive rolling window.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 1 { return Err(super::invalid_period("window", period, 1)); }
        Ok(Self { period, closes: VecDeque::with_capacity(period), value: None })
    }

    /// Appends one close and returns `(zone, equilibrium)`.
    pub fn append(&mut self, close: f64) -> (i32, f64) {
        self.closes.push_back(close);
        if self.closes.len() > self.period { self.closes.pop_front(); }
        let low = self.closes.iter().copied().fold(f64::INFINITY, f64::min);
        let high = self.closes.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let equilibrium = (high + low) / 2.0;
        let zone = if close > equilibrium { 1 } else if close < equilibrium { -1 } else { 0 };
        self.value = Some((zone, equilibrium));
        (zone, equilibrium)
    }

    /// Returns the latest zone and equilibrium.
    pub fn value(&self) -> Option<(i32, f64)> { self.value }

    /// Clears all rolling history.
    pub fn reset(&mut self) { self.closes.clear(); self.value = None; }
}
