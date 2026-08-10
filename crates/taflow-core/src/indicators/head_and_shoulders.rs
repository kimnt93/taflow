use crate::error::TaResult;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct HeadAndShoulders {
    rows: VecDeque<[f64; 4]>,
    count: usize,
    value: Option<f64>,
}

impl HeadAndShoulders {
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
        let left_shoulder = rows[1][1];
        let head = rows[3][1];
        let right_shoulder = rows[5][1];
        let scale = head.abs().max(1.0);
        let bearish = head > left_shoulder
            && head > right_shoulder
            && (left_shoulder - right_shoulder).abs() / scale <= 0.03;
        let left_trough = rows[1][2];
        let bottom = rows[3][2];
        let right_trough = rows[5][2];
        let bullish = bottom < left_trough
            && bottom < right_trough
            && (left_trough - right_trough).abs() / bottom.abs().max(1.0) <= 0.03;
        if bearish {
            -1.0
        } else if bullish {
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
