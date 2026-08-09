//! Persistent falling-direction state.

use super::operator_states::validate_period;
use crate::error::TaResult;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Falling {
    values: VecDeque<f64>,
    period: usize,
    value: Option<f64>,
}

impl Falling {
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            values: VecDeque::with_capacity(period + 1),
            period,
            value: None,
        })
    }
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.period + 1 {
            self.values.pop_front();
        }
        self.values.push_back(input);
        self.value = (self.values.len() == self.period + 1).then(|| {
            if input < self.values.front().copied().unwrap() {
                1.0
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
        self.values.clear();
        self.value = None;
    }
}
