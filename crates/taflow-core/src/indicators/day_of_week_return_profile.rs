use crate::error::TaResult;

#[derive(Debug, Clone)]
pub struct DayOfWeekReturnProfile {
    offset_minutes: i32,
    sums: [f64; 7],
    counts: [usize; 7],
    previous_close: Option<f64>,
    value: Option<f64>,
}
impl DayOfWeekReturnProfile {
    pub fn new(utc_offset_minutes: i32) -> TaResult<Self> {
        Ok(Self {
            offset_minutes: utc_offset_minutes,
            sums: [0.0; 7],
            counts: [0; 7],
            previous_close: None,
            value: None,
        })
    }
    fn weekday(&self, timestamp: i64) -> usize {
        let day = (timestamp + self.offset_minutes as i64 * 60_000_000_000)
            .div_euclid(86_400_000_000_000);
        (day + 3).rem_euclid(7) as usize
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
        let day = self.weekday(timestamp);
        self.value = self.previous_close.filter(|x| *x != 0.0).map(|previous| {
            self.sums[day] += close / previous - 1.0;
            self.counts[day] += 1;
            self.sums[day] / self.counts[day] as f64
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
