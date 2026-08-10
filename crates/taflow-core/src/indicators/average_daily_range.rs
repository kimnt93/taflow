use crate::error::{TaError, TaResult};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct AverageDailyRange {
    period: usize,
    offset_minutes: i32,
    day: Option<i64>,
    high: f64,
    low: f64,
    ranges: VecDeque<f64>,
    sum: f64,
    value: Option<f64>,
}
impl AverageDailyRange {
    pub fn new(period: usize, utc_offset_minutes: i32) -> TaResult<Self> {
        if period == 0 {
            return Err(TaError::InvalidParameter {
                name: "period",
                value: period.to_string(),
                reason: "must be positive",
            });
        }
        Ok(Self {
            period,
            offset_minutes: utc_offset_minutes,
            day: None,
            high: f64::NEG_INFINITY,
            low: f64::INFINITY,
            ranges: VecDeque::with_capacity(period),
            sum: 0.0,
            value: None,
        })
    }
    fn day(&self, timestamp: i64) -> i64 {
        (timestamp + self.offset_minutes as i64 * 60_000_000_000).div_euclid(86_400_000_000_000)
    }
    pub fn append(
        &mut self,
        _open: f64,
        high: f64,
        low: f64,
        _close: f64,
        _volume: f64,
        timestamp: i64,
    ) -> Option<f64> {
        let day = self.day(timestamp);
        if self.day.is_some() && self.day != Some(day) {
            let range = self.high - self.low;
            if self.ranges.len() == self.period {
                self.sum -= self.ranges.pop_front().expect("full window");
            }
            self.ranges.push_back(range);
            self.sum += range;
            self.value = Some(self.sum / self.ranges.len() as f64);
            self.high = high;
            self.low = low;
        } else {
            self.high = self.high.max(high);
            self.low = self.low.min(low);
        }
        self.day = Some(day);
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.day = None;
        self.high = f64::NEG_INFINITY;
        self.low = f64::INFINITY;
        self.ranges.clear();
        self.sum = 0.0;
        self.value = None;
    }
}
