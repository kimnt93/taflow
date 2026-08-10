use crate::error::TaResult;

/// Close-to-open return held throughout each local trading day.
#[derive(Debug, Clone)]
pub struct OvernightGap {
    offset_minutes: i32,
    day: Option<i64>,
    last_close: Option<f64>,
    value: Option<f64>,
}

impl OvernightGap {
    /// Create a calendar state using a signed UTC offset in minutes.
    pub fn new(utc_offset_minutes: i32) -> TaResult<Self> {
        Ok(Self {
            offset_minutes: utc_offset_minutes,
            day: None,
            last_close: None,
            value: None,
        })
    }

    /// Append one OHLCV bar with a Unix-nanosecond timestamp.
    pub fn append(
        &mut self,
        open: f64,
        _high: f64,
        _low: f64,
        close: f64,
        _volume: f64,
        timestamp: i64,
    ) -> Option<f64> {
        let local_seconds =
            timestamp.div_euclid(1_000_000_000) + i64::from(self.offset_minutes) * 60;
        let day = local_seconds.div_euclid(86_400);
        if self.day != Some(day) {
            if let Some(previous_close) = self.last_close {
                self.value = Some(if previous_close == 0.0 {
                    0.0
                } else {
                    open / previous_close - 1.0
                });
            }
            self.day = Some(day);
        }
        self.last_close = Some(close);
        self.value
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.day = None;
        self.last_close = None;
        self.value = None;
    }
}
