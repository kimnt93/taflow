//! Stateful Kaufman Adaptive Moving Average.
//!
//! KAMA adapts its smoothing constant from the ratio between net direction
//! and total absolute movement over the configured lookback window.

use std::collections::VecDeque;

use crate::error::TaResult;

use super::{invalid_period, StreamingIndicator};

/// Incremental KAMA with the same seed and recurrence as TA-Lib.
#[derive(Debug, Clone)]
pub struct Kama {
    period: usize,
    prices: VecDeque<f64>,
    changes: VecDeque<f64>,
    volatility: f64,
    previous_kama: Option<f64>,
    value: Option<f64>,
}

impl Kama {
    /// Creates a KAMA state with a positive period.
    pub fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("timeperiod", period, 1));
        }
        Ok(Self {
            period,
            prices: VecDeque::with_capacity(period + 1),
            changes: VecDeque::with_capacity(period),
            volatility: 0.0,
            previous_kama: None,
            value: None,
        })
    }
}

impl StreamingIndicator for Kama {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        if self.period == 1 {
            self.value = Some(input);
            return self.value;
        }
        if let Some(previous) = self.prices.back().copied() {
            let change = (input - previous).abs();
            if self.changes.len() == self.period {
                let old = self.changes.pop_front().expect("change window is full");
                self.volatility -= old;
                self.volatility += change;
            } else {
                self.volatility += change;
            }
            self.changes.push_back(change);
        }
        if self.prices.len() == self.period + 1 {
            self.prices.pop_front();
        }
        self.prices.push_back(input);
        if self.prices.len() < self.period + 1 {
            return None;
        }

        let oldest = *self.prices.front().expect("full price window has a front");
        let direction = input - oldest;
        let efficiency = if self.volatility <= direction || self.volatility.abs() < 1.0e-14 {
            1.0
        } else {
            (direction / self.volatility).abs()
        };
        let slow = 2.0 / 31.0;
        let smoothing = efficiency.mul_add(2.0 / 3.0 - slow, slow);
        let previous = self
            .previous_kama
            .unwrap_or_else(|| self.prices[self.period - 1]);
        let next = (input - previous).mul_add(smoothing * smoothing, previous);
        self.previous_kama = Some(next);
        self.value = Some(next);
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.prices.clear();
        self.changes.clear();
        self.volatility = 0.0;
        self.previous_kama = None;
        self.value = None;
    }
}
