use crate::error::TaResult;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct ThreeDrives {
    rows: VecDeque<[f64; 4]>,
    count: usize,
    value: Option<f64>,
}

impl ThreeDrives {
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            rows: VecDeque::with_capacity(6),
            count: 0,
            value: None,
        })
    }

    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<f64> {
        self.count += 1;
        if self.rows.len() == 6 {
            self.rows.pop_front();
        }
        self.rows.push_back([open, high, low, close]);
        self.value = (self.rows.len() == 6).then(|| Self::signal(&self.rows));
        self.value
    }

    fn signal(rows: &VecDeque<[f64; 4]>) -> f64 {
        let first = rows[1][3] - rows[0][3];
        let second = rows[3][3] - rows[2][3];
        let third = rows[5][3] - rows[4][3];
        let same_direction = first.signum() == second.signum() && second.signum() == third.signum();
        let comparable = first != 0.0
            && (second.abs() / first.abs() - 1.272).abs() <= 0.35
            && (third.abs() / second.abs() - 1.272).abs() <= 0.35;
        if same_direction && comparable {
            -third.signum()
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
