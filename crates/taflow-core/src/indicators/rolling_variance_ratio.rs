use crate::error::TaResult;
use crate::stream::invalid_period;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct RollingVarianceRatio {
    period: usize,
    q: usize,
    pairs: VecDeque<(f64, f64)>,
    value: Option<f64>,
}
impl RollingVarianceRatio {
    pub fn new(period: usize, q: usize) -> TaResult<Self> {
        if q == 0 {
            return Err(invalid_period("q", q, 1));
        }
        Ok(Self {
            period,
            q,
            pairs: VecDeque::with_capacity(period),
            value: None,
        })
    }
    pub fn append(&mut self, a: f64, b: f64) -> Option<f64> {
        self.pairs.push_back((a, b));
        if self.pairs.len() > self.period {
            self.pairs.pop_front();
        }
        self.value = (self.pairs.len() == self.period).then(|| {
            let mean = self.pairs.iter().map(|(x, _)| x).sum::<f64>() / self.period as f64;
            let var = self
                .pairs
                .iter()
                .map(|(x, _)| (x - mean).powi(2))
                .sum::<f64>();
            let mean_b = self.pairs.iter().map(|(_, y)| y).sum::<f64>() / self.period as f64;
            let var_b = self
                .pairs
                .iter()
                .map(|(_, y)| (y - mean_b).powi(2))
                .sum::<f64>();
            if var_b == 0.0 {
                0.0
            } else {
                var / var_b
            }
        });
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.pairs.clear();
        self.value = None;
    }
}
