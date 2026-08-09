use crate::error::{TaError, TaResult};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
/// Causal rolling rescaled-range Hurst estimate.
pub struct Hurst {
    values: VecDeque<f64>,
    period: usize,
    log_period: f64,
    value: Option<f64>,
}

impl Hurst {
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
            log_period: (period as f64).ln(),
            value: None,
        })
    }

    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.period {
            self.values.pop_front();
        }
        self.values.push_back(input);
        self.value = (self.values.len() == self.period).then(|| {
            let n = self.period as f64;
            let (front, back) = self.values.as_slices();
            let mut sum = 0.0;
            for &value in front {
                sum += value;
            }
            for &value in back {
                sum += value;
            }
            let mean = sum / n;
            let mut cumulative = 0.0;
            let mut minimum = f64::INFINITY;
            let mut maximum = f64::NEG_INFINITY;
            let mut squared = 0.0;
            for &value in front.iter().chain(back) {
                let deviation = value - mean;
                cumulative += deviation;
                if cumulative < minimum {
                    minimum = cumulative;
                }
                if cumulative > maximum {
                    maximum = cumulative;
                }
                squared += deviation * deviation;
            }
            let standard_deviation = (squared / n).sqrt();
            let rescaled_range = (maximum - minimum) / standard_deviation;
            if rescaled_range > 0.0 {
                (rescaled_range.ln() / self.log_period).clamp(0.0, 1.0)
            } else {
                0.5
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
