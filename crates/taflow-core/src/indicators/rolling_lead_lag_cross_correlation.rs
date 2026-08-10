use crate::error::TaResult;
use std::collections::VecDeque;
#[derive(Debug, Clone)]
pub struct RollingLeadLagCrossCorrelation {
    period: usize,
    lag: usize,
    pairs: VecDeque<(f64, f64)>,
    value: Option<f64>,
}
impl RollingLeadLagCrossCorrelation {
    pub fn new(period: usize, lag: usize) -> TaResult<Self> {
        Ok(Self {
            period,
            lag,
            pairs: VecDeque::with_capacity(period + lag),
            value: None,
        })
    }
    pub fn append(&mut self, x: f64, y: f64) -> Option<f64> {
        self.pairs.push_back((x, y));
        if self.pairs.len() > self.period + self.lag {
            self.pairs.pop_front();
        }
        self.value = (self.pairs.len() == self.period + self.lag).then(|| {
            let n = self.period;
            let a: Vec<_> = self.pairs.iter().skip(self.lag).map(|p| p.0).collect();
            let b: Vec<_> = self.pairs.iter().take(n).map(|p| p.1).collect();
            let ma = a.iter().sum::<f64>() / n as f64;
            let mb = b.iter().sum::<f64>() / n as f64;
            let (mut c, mut va, mut vb) = (0.0, 0.0, 0.0);
            for i in 0..n {
                let u = a[i] - ma;
                let v = b[i] - mb;
                c += u * v;
                va += u * u;
                vb += v * v;
            }
            if va * vb == 0.0 {
                0.0
            } else {
                c / (va * vb).sqrt()
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
