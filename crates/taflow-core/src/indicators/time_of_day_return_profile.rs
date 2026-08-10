use crate::error::{TaError, TaResult};

#[derive(Debug, Clone)]
pub struct TimeOfDayReturnProfile {
    buckets: usize,
    offset_minutes: i32,
    sums: Vec<f64>,
    counts: Vec<usize>,
    previous_close: Option<f64>,
    value: Option<f64>,
}
impl TimeOfDayReturnProfile {
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
            offset_minutes: utc_offset_minutes,
            sums: vec![0.0; buckets],
            counts: vec![0; buckets],
            previous_close: None,
            value: None,
        })
    }
    fn bucket(&self, timestamp: i64) -> usize {
        let shifted = timestamp + self.offset_minutes as i64 * 60_000_000_000;
        let within = shifted.rem_euclid(86_400_000_000_000) as u128;
        ((within * self.buckets as u128) / 86_400_000_000_000u128) as usize
    }
    pub fn append(
        &mut self,
        _open: f64,
        _high: f64,
        _low: f64,
        close: f64,
        _volume: f64,
        timestamp: i64,
    ) -> Option<f64> {
        let bucket = self.bucket(timestamp);
        self.value = self.previous_close.filter(|x| *x != 0.0).map(|previous| {
            self.sums[bucket] += close / previous - 1.0;
            self.counts[bucket] += 1;
            self.sums[bucket] / self.counts[bucket] as f64
        });
        self.previous_close = Some(close);
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.sums.fill(0.0);
        self.counts.fill(0);
        self.previous_close = None;
        self.value = None;
    }
}
