use crate::error::{TaError, TaResult};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct RollingTreynorRatio {
    values: VecDeque<(f64, f64)>,
    period: usize,
    value: Option<f64>,
}
impl RollingTreynorRatio {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        if timeperiod == 0 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: "0".into(),
                reason: "must be >= 1",
            });
        }
        Ok(Self {
            values: VecDeque::with_capacity(timeperiod),
            period: timeperiod,
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
            let mean_x = self.values.iter().map(|x| x.0).sum::<f64>() / n;
            let mean_y = self.values.iter().map(|x| x.1).sum::<f64>() / n;
            let cov = self
                .values
                .iter()
                .map(|x| (x.0 - mean_x) * (x.1 - mean_y))
                .sum::<f64>()
                / n;
            let var = self
                .values
                .iter()
                .map(|x| (x.1 - mean_y).powi(2))
                .sum::<f64>()
                / n;
            if var > 0.0 {
                mean_x / (cov / var)
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
