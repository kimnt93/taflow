use crate::error::TaResult;

#[derive(Debug, Clone)]
pub struct SessionRange {
    offset_minutes: i32,
    day: Option<i64>,
    high: f64,
    low: f64,
    value: Option<f64>,
}
impl SessionRange {
    pub fn new(utc_offset_minutes: i32) -> TaResult<Self> {
        Ok(Self {
            offset_minutes: utc_offset_minutes,
            day: None,
            high: f64::NEG_INFINITY,
            low: f64::INFINITY,
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
        if self.day != Some(day) {
            self.day = Some(day);
            self.high = high;
            self.low = low;
        } else {
            self.high = self.high.max(high);
            self.low = self.low.min(low);
        }
        self.value = Some(self.high - self.low);
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.day = None;
        self.high = f64::NEG_INFINITY;
        self.low = f64::INFINITY;
        self.value = None;
    }
}
