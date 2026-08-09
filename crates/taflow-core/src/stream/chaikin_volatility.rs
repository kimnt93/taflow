//! Persistent Chaikin Volatility state.

use super::operator_states::validate_period;
use crate::error::TaResult;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct ChaikinVolatility {
    period: usize,
    roc_period: usize,
    alpha: f64,
    ema: Option<f64>,
    history: VecDeque<f64>,
    value: Option<f64>,
}

impl ChaikinVolatility {
    pub fn new(period: usize, roc_period: usize) -> TaResult<Self> {
        validate_period(period)?;
        validate_period(roc_period)?;
        Ok(Self {
            period,
            roc_period,
            alpha: 2.0 / (period as f64 + 1.0),
            ema: None,
            history: VecDeque::with_capacity(roc_period + 1),
            value: None,
        })
    }
    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        let range = high - low;
        let ema = self
            .ema
            .map_or(range, |value| value + self.alpha * (range - value));
        self.ema = Some(ema);
        if self.history.len() == self.roc_period + 1 {
            self.history.pop_front();
        }
        self.history.push_back(ema);
        self.value = (self.history.len() == self.roc_period + 1).then(|| {
            let old = self.history.front().copied().unwrap();
            if old != 0.0 {
                (ema - old) / old * 100.0
            } else {
                0.0
            }
        });
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.ema = None;
        self.history.clear();
        self.value = None;
    }
}
