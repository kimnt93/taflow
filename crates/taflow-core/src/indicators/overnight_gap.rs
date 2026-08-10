use crate::error::TaResult;

#[derive(Debug, Clone)]
pub struct OvernightGap {
    offset_minutes: i32,
    day: Option<i64>,
    previous_close: Option<f64>,
    value: Option<f64>,
}
impl OvernightGap {
    pub fn new(utc_offset_minutes: i32) -> TaResult<Self> {
        Ok(Self {
            offset_minutes: utc_offset_minutes,
            day: None,
            previous_close: None,
            value: None,
        })
    }
    fn day(&self, timestamp: i64) -> i64 {
        (timestamp + self.offset_minutes as i64 * 60_000_000_000).div_euclid(86_400_000_000_000)
    }
    pub fn append(
        &mut self,
        open: f64,
        _high: f64,
        _low: f64,
        close: f64,
        _volume: f64,
        timestamp: i64,
    ) -> Option<f64> {
        let day = self.day(timestamp);
        self.value = if self.day.is_some() && self.day != Some(day) {
            self.previous_close
                .filter(|x| *x != 0.0)
                .map(|x| open / x - 1.0)
        } else {
            None
        };
        self.day = Some(day);
        self.previous_close = Some(close);
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.day = None;
        self.previous_close = None;
        self.value = None;
    }
}
