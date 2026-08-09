use super::operator_states::validate_period;
use crate::error::TaResult;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
/// Rolling ordinary-least-squares hedge ratio of `y` against `x`.
pub struct HedgeRatio {
    values: VecDeque<(f64, f64)>,
    period: usize,
    value: Option<f64>,
}

impl HedgeRatio {
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            values: VecDeque::with_capacity(period),
            period,
            value: None,
        })
    }

    pub fn append(&mut self, x: f64, y: f64) -> Option<f64> {
        if self.values.len() == self.period {
            self.values.pop_front();
        }
        self.values.push_back((x, y));
        self.value = if self.values.len() == self.period {
            let n = self.period as f64;
            let (front, back) = self.values.as_slices();
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;
            for &(x, y) in front {
                sum_x += x;
                sum_y += y;
            }
            for &(x, y) in back {
                sum_x += x;
                sum_y += y;
            }
            let mean_x = sum_x / n;
            let mean_y = sum_y / n;
            let mut covariance = 0.0;
            let mut variance = 0.0;
            for &(x, y) in front {
                let delta_x = x - mean_x;
                covariance += delta_x * (y - mean_y);
                variance += delta_x * delta_x;
            }
            for &(x, y) in back {
                let delta_x = x - mean_x;
                covariance += delta_x * (y - mean_y);
                variance += delta_x * delta_x;
            }
            Some(if variance > 0.0 {
                covariance / variance
            } else {
                0.0
            })
        } else {
            None
        };
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
