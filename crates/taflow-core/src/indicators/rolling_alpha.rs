use crate::error::TaResult;
use crate::stream::operator_states::validate_period;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
/// Rolling regression alpha of an input series against a benchmark.
pub struct RollingAlpha {
    values: VecDeque<(f64, f64)>,
    period: usize,
    value: Option<f64>,
}

impl RollingAlpha {
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            values: VecDeque::with_capacity(period),
            period,
            value: None,
        })
    }

    pub fn append(&mut self, input: f64, benchmark: f64) -> Option<f64> {
        if self.values.len() == self.period {
            self.values.pop_front();
        }
        self.values.push_back((input, benchmark));
        self.value = (self.values.len() == self.period).then(|| {
            let n = self.period as f64;
            let (front, back) = self.values.as_slices();
            let mut sum_input = 0.0;
            let mut sum_benchmark = 0.0;
            for &(input, benchmark) in front {
                sum_input += input;
                sum_benchmark += benchmark;
            }
            for &(input, benchmark) in back {
                sum_input += input;
                sum_benchmark += benchmark;
            }
            let mean_input = sum_input / n;
            let mean_benchmark = sum_benchmark / n;
            let mut covariance = 0.0;
            let mut variance = 0.0;
            for &(input, benchmark) in front {
                let delta_benchmark = benchmark - mean_benchmark;
                covariance += (input - mean_input) * delta_benchmark;
                variance += delta_benchmark * delta_benchmark;
            }
            for &(input, benchmark) in back {
                let delta_benchmark = benchmark - mean_benchmark;
                covariance += (input - mean_input) * delta_benchmark;
                variance += delta_benchmark * delta_benchmark;
            }
            let beta = if variance > 0.0 {
                covariance / variance
            } else {
                0.0
            };
            mean_input - beta * mean_benchmark
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
