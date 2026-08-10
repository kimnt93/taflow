use crate::error::{TaError, TaResult};
use crate::stream::invalid_period;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HurstChannelValue {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}

/// SMA centerline wrapped by a multiple of the rolling high-low range.
#[derive(Debug, Clone)]
pub struct HurstChannel {
    period: usize,
    multiplier: f64,
    rows: VecDeque<(f64, f64, f64)>,
    close_sum: f64,
    value: Option<HurstChannelValue>,
}

impl HurstChannel {
    /// Create a range channel with positive period and multiplier.
    pub fn new(period: usize, multiplier: f64) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("period", period, 1));
        }
        if !multiplier.is_finite() || multiplier <= 0.0 {
            return Err(TaError::InvalidParameter {
                name: "multiplier",
                value: multiplier.to_string(),
                reason: "must be finite and positive",
            });
        }
        Ok(Self {
            period,
            multiplier,
            rows: VecDeque::with_capacity(period),
            close_sum: 0.0,
            value: None,
        })
    }

    /// Append one high/low/close bar and return upper, middle, and lower.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<HurstChannelValue> {
        if self.rows.len() == self.period {
            self.close_sum -= self.rows.pop_front().expect("full window").2;
        }
        self.rows.push_back((high, low, close));
        self.close_sum += close;
        self.value = (self.rows.len() == self.period).then(|| {
            let highest = self
                .rows
                .iter()
                .map(|row| row.0)
                .fold(f64::NEG_INFINITY, f64::max);
            let lowest = self
                .rows
                .iter()
                .map(|row| row.1)
                .fold(f64::INFINITY, f64::min);
            let middle = self.close_sum / self.period as f64;
            let width = self.multiplier * (highest - lowest);
            HurstChannelValue {
                upper: middle + width,
                middle,
                lower: middle - width,
            }
        });
        self.value
    }

    /// Return the latest channel, or `None` during warm-up.
    pub fn value(&self) -> Option<HurstChannelValue> {
        self.value
    }

    /// Clear the rolling window and latest value.
    pub fn reset(&mut self) {
        self.rows.clear();
        self.close_sum = 0.0;
        self.value = None;
    }
}
