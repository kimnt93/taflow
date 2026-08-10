use crate::error::{TaError, TaResult};
use std::collections::VecDeque;
/// Simple mean of the percentage of new extremes that are new highs.
#[derive(Debug, Clone)]
pub struct HighLowIndex {
    period: usize,
    rows: VecDeque<f64>,
    sum: f64,
    count: usize,
    value: Option<f64>,
}
impl HighLowIndex {
    /// Create a rolling state with a positive smoothing period.
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
            sum: 0.0,
            count: 0,
            value: None,
        })
    }
    /// Append aggregate new-high and new-low counts for one market tick.
    pub fn append(&mut self, new_highs: f64, new_lows: f64) -> Option<f64> {
        self.count += 1;
        let percent = 100.0 * new_highs / (new_highs + new_lows).max(1.0);
        if self.rows.len() == self.period {
            self.sum -= self.rows.pop_front().expect("full window");
        }
        self.rows.push_back(percent);
        self.sum += percent;
        self.value = (self.rows.len() == self.period).then(|| self.sum / self.period as f64);
        self.value
    }
    /// Return the number of processed market ticks.
    pub fn len(&self) -> usize {
        self.count
    }
    /// Return the latest smoothed percentage, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Clear the rolling window while retaining its allocation.
    pub fn reset(&mut self) {
        self.rows.clear();
        self.sum = 0.0;
        self.count = 0;
        self.value = None;
    }
}
