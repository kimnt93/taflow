use crate::error::{TaError, TaResult};
use crate::stream::StreamingIndicator;
use std::collections::VecDeque;

/// Causal finite-window approximation of an empirical-mode residual.
#[derive(Debug, Clone)]
pub struct EmpiricalModeDecomposition {
    period: usize,
    fraction: f64,
    values: VecDeque<f64>,
    sum: f64,
    value: Option<f64>,
}

impl EmpiricalModeDecomposition {
    pub fn new(period: usize, fraction: f64) -> TaResult<Self> {
        if period == 0 {
            return Err(TaError::InvalidParameter {
                name: "period",
                value: period.to_string(),
                reason: "must be positive",
            });
        }
        if !(0.0..=1.0).contains(&fraction) {
            return Err(TaError::InvalidParameter {
                name: "fraction",
                value: fraction.to_string(),
                reason: "must be between zero and one",
            });
        }
        Ok(Self {
            period,
            fraction,
            values: VecDeque::with_capacity(period),
            sum: 0.0,
            value: None,
        })
    }
    pub fn append(&mut self, price: f64) -> Option<f64> {
        if self.values.len() == self.period {
            self.sum -= self.values.pop_front().expect("full window");
        }
        self.values.push_back(price);
        self.sum += price;
        self.value = (self.values.len() == self.period)
            .then(|| price - self.fraction * self.sum / self.period as f64);
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.values.clear();
        self.sum = 0.0;
        self.value = None;
    }
}
impl StreamingIndicator for EmpiricalModeDecomposition {
    type Output = f64;
    fn append(&mut self, value: f64) -> Option<f64> {
        Self::append(self, value)
    }
    fn value(&self) -> Option<f64> {
        Self::value(self)
    }
    fn reset(&mut self) {
        Self::reset(self)
    }
}
