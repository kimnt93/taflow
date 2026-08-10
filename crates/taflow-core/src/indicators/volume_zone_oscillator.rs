use crate::error::TaResult;

#[derive(Debug, Clone)]
pub struct VolumeZoneOscillator {
    previous_close: Option<f64>,
    positive: f64,
    total: f64,
    value: Option<f64>,
}
impl VolumeZoneOscillator {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        if timeperiod == 0 {
            return Err(crate::error::TaError::InvalidParameter {
                name: "timeperiod",
                value: "0".into(),
                reason: "must be >= 1",
            });
        }
        Ok(Self {
            previous_close: None,
            positive: 0.0,
            total: 0.0,
            value: None,
        })
    }
    pub fn append(&mut self, close: f64, volume: f64) -> Option<f64> {
        let signed = self.previous_close.map_or(0.0, |p| {
            if close > p {
                volume
            } else if close < p {
                -volume
            } else {
                0.0
            }
        });
        self.previous_close = Some(close);
        self.positive += signed;
        self.total += volume.abs();
        self.value = (self.total > 0.0).then_some(100.0 * self.positive / self.total);
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.positive = 0.0;
        self.total = 0.0;
        self.value = None;
    }
}
