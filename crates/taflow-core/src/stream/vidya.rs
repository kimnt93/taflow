//! Stateful Variable Index Dynamic Average.

use std::collections::VecDeque;

use crate::error::{TaError, TaResult};
use super::StreamingIndicator;

/// CMO-modulated exponential average with causal warm-up from the first bar.
#[derive(Debug, Clone)]
pub struct VariableIndexDynamicAverage {
    period: usize,
    alpha: f64,
    closes: VecDeque<f64>,
    value: Option<f64>,
}

impl VariableIndexDynamicAverage {
    /// Creates VIDYA from a positive period and alpha in `(0, 1]`.
    pub fn new(period: usize, alpha: f64) -> TaResult<Self> {
        if period < 1 {
            return Err(TaError::InvalidParameter { name: "length", value: period.to_string(), reason: "must be positive" });
        }
        if !(0.0..=1.0).contains(&alpha) || alpha == 0.0 {
            return Err(TaError::InvalidParameter { name: "alpha", value: alpha.to_string(), reason: "must be in (0, 1]" });
        }
        Ok(Self { period, alpha, closes: VecDeque::with_capacity(period + 1), value: None })
    }
}

impl StreamingIndicator for VariableIndexDynamicAverage {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        self.closes.push_back(input);
        if self.closes.len() > self.period + 1 {
            self.closes.pop_front();
        }
        if self.value.is_none() {
            self.value = Some(input);
            return self.value;
        }
        let mut up = 0.0;
        let mut down = 0.0;
        let mut previous = self.closes.front().copied().unwrap_or(input);
        for &close in self.closes.iter().skip(1) {
            let change = close - previous;
            if change > 0.0 { up += change; } else { down -= change; }
            previous = close;
        }
        let total = up + down;
        let weight = if total == 0.0 { 0.0 } else { (up - down).abs() / total };
        let previous_value = self.value.expect("initialized above");
        self.value = Some(self.alpha * weight * input + (1.0 - self.alpha * weight) * previous_value);
        self.value
    }

    fn value(&self) -> Option<f64> { self.value }

    fn reset(&mut self) { self.closes.clear(); self.value = None; }
}
