use crate::error::TaResult;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Mean close returns ordered from Monday through Sunday.
pub struct DayOfWeekReturnProfileValue {
    pub bins: [f64; 7],
}

/// Running mean close return for Monday through Sunday.
#[derive(Debug, Clone)]
pub struct DayOfWeekReturnProfile {
    offset_minutes: i32,
    previous_close: Option<f64>,
    sums: [f64; 7],
    counts: [u64; 7],
    value: Option<DayOfWeekReturnProfileValue>,
}

impl DayOfWeekReturnProfile {
    /// Create an empty weekday profile with a fixed UTC offset.
    pub fn new(utc_offset_minutes: i32) -> TaResult<Self> {
        Ok(Self {
            offset_minutes: utc_offset_minutes,
            previous_close: None,
            sums: [0.0; 7],
            counts: [0; 7],
            value: None,
        })
    }

    /// Append one OHLCV bar and Unix-nanosecond timestamp.
    pub fn append(
        &mut self,
        _open: f64,
        _high: f64,
        _low: f64,
        close: f64,
        _volume: f64,
        timestamp: i64,
    ) -> Option<DayOfWeekReturnProfileValue> {
        let Some(previous) = self.previous_close.replace(close) else {
            return None;
        };
        let local_seconds =
            timestamp.div_euclid(1_000_000_000) + i64::from(self.offset_minutes) * 60;
        let weekday = (local_seconds.div_euclid(86_400) + 3).rem_euclid(7) as usize;
        self.sums[weekday] += if previous == 0.0 {
            0.0
        } else {
            close / previous - 1.0
        };
        self.counts[weekday] += 1;
        let mut bins = [0.0; 7];
        for index in 0..7 {
            if self.counts[index] > 0 {
                bins[index] = self.sums[index] / self.counts[index] as f64;
            }
        }
        self.value = Some(DayOfWeekReturnProfileValue { bins });
        self.value
    }

    /// Return Monday-to-Sunday means after the first return exists.
    pub fn value(&self) -> Option<DayOfWeekReturnProfileValue> {
        self.value
    }
    /// Restore fresh-state behavior and clear all weekday accumulators.
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.sums = [0.0; 7];
        self.counts = [0; 7];
        self.value = None;
    }
}
