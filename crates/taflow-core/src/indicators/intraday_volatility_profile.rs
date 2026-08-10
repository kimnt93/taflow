use crate::error::{TaError, TaResult};
#[derive(Debug, Clone)]
pub struct IntradayVolatilityProfile {
    buckets: usize,
    offset: i32,
    sums: Vec<f64>,
    counts: Vec<usize>,
    previous: Option<f64>,
    value: Option<f64>,
}
impl IntradayVolatilityProfile {
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
            previous: None,
            value: None,
        })
    }
    fn bucket(&self, t: i64) -> usize {
        let x = (t + self.offset as i64 * 60_000_000_000).rem_euclid(86_400_000_000_000) as u128;
        ((x * self.buckets as u128) / 86_400_000_000_000u128) as usize
    }
    pub fn append(&mut self, _o: f64, _h: f64, _l: f64, c: f64, _v: f64, t: i64) -> Option<f64> {
        let b = self.bucket(t);
        self.value = self.previous.filter(|x| *x != 0.0).map(|p| {
            self.sums[b] += (c / p - 1.0).abs();
            self.counts[b] += 1;
            self.sums[b] / self.counts[b] as f64
        });
        self.previous = Some(c);
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.sums.fill(0.0);
        self.counts.fill(0);
        self.previous = None;
        self.value = None;
    }
}
