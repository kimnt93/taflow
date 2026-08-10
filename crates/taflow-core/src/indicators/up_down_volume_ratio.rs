use crate::error::TaResult;
#[derive(Debug, Clone, Default)]
pub struct UpDownVolumeRatio {
    up: f64,
    down: f64,
    value: Option<f64>,
}
impl UpDownVolumeRatio {
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }
    pub fn append(
        &mut self,
        change: f64,
        volume: f64,
        _new_high: f64,
        _new_low: f64,
    ) -> Option<f64> {
        if change > 0.0 {
            self.up += volume
        } else if change < 0.0 {
            self.down += volume
        }
        self.value = (self.down != 0.0).then(|| self.up / self.down);
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.up = 0.0;
        self.down = 0.0;
        self.value = None;
    }
}
