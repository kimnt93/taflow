use crate::error::TaResult;

#[derive(Debug, Clone)]
pub struct SessionVolumeWeightedAveragePrice {
    offset_minutes: i32,
    day: Option<i64>,
    weighted_sum: f64,
    volume_sum: f64,
    value: Option<f64>,
}
impl SessionVolumeWeightedAveragePrice {
    pub fn new(utc_offset_minutes: i32) -> TaResult<Self> {
        Ok(Self {
            offset_minutes: utc_offset_minutes,
            day: None,
            weighted_sum: 0.0,
            volume_sum: 0.0,
            value: None,
        })
    }
    fn day(&self, timestamp: i64) -> i64 {
        (timestamp + self.offset_minutes as i64 * 60_000_000_000).div_euclid(86_400_000_000_000)
    }
    pub fn append(
        &mut self,
        _open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        timestamp: i64,
    ) -> Option<f64> {
        let day = self.day(timestamp);
        if self.day != Some(day) {
            self.day = Some(day);
            self.weighted_sum = 0.0;
            self.volume_sum = 0.0;
        }
        self.weighted_sum += ((high + low + close) / 3.0) * volume;
        self.volume_sum += volume;
        self.value = (self.volume_sum != 0.0).then(|| self.weighted_sum / self.volume_sum);
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.day = None;
        self.weighted_sum = 0.0;
        self.volume_sum = 0.0;
        self.value = None;
    }
}
