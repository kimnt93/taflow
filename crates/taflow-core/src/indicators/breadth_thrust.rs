use crate::error::{TaError, TaResult};
use std::collections::VecDeque;
/// Simple mean of the cross-sectional advancing-issues share.
#[derive(Debug, Clone)]
pub struct BreadthThrust {
    period: usize,
    rows: VecDeque<f64>,
    sum: f64,
    count: usize,
    value: Option<f64>,
}
impl BreadthThrust {
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
    /// Append aggregate advancing and declining issue counts.
    pub fn append(&mut self, advancers: f64, decliners: f64) -> Option<f64> {
        self.count += 1;
        let share = advancers / (advancers + decliners).max(1.0);
        if self.rows.len() == self.period {
            self.sum -= self.rows.pop_front().expect("full window");
        }
        self.rows.push_back(share);
        self.sum += share;
        self.value = (self.rows.len() == self.period).then(|| self.sum / self.period as f64);
        self.value
    }
    /// Return the latest smoothed advancing share, or `None` in warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Return the number of processed market ticks.
    pub fn len(&self) -> usize {
        self.count
    }
    /// Return whether no market ticks have been processed.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    /// Clear the rolling window while retaining its allocation.
    pub fn reset(&mut self) {
        self.rows.clear();
        self.sum = 0.0;
        self.count = 0;
        self.value = None;
    }
}
