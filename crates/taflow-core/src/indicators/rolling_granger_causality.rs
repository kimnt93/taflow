use crate::error::TaResult;
use crate::stream::invalid_period;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct RollingGrangerCausality {
    period: usize,
    lag: usize,
    pairs: VecDeque<(f64, f64)>,
    value: Option<f64>,
}
impl RollingGrangerCausality {
    pub fn new(period: usize, lag: usize) -> TaResult<Self> {
        if lag == 0 {
            return Err(invalid_period("lag", lag, 1));
        }
        Ok(Self {
            period,
            lag,
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
            if self.period <= self.lag {
                0.0
            } else {
                let mut n = 0.0;
                let mut d = 0.0;
                for i in self.lag..self.period {
                    let x = self.pairs[i].0 - self.pairs[i - self.lag].0;
                    let y = self.pairs[i].1 - self.pairs[i - self.lag].1;
                    n += x * y;
                    d += x * x;
                }
                if d == 0.0 {
                    0.0
                } else {
                    n / d
                }
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
