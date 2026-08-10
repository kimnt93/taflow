use crate::error::TaResult;
use std::collections::VecDeque;
#[derive(Debug, Clone)]
pub struct TrianglePattern {
    rows: VecDeque<(f64, f64, f64, f64)>,
    count: usize,
    value: Option<f64>,
}
impl TrianglePattern {
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
            let hs = self.rows[19].1 - self.rows[0].1;
            let ls = self.rows[19].2 - self.rows[0].2;
            if hs < 0.0 && ls > 0.0 {
                1.0
            } else if hs.abs() < ls.abs() * 0.2 && ls > 0.0 {
                1.0
            } else if ls.abs() < hs.abs() * 0.2 && hs < 0.0 {
                -1.0
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
