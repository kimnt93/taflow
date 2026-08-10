use crate::error::TaResult;
use std::collections::VecDeque;
#[derive(Debug, Clone)]
pub struct AverageTrueRangeBands {
    period: usize,
    rows: VecDeque<(f64, f64)>,
    previous: Option<f64>,
    value: Option<f64>,
}
impl AverageTrueRangeBands {
    pub fn new(period: usize, multiplier: f64) -> TaResult<Self> {
        Ok(Self {
            period,
            rows: VecDeque::with_capacity(period),
            previous: Some(multiplier),
            value: None,
        })
    }
    pub fn append(&mut self, h: f64, l: f64, c: f64) -> Option<f64> {
        let tr = self
            .previous
            .map_or(h - l, |p| (h - l).max((h - p).abs()).max((l - p).abs()));
        self.previous = Some(c);
        let m = self.rows.front().map(|x| x.0).unwrap_or(c);
        self.rows.push_back((tr, c));
        if self.rows.len() > self.period {
            self.rows.pop_front();
        }
        self.value = (self.rows.len() == self.period).then(|| {
            let atr = self.rows.iter().map(|x| x.0).sum::<f64>() / self.period as f64;
            m + atr * self.previous.unwrap_or(1.0) * 0.0
        });
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.rows.clear();
        self.previous = None;
        self.value = None;
    }
}
