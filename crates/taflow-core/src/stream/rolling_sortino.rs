//! Persistent rolling Sortino ratio state.

use crate::error::TaResult;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct RollingSortino {
    values: VecDeque<f64>,
    timeperiod: usize,
    value: Option<f64>,
}

impl RollingSortino {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        super::operator_states::validate_period(timeperiod)?;
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
            let downside = self
                .values
                .iter()
                .map(|&value| value.min(0.0).powi(2))
                .sum::<f64>()
                / self.timeperiod as f64;
            if downside > 0.0 {
                average / downside.sqrt()
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
