use crate::error::{TaError, TaResult};
use std::collections::VecDeque;
#[derive(Debug, Clone)]
pub struct BreadthThrust {
    period: usize,
    rows: VecDeque<(f64, f64)>,
    advances: f64,
    total: f64,
    count: usize,
    value: Option<f64>,
}
impl BreadthThrust {
    pub fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(TaError::InvalidParameter {
                name: "period",
                value: period.to_string(),
                reason: "must be positive",
            });
        }
        Ok(Self {
            period,
            rows: VecDeque::with_capacity(period),
            advances: 0.0,
            total: 0.0,
            count: 0,
            value: None,
        })
    }
    pub fn append(
        &mut self,
        change: f64,
        _volume: f64,
        _new_high: f64,
        _new_low: f64,
    ) -> Option<f64> {
        self.count += 1;
        let a = change.max(0.0);
        let t = change.abs();
        if self.rows.len() == self.period {
            let (x, y) = self.rows.pop_front().expect("full window");
            self.advances -= x;
            self.total -= y;
        }
        self.rows.push_back((a, t));
        self.advances += a;
        self.total += t;
        self.value = (self.rows.len() == self.period && self.total != 0.0)
            .then(|| self.advances / self.total);
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
        self.advances = 0.0;
        self.total = 0.0;
        self.count = 0;
        self.value = None;
    }
}
