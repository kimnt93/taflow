use crate::error::{TaError, TaResult};

/// Borrowed volatility values for every configured intraday time bucket.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntradayVolatilityProfileValue<'a> {
    pub bins: &'a [f64],
}

/// Running sample volatility of close-to-close returns by time of day.
#[derive(Debug, Clone)]
pub struct IntradayVolatilityProfile {
    buckets: usize,
    offset: i32,
    previous_close: Option<f64>,
    counts: Vec<usize>,
    means: Vec<f64>,
    second_moments: Vec<f64>,
    bins: Vec<f64>,
    available: bool,
}

impl IntradayVolatilityProfile {
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
            previous_close: None,
            counts: vec![0; buckets],
            means: vec![0.0; buckets],
            second_moments: vec![0.0; buckets],
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
        close: f64,
        _volume: f64,
        timestamp: i64,
    ) -> Option<IntradayVolatilityProfileValue<'_>> {
        let previous = self.previous_close.replace(close)?;
        let bucket = self.bucket(timestamp);
        let return_value = if previous != 0.0 {
            close / previous - 1.0
        } else {
            0.0
        };

        self.counts[bucket] += 1;
        let count = self.counts[bucket] as f64;
        let delta = return_value - self.means[bucket];
        self.means[bucket] += delta / count;
        let delta_after_mean = return_value - self.means[bucket];
        self.second_moments[bucket] += delta * delta_after_mean;
        self.bins[bucket] = if self.counts[bucket] > 1 {
            (self.second_moments[bucket] / (count - 1.0)).sqrt()
        } else {
            0.0
        };
        self.available = true;
        self.value()
    }

    /// Return all current bucket volatilities after the first return exists.
    pub fn value(&self) -> Option<IntradayVolatilityProfileValue<'_>> {
        self.available
            .then_some(IntradayVolatilityProfileValue { bins: &self.bins })
    }

    /// Restore the profile to its newly constructed state without reallocating.
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.counts.fill(0);
        self.means.fill(0.0);
        self.second_moments.fill(0.0);
        self.bins.fill(0.0);
        self.available = false;
    }
}
