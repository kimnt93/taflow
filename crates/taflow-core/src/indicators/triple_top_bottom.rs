use crate::error::TaResult;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct TripleTopBottom {
    rows: VecDeque<[f64; 4]>,
    count: usize,
    value: Option<f64>,
}

impl TripleTopBottom {
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
        let highs = [rows[1][1], rows[3][1], rows[5][1]];
        let lows = [rows[1][2], rows[3][2], rows[5][2]];
        let high_span = highs.iter().copied().fold(f64::NEG_INFINITY, f64::max)
            - highs.iter().copied().fold(f64::INFINITY, f64::min);
        let low_span = lows.iter().copied().fold(f64::NEG_INFINITY, f64::max)
            - lows.iter().copied().fold(f64::INFINITY, f64::min);
        if high_span / highs[0].abs().max(1.0) <= 0.02 {
            -1.0
        } else if low_span / lows[0].abs().max(1.0) <= 0.02 {
            1.0
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
