use crate::error::{TaError, TaResult};
use std::collections::VecDeque;

/// Population covariance of trailing one-bar changes in two level series.
#[derive(Debug, Clone)]
pub struct RollingCovariance {
    period: usize,
    previous: Option<(f64, f64)>,
    changes: VecDeque<(f64, f64)>,
    sum_left: f64,
    sum_right: f64,
    sum_product: f64,
    value: Option<f64>,
}

impl RollingCovariance {
    /// Create a return-covariance window containing at least two changes.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: period.to_string(),
                reason: "must be >= 2",
            });
        }
        Ok(Self {
            period,
            previous: None,
            changes: VecDeque::with_capacity(period),
            sum_left: 0.0,
            sum_right: 0.0,
            sum_product: 0.0,
            value: None,
        })
    }

    /// Append one level pair and return covariance after `period + 1` levels.
    pub fn append(&mut self, left: f64, right: f64) -> Option<f64> {
        let Some((previous_left, previous_right)) = self.previous.replace((left, right)) else {
            return None;
        };
        let change = (left - previous_left, right - previous_right);
        if self.changes.len() == self.period {
            let expired = self.changes.pop_front().expect("full covariance window");
            self.sum_left -= expired.0;
            self.sum_right -= expired.1;
            self.sum_product -= expired.0 * expired.1;
        }
        self.changes.push_back(change);
        self.sum_left += change.0;
        self.sum_right += change.1;
        self.sum_product += change.0 * change.1;
        self.value = (self.changes.len() == self.period).then(|| {
            let count = self.period as f64;
            self.sum_product / count - (self.sum_left / count) * (self.sum_right / count)
        });
        self.value
    }

    /// Return the latest covariance, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restore the newly constructed state without reallocating its window.
    pub fn reset(&mut self) {
        self.previous = None;
        self.changes.clear();
        self.sum_left = 0.0;
        self.sum_right = 0.0;
        self.sum_product = 0.0;
        self.value = None;
    }
}
