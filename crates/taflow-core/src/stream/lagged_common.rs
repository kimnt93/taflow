//! Private shared state for lagged stream indicators.

use std::collections::VecDeque;

use crate::error::TaResult;

use super::invalid_period;

#[derive(Debug, Clone)]
pub(super) struct LaggedValue {
    period: usize,
    values: VecDeque<f64>,
}

impl LaggedValue {
    pub(super) fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("timeperiod", period, 1));
        }
        Ok(Self {
            period,
            values: VecDeque::with_capacity(period),
        })
    }

    pub(super) fn append(&mut self, input: f64) -> Option<(f64, f64)> {
        if self.values.len() < self.period {
            self.values.push_back(input);
            return None;
        }
        let previous = self.values.pop_front().expect("lag window is full");
        self.values.push_back(input);
        Some((input, previous))
    }

    pub(super) fn reset(&mut self) {
        self.values.clear();
    }
}

pub(super) fn validate_rate_of_change(input: &[f64], timeperiod: usize) -> TaResult<()> {
    if timeperiod == 0 {
        return Err(crate::TaError::InvalidParameter {
            name: "timeperiod",
            value: "0".to_string(),
            reason: "must be >= 1",
        });
    }
    if input.len() <= timeperiod {
        return Err(crate::TaError::InsufficientData {
            need: timeperiod + 1,
            got: input.len(),
        });
    }
    Ok(())
}
