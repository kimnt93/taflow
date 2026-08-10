use crate::error::TaResult;
use crate::stream::invalid_period;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Borrowed mean-return values for every configured intraday bucket.
pub struct TimeOfDayReturnProfileValue<'a> {
    pub bins: &'a [f64],
}

/// Running mean close return in equal local-time-of-day buckets.
#[derive(Debug, Clone)]
pub struct TimeOfDayReturnProfile {
    buckets: usize,
    offset_minutes: i32,
    previous_close: Option<f64>,
    sums: Vec<f64>,
    counts: Vec<u64>,
    bins: Vec<f64>,
    ready: bool,
}

impl TimeOfDayReturnProfile {
    /// Create an empty profile with equally sized local-day buckets.
    pub fn new(buckets: usize, utc_offset_minutes: i32) -> TaResult<Self> {
        if buckets == 0 {
            return Err(invalid_period("buckets", buckets, 1));
        }
        Ok(Self {
            buckets,
            offset_minutes: utc_offset_minutes,
            previous_close: None,
            sums: vec![0.0; buckets],
            counts: vec![0; buckets],
            bins: vec![0.0; buckets],
            ready: false,
        })
    }

    fn bucket(&self, timestamp: i64) -> usize {
        let seconds = timestamp.div_euclid(1_000_000_000) + i64::from(self.offset_minutes) * 60;
        ((seconds.rem_euclid(86_400) as usize * self.buckets) / 86_400).min(self.buckets - 1)
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
    ) -> Option<TimeOfDayReturnProfileValue<'_>> {
        let Some(previous) = self.previous_close.replace(close) else {
            return None;
        };
        let bucket = self.bucket(timestamp);
        let return_value = if previous == 0.0 {
            0.0
        } else {
            close / previous - 1.0
        };
        self.sums[bucket] += return_value;
        self.counts[bucket] += 1;
        self.bins[bucket] = self.sums[bucket] / self.counts[bucket] as f64;
        self.ready = true;
        self.value()
    }

    /// Return all bucket means after the first close return exists.
    pub fn value(&self) -> Option<TimeOfDayReturnProfileValue<'_>> {
        self.ready
            .then_some(TimeOfDayReturnProfileValue { bins: &self.bins })
    }
    /// Restore fresh-state behavior without reallocating profile vectors.
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.sums.fill(0.0);
        self.counts.fill(0);
        self.bins.fill(0.0);
        self.ready = false;
    }
}
