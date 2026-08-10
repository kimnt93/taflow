use crate::error::{TaError, TaResult};

/// Borrowed average-volume values for every configured intraday time bucket.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VolumeByTimeProfileValue<'a> {
    pub bins: &'a [f64],
}

/// Running arithmetic mean of volume grouped by time of day.
#[derive(Debug, Clone)]
pub struct VolumeByTimeProfile {
    buckets: usize,
    offset: i32,
    sums: Vec<f64>,
    counts: Vec<usize>,
    bins: Vec<f64>,
    available: bool,
}

impl VolumeByTimeProfile {
    /// Create an empty profile with evenly sized daily buckets.
    pub fn new(buckets: usize, utc_offset_minutes: i32) -> TaResult<Self> {
        if buckets == 0 {
            return Err(TaError::InvalidParameter {
                name: "buckets",
                value: buckets.to_string(),
                reason: "must be positive",
            });
        }
        Ok(Self {
            buckets,
            offset: utc_offset_minutes,
            sums: vec![0.0; buckets],
            counts: vec![0; buckets],
            bins: vec![0.0; buckets],
            available: false,
        })
    }

    fn bucket(&self, timestamp: i64) -> usize {
        const DAY_NS: i128 = 86_400_000_000_000;
        let local = timestamp as i128 + self.offset as i128 * 60_000_000_000;
        (local.rem_euclid(DAY_NS) * self.buckets as i128 / DAY_NS) as usize
    }

    /// Append one chronological OHLCV bar and Unix-nanosecond timestamp.
    pub fn append(
        &mut self,
        _open: f64,
        _high: f64,
        _low: f64,
        _close: f64,
        volume: f64,
        timestamp: i64,
    ) -> Option<VolumeByTimeProfileValue<'_>> {
        let bucket = self.bucket(timestamp);
        self.sums[bucket] += volume;
        self.counts[bucket] += 1;
        self.bins[bucket] = self.sums[bucket] / self.counts[bucket] as f64;
        self.available = true;
        self.value()
    }

    /// Return all current bucket means after at least one bar exists.
    pub fn value(&self) -> Option<VolumeByTimeProfileValue<'_>> {
        self.available
            .then_some(VolumeByTimeProfileValue { bins: &self.bins })
    }

    /// Restore the profile to its newly constructed state without reallocating.
    pub fn reset(&mut self) {
        self.sums.fill(0.0);
        self.counts.fill(0);
        self.bins.fill(0.0);
        self.available = false;
    }
}
