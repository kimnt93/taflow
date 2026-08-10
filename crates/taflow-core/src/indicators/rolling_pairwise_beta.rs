use crate::error::TaResult;
use std::collections::VecDeque;
#[derive(Debug, Clone)]
pub struct RollingPairwiseBeta {
    period: usize,
    pairs: VecDeque<(f64, f64)>,
    value: Option<f64>,
}
impl RollingPairwiseBeta {
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
            let n = self.period as f64;
            let mx = self.pairs.iter().map(|p| p.0).sum::<f64>() / n;
            let my = self.pairs.iter().map(|p| p.1).sum::<f64>() / n;
            let (mut c, mut v) = (0.0, 0.0);
            for &(a, b) in &self.pairs {
                c += (a - mx) * (b - my);
                v += (b - my).powi(2);
            }
            if v == 0.0 {
                0.0
            } else {
                c / v
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
