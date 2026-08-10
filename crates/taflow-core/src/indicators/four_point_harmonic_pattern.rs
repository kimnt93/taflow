use crate::error::TaResult;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct FourPointHarmonicPattern {
    rows: VecDeque<[f64; 4]>,
    count: usize,
    value: Option<f64>,
}

impl FourPointHarmonicPattern {
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            rows: VecDeque::with_capacity(5),
            count: 0,
            value: None,
        })
    }

    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<f64> {
        self.count += 1;
        if self.rows.len() == 5 {
            self.rows.pop_front();
        }
        self.rows.push_back([open, high, low, close]);
        self.value = (self.rows.len() == 5).then(|| Self::signal(&self.rows));
        self.value
    }

    fn signal(rows: &VecDeque<[f64; 4]>) -> f64 {
        let a = rows[1][3] - rows[0][3];
        let b = rows[2][3] - rows[1][3];
        let c = rows[3][3] - rows[2][3];
        let d = rows[4][3] - rows[3][3];
        if a == 0.0 || b == 0.0 || c == 0.0 {
            return 0.0;
        }
        let alternating =
            a.signum() != b.signum() && b.signum() != c.signum() && c.signum() != d.signum();
        let reciprocal = (b.abs() / a.abs() - d.abs() / c.abs()).abs() <= 0.15;
        if alternating && reciprocal {
            -d.signum()
        } else {
            0.0
        }
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
