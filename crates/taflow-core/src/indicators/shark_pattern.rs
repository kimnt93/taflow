use crate::error::TaResult;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct SharkPattern {
    rows: VecDeque<[f64; 4]>,
    count: usize,
    value: Option<f64>,
}

impl SharkPattern {
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
        let x = rows[1][3];
        let a = rows[2][3];
        let b = rows[3][3];
        let c = rows[4][3];
        let d = rows[5][3];
        let xa = (a - x).abs();
        let ab = (b - a).abs();
        let bc = (c - b).abs();
        let cd = (d - c).abs();
        if xa == 0.0 || ab == 0.0 || bc == 0.0 {
            return 0.0;
        }
        let valid = (0.382..=0.886).contains(&(ab / xa))
            && (1.13..=1.618).contains(&(bc / ab))
            && (0.886..=1.13).contains(&(cd / bc));
        if valid {
            -(d - c).signum()
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
