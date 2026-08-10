use crate::error::TaResult;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OvernightIntradayReturnValue {
    pub overnight: f64,
    pub intraday: f64,
}

/// Split each local session's return into overnight and intraday components.
#[derive(Debug, Clone)]
pub struct OvernightIntradayReturn {
    offset_minutes: i32,
    day: Option<i64>,
    last_close: Option<f64>,
    day_open: f64,
    overnight: Option<f64>,
    value: Option<OvernightIntradayReturnValue>,
}

impl OvernightIntradayReturn {
    /// Create a state using a signed UTC offset in minutes.
    pub fn new(utc_offset_minutes: i32) -> TaResult<Self> {
        Ok(Self {
            offset_minutes: utc_offset_minutes,
            day: None,
            last_close: None,
            day_open: 0.0,
            overnight: None,
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
    ) -> Option<OvernightIntradayReturnValue> {
        let local_seconds =
            timestamp.div_euclid(1_000_000_000) + i64::from(self.offset_minutes) * 60;
        let day = local_seconds.div_euclid(86_400);
        if self.day != Some(day) {
            if let Some(previous_close) = self.last_close {
                self.overnight = Some(if previous_close == 0.0 {
                    0.0
                } else {
                    open / previous_close - 1.0
                });
            }
            self.day_open = open;
            self.day = Some(day);
        }
        self.last_close = Some(close);
        self.value = self
            .overnight
            .map(|overnight| OvernightIntradayReturnValue {
                overnight,
                intraday: if self.day_open == 0.0 {
                    0.0
                } else {
                    close / self.day_open - 1.0
                },
            });
        self.value
    }

    /// Return the latest return decomposition.
    pub fn value(&self) -> Option<OvernightIntradayReturnValue> {
        self.value
    }

    /// Clear calendar anchors and latest decomposition.
    pub fn reset(&mut self) {
        self.day = None;
        self.last_close = None;
        self.day_open = 0.0;
        self.overnight = None;
        self.value = None;
    }
}
