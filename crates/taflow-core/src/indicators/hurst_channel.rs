use crate::error::TaResult;
use std::collections::VecDeque;
#[derive(Debug, Clone)]
pub struct HurstChannel {
    period: usize,
    multiplier: f64,
    rows: VecDeque<(f64, f64, f64)>,
    value: Option<f64>,
}
impl HurstChannel {
    pub fn new(period: usize, multiplier: f64) -> TaResult<Self> {
        Ok(Self {
            period,
            multiplier,
            rows: VecDeque::with_capacity(period),
            value: None,
        })
    }
    pub fn append(&mut self, h: f64, l: f64, c: f64) -> Option<f64> {
        self.rows.push_back((h, l, c));
        if self.rows.len() > self.period {
            self.rows.pop_front();
        }
        self.value = (self.rows.len() == self.period).then(|| {
            let high = self
                .rows
                .iter()
                .map(|x| x.0)
                .fold(f64::NEG_INFINITY, f64::max);
            let low = self.rows.iter().map(|x| x.1).fold(f64::INFINITY, f64::min);
            let middle = self.rows.iter().map(|x| x.2).sum::<f64>() / self.period as f64;
            middle + self.multiplier * (high - low)
        });
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.rows.clear();
        self.value = None;
    }
}
