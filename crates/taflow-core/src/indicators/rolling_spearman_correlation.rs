use crate::error::TaResult;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct RollingSpearmanCorrelation {
    period: usize,
    pairs: VecDeque<(f64, f64)>,
    value: Option<f64>,
}
fn rank(values: &[f64], index: usize) -> f64 {
    let x = values[index];
    1.0 + values.iter().filter(|&&v| v < x).count() as f64
        + 0.5 * values.iter().filter(|&&v| v == x).count().saturating_sub(1) as f64
}
impl RollingSpearmanCorrelation {
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            period,
            pairs: VecDeque::with_capacity(period),
            value: None,
        })
    }
    pub fn append(&mut self, x: f64, y: f64) -> Option<f64> {
        self.pairs.push_back((x, y));
        if self.pairs.len() > self.period {
            self.pairs.pop_front();
        }
        self.value = (self.pairs.len() == self.period).then(|| {
            let a: Vec<_> = self.pairs.iter().map(|p| p.0).collect();
            let b: Vec<_> = self.pairs.iter().map(|p| p.1).collect();
            let ra: Vec<_> = (0..self.period).map(|i| rank(&a, i)).collect();
            let rb: Vec<_> = (0..self.period).map(|i| rank(&b, i)).collect();
            let ma = ra.iter().sum::<f64>() / self.period as f64;
            let mb = rb.iter().sum::<f64>() / self.period as f64;
            let (mut n, mut da, mut db) = (0.0, 0.0, 0.0);
            for i in 0..self.period {
                let u = ra[i] - ma;
                let v = rb[i] - mb;
                n += u * v;
                da += u * u;
                db += v * v;
            }
            if da * db == 0.0 {
                0.0
            } else {
                n / (da * db).sqrt()
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
