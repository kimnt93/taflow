use crate::error::TaResult;
use std::collections::VecDeque;
#[derive(Debug, Clone)]
pub struct RollingCoefficientOfDetermination {
    period: usize,
    pairs: VecDeque<(f64, f64)>,
    value: Option<f64>,
}
impl RollingCoefficientOfDetermination {
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
            let (mut c, mut a, mut b) = (0.0, 0.0, 0.0);
            for &(x, y) in &self.pairs {
                let u = x - mx;
                let v = y - my;
                c += u * v;
                a += u * u;
                b += v * v;
            }
            if a * b == 0.0 {
                0.0
            } else {
                (c / (a * b).sqrt()).powi(2)
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
