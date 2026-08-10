use crate::error::TaResult;

/// Per-session ranges for Asia, Europe, and the United States.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SessionRangeValue {
    pub asia: f64,
    pub europe: f64,
    pub united_states: f64,
}

#[derive(Debug, Clone, Copy)]
struct Extent {
    high: f64,
    low: f64,
}

impl Extent {
    const EMPTY: Self = Self {
        high: f64::NEG_INFINITY,
        low: f64::INFINITY,
    };
    fn append(&mut self, high: f64, low: f64) {
        self.high = self.high.max(high);
        self.low = self.low.min(low);
    }
    fn range(self) -> f64 {
        if self.high >= self.low {
            self.high - self.low
        } else {
            0.0
        }
    }
}

/// Track high-low ranges in three eight-hour local trading sessions.
#[derive(Debug, Clone)]
pub struct SessionRange {
    offset_minutes: i32,
    day: Option<i64>,
    sessions: [Extent; 3],
    value: Option<SessionRangeValue>,
}

impl SessionRange {
    /// Create a state using a signed UTC offset in minutes.
    pub fn new(utc_offset_minutes: i32) -> TaResult<Self> {
        Ok(Self {
            offset_minutes: utc_offset_minutes,
            day: None,
            sessions: [Extent::EMPTY; 3],
            value: None,
        })
    }

    /// Append one OHLCV bar with a Unix-nanosecond timestamp.
    pub fn append(
        &mut self,
        _open: f64,
        high: f64,
        low: f64,
        _close: f64,
        _volume: f64,
        timestamp: i64,
    ) -> Option<SessionRangeValue> {
        let local_seconds =
            timestamp.div_euclid(1_000_000_000) + i64::from(self.offset_minutes) * 60;
        let day = local_seconds.div_euclid(86_400);
        if self.day != Some(day) {
            self.day = Some(day);
            self.sessions = [Extent::EMPTY; 3];
        }
        let hour = local_seconds.rem_euclid(86_400) / 3_600;
        self.sessions[(hour / 8) as usize].append(high, low);
        self.value = Some(SessionRangeValue {
            asia: self.sessions[0].range(),
            europe: self.sessions[1].range(),
            united_states: self.sessions[2].range(),
        });
        self.value
    }

    /// Return the latest three-session snapshot.
    pub fn value(&self) -> Option<SessionRangeValue> {
        self.value
    }

    /// Clear the day, extents, and latest snapshot.
    pub fn reset(&mut self) {
        self.day = None;
        self.sessions = [Extent::EMPTY; 3];
        self.value = None;
    }
}
