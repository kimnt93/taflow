//! Persistent rolling Sharpe ratio state.

use crate::error::TaResult;
use crate::stream::operator_states::validate_period;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct RollingSharpe {
    values: VecDeque<f64>,
    timeperiod: usize,
    value: Option<f64>,
}

impl RollingSharpe {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self {
            values: VecDeque::with_capacity(timeperiod),
            timeperiod,
            value: None,
        })
    }

    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod {
            self.values.pop_front();
        }
        self.values.push_back(input);
        self.value = (self.values.len() == self.timeperiod).then(|| {
            let average = self.values.iter().sum::<f64>() / self.timeperiod as f64;
            let variance = self
                .values
                .iter()
                .map(|&value| (value - average).powi(2))
                .sum::<f64>()
                / (self.timeperiod - 1) as f64;
            if variance > 0.0 {
                average / variance.sqrt()
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
