use crate::error::{TaError, TaResult};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
/// Rolling lag-one Pearson autocorrelation.
pub struct RollingAutocorr {
    values: VecDeque<f64>,
    period: usize,
    scratch: Box<[f64]>,
    value: Option<f64>,
}

impl RollingAutocorr {
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: period.to_string(),
                reason: "must be >= 2",
            });
        }
        Ok(Self {
            values: VecDeque::with_capacity(period),
            period,
            scratch: vec![0.0; period].into_boxed_slice(),
            value: None,
        })
    }

    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.period {
            self.values.pop_front();
        }
        self.values.push_back(input);
        if self.values.len() < self.period {
            self.value = None;
            return None;
        }
        let (front, back) = self.values.as_slices();
        self.scratch[..front.len()].copy_from_slice(front);
        self.scratch[front.len()..].copy_from_slice(back);
        let window = &self.scratch[..];
        let mean = window.iter().sum::<f64>() / self.period as f64;
        let mut denominator = 0.0;
        let mut numerator = 0.0;
        for &value in window {
            let deviation = value - mean;
            denominator += deviation * deviation;
        }
        for index in 0..self.period - 1 {
            numerator += (window[index] - mean) * (window[index + 1] - mean);
        }
        self.value = Some(if denominator == 0.0 {
            0.0
        } else {
            numerator / denominator
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
