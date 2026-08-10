use crate::error::TaResult;
use std::collections::VecDeque;
#[derive(Debug, Clone)]
pub struct TwiggsMoneyFlow {
    period: usize,
    rows: VecDeque<(f64, f64)>,
    value: Option<f64>,
}
impl TwiggsMoneyFlow {
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            period,
            rows: VecDeque::with_capacity(period),
            value: None,
        })
    }
    pub fn append(&mut self, h: f64, l: f64, c: f64, v: f64) -> Option<f64> {
        let mf = if h == l {
            0.0
        } else {
            ((2.0 * c - h - l) / (h - l)) * v
        };
        self.rows.push_back((mf, v));
        if self.rows.len() > self.period {
            self.rows.pop_front();
        }
        self.value = (self.rows.len() == self.period).then(|| {
            let a = self.rows.iter().map(|x| x.0).sum::<f64>();
            let b = self.rows.iter().map(|x| x.1).sum::<f64>();
            if b == 0.0 {
                0.0
            } else {
                a / b
            }
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
