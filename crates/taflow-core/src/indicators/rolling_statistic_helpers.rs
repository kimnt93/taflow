use std::collections::VecDeque;

use crate::error::TaResult;
use crate::stream::operator_states::validate_period;

#[derive(Debug, Clone)]
pub(crate) struct RollingValues {
    values: VecDeque<f64>,
    period: usize,
}

impl RollingValues {
    pub(crate) fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            values: VecDeque::with_capacity(period),
            period,
        })
    }

    pub(crate) fn push(&mut self, value: f64) {
        if self.values.len() == self.period {
            self.values.pop_front();
        }
        self.values.push_back(value);
    }

    pub(crate) fn is_full(&self) -> bool {
        self.values.len() == self.period
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &f64> {
        self.values.iter()
    }

    pub(crate) fn window(&self) -> &VecDeque<f64> {
        &self.values
    }

    pub(crate) fn clear(&mut self) {
        self.values.clear();
    }
}

pub(crate) fn quantile(values: &VecDeque<f64>, probability: f64) -> f64 {
    let mut sorted: Vec<f64> = values.iter().copied().collect();
    sorted.sort_by(f64::total_cmp);
    let position = probability * (sorted.len().saturating_sub(1) as f64);
    let lower = position.floor() as usize;
    let upper = position.ceil() as usize;
    sorted[lower] + (sorted[upper] - sorted[lower]) * (position - lower as f64)
}
