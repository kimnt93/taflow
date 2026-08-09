//! Persistent Hull moving average state.

use super::operator_states::{validate_period, weighted_mean_slice, ContiguousWindow};
use crate::error::TaResult;

#[derive(Debug, Clone)]
pub struct HullMovingAverage {
    raw: ContiguousWindow,
    intermediate: ContiguousWindow,
    period: usize,
    half: usize,
    value: Option<f64>,
}

impl HullMovingAverage {
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        let half = (period / 2).max(1);
        let smooth = ((period as f64).sqrt().floor() as usize).max(1);
        Ok(Self {
            raw: ContiguousWindow::new(period),
            intermediate: ContiguousWindow::new(smooth),
            period,
            half,
            value: None,
        })
    }

    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.raw.push(input);
        if self.raw.is_full() {
            let window = self.raw.window();
            let half = weighted_mean_slice(&window[self.period - self.half..]);
            let full = weighted_mean_slice(window);
            self.intermediate.push(2.0 * half - full);
            self.value = self
                .intermediate
                .is_full()
                .then(|| weighted_mean_slice(self.intermediate.window()));
        } else {
            self.value = None;
        }
        self.value
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }

    pub fn reset(&mut self) {
        self.raw.clear();
        self.intermediate.clear();
        self.value = None;
    }
}
