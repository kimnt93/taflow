use crate::error::{TaError, TaResult};
use std::collections::VecDeque;

/// Rolling mean asset return divided by covariance-derived benchmark beta.
#[derive(Debug, Clone)]
pub struct RollingTreynorRatio {
    values: VecDeque<(f64, f64)>,
    period: usize,
    sum_asset: f64,
    sum_benchmark: f64,
    sum_benchmark_squared: f64,
    sum_cross_product: f64,
    value: Option<f64>,
}
impl RollingTreynorRatio {
    /// Creates the ratio with a positive rolling period.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        if timeperiod < 2 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: "0".into(),
                reason: "must be >= 2",
            });
        }
        Ok(Self {
            values: VecDeque::with_capacity(timeperiod),
            period: timeperiod,
            sum_asset: 0.0,
            sum_benchmark: 0.0,
            sum_benchmark_squared: 0.0,
            sum_cross_product: 0.0,
            value: None,
        })
    }
    /// Appends one aligned asset/benchmark return pair.
    pub fn append(&mut self, input: f64, benchmark: f64) -> Option<f64> {
        if !input.is_finite() || !benchmark.is_finite() {
            return None;
        }
        if self.values.len() == self.period {
            let (old_input, old_benchmark) = self.values.pop_front().expect("window is full");
            self.sum_asset -= old_input;
            self.sum_benchmark -= old_benchmark;
            self.sum_benchmark_squared -= old_benchmark * old_benchmark;
            self.sum_cross_product -= old_input * old_benchmark;
        }
        self.values.push_back((input, benchmark));
        self.sum_asset += input;
        self.sum_benchmark += benchmark;
        self.sum_benchmark_squared += benchmark * benchmark;
        self.sum_cross_product += input * benchmark;
        self.value = (self.values.len() == self.period).then(|| {
            let n = self.period as f64;
            let mean_asset = self.sum_asset / n;
            let mean_benchmark = self.sum_benchmark / n;
            let variance = self.sum_benchmark_squared / n - mean_benchmark * mean_benchmark;
            if variance <= 0.0 {
                return 0.0;
            }
            let covariance = self.sum_cross_product / n - mean_asset * mean_benchmark;
            let beta = covariance / variance;
            if beta == 0.0 {
                0.0
            } else {
                mean_asset / beta
            }
        });
        self.value
    }
    /// Returns the latest ratio, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Clears retained pairs and the latest ratio.
    pub fn reset(&mut self) {
        self.values.clear();
        self.sum_asset = 0.0;
        self.sum_benchmark = 0.0;
        self.sum_benchmark_squared = 0.0;
        self.sum_cross_product = 0.0;
        self.value = None;
    }
}
