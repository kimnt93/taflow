use crate::error::TaResult;
#[derive(Debug, Clone)]
pub struct TradeVolumeIndex {
    previous: Option<f64>,
    value: f64,
}
impl TradeVolumeIndex {
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            previous: None,
            value: 0.0,
        })
    }
    pub fn append(&mut self, close: f64, volume: f64) -> Option<f64> {
        if let Some(p) = self.previous {
            if close > p {
                self.value += volume
            } else if close < p {
                self.value -= volume
            }
        }
        self.previous = Some(close);
        Some(self.value)
    }
    pub fn value(&self) -> Option<f64> {
        self.previous.map(|_| self.value)
    }
    pub fn reset(&mut self) {
        self.previous = None;
        self.value = 0.0;
    }
}
