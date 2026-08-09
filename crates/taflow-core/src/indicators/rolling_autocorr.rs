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
        let left = &window[..self.period - 1];
        let right = &window[1..];
        let n = (self.period - 1) as f64;
        let mut left_sum = 0.0;
        let mut right_sum = 0.0;
        for &value in left {
            left_sum += value;
        }
        for &value in right {
            right_sum += value;
        }
        let left_mean = left_sum / n;
        let right_mean = right_sum / n;
        let mut left_variance = 0.0;
        let mut right_variance = 0.0;
        let mut covariance = 0.0;
        for index in 0..self.period - 1 {
            let left_delta = left[index] - left_mean;
            let right_delta = right[index] - right_mean;
            left_variance += left_delta * left_delta;
            right_variance += right_delta * right_delta;
            covariance += left_delta * right_delta;
        }
        self.value = Some(if left_variance == 0.0 || right_variance == 0.0 {
            0.0
        } else {
            covariance / (left_variance * right_variance).sqrt()
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
