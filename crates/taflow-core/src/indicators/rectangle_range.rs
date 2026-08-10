use crate::error::TaResult;
use std::collections::VecDeque;
#[derive(Debug, Clone)]
pub struct RectangleRange {
    rows: VecDeque<(f64, f64, f64, f64)>,
    count: usize,
    value: Option<f64>,
}
impl RectangleRange {
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            rows: VecDeque::with_capacity(20),
            count: 0,
            value: None,
        })
    }
    pub fn append(&mut self, o: f64, h: f64, l: f64, c: f64) -> Option<f64> {
        self.count += 1;
        if self.rows.len() == 20 {
            self.rows.pop_front();
        }
        self.rows.push_back((o, h, l, c));
        self.value = (self.rows.len() == 20).then(|| {
            let hi = self
                .rows
                .iter()
                .map(|x| x.1)
                .fold(f64::NEG_INFINITY, f64::max);
            let lo = self.rows.iter().map(|x| x.2).fold(f64::INFINITY, f64::min);
            let mean = self.rows.iter().map(|x| x.3).sum::<f64>() / 20.0;
            if mean != 0.0 && (hi - lo) / mean.abs() < 0.05 {
                1.0
            } else {
                0.0
            }
        });
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn len(&self) -> usize {
        self.count
    }
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    pub fn reset(&mut self) {
        self.rows.clear();
        self.count = 0;
        self.value = None;
    }
}
