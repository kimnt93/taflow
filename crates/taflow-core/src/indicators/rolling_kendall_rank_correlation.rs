use crate::error::TaResult;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct RollingKendallRankCorrelation {
    period: usize,
    pairs: VecDeque<(f64, f64)>,
    value: Option<f64>,
}
impl RollingKendallRankCorrelation {
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
            let (mut c, mut d) = (0.0, 0.0);
            let v: Vec<_> = self.pairs.iter().collect();
            for i in 0..v.len() {
                for j in i + 1..v.len() {
                    let dx = v[j].0 - v[i].0;
                    let dy = v[j].1 - v[i].1;
                    let s = dx * dy;
                    if s > 0.0 {
                        c += 1.0
                    } else if s < 0.0 {
                        d += 1.0
                    }
                }
            }
            let den = (self.period * (self.period - 1) / 2) as f64;
            if den == 0.0 {
                0.0
            } else {
                (c - d) / den
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
