use crate::error::TaResult;
#[derive(Debug, Clone)]
pub struct BetterVolume {
    value: Option<f64>,
}
impl BetterVolume {
    pub fn new() -> TaResult<Self> {
        Ok(Self { value: None })
    }
    pub fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> Option<f64> {
        let range = (high - low).abs();
        self.value = Some(if range == 0.0 {
            0.0
        } else {
            volume / range * (close - low) / (range)
        });
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.value = None;
    }
}
