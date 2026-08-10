use crate::error::TaResult;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct RollingCointegration {
    period: usize,
    pairs: VecDeque<(f64, f64)>,
    value: Option<f64>,
}
impl RollingCointegration {
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            period,
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
            let n = self.period as f64;
            let ma = self.pairs.iter().map(|p| p.0).sum::<f64>() / n;
            let mb = self.pairs.iter().map(|p| p.1).sum::<f64>() / n;
            let (mut cov, mut var) = (0.0, 0.0);
            for &(x, y) in &self.pairs {
                cov += (x - ma) * (y - mb);
                var += (x - ma).powi(2);
            }
            if var == 0.0 {
                0.0
            } else {
                let beta = cov / var;
                let alpha = mb - beta * ma;
                let sse = self
                    .pairs
                    .iter()
                    .map(|&(x, y)| (y - alpha - beta * x).powi(2))
                    .sum::<f64>();
                (sse / n).sqrt()
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
