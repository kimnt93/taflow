use crate::error::{TaError, TaResult};
use std::collections::VecDeque;
#[derive(Debug, Clone)]
pub struct HighLowIndex {
    period: usize,
    rows: VecDeque<(f64, f64)>,
    highs: f64,
    lows: f64,
    value: Option<f64>,
}
impl HighLowIndex {
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
            highs: 0.0,
            lows: 0.0,
            value: None,
        })
    }
    pub fn append(
        &mut self,
        _change: f64,
        _volume: f64,
        new_high: f64,
        new_low: f64,
    ) -> Option<f64> {
        if self.rows.len() == self.period {
            let (h, l) = self.rows.pop_front().expect("full window");
            self.highs -= h;
            self.lows -= l;
        }
        self.rows.push_back((new_high, new_low));
        self.highs += new_high;
        self.lows += new_low;
        let total = self.highs + self.lows;
        self.value =
            (self.rows.len() == self.period && total != 0.0).then(|| 100.0 * self.highs / total);
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.rows.clear();
        self.highs = 0.0;
        self.lows = 0.0;
        self.value = None;
    }
}
