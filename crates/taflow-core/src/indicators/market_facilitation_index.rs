use crate::error::TaResult;
#[derive(Debug, Clone)]
pub struct MarketFacilitationIndex {
    value: Option<f64>,
}
impl MarketFacilitationIndex {
    pub fn new() -> TaResult<Self> {
        Ok(Self { value: None })
    }
    pub fn append(&mut self, h: f64, l: f64, v: f64) -> Option<f64> {
        self.value = Some(if v == 0.0 { 0.0 } else { (h - l) / v });
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.value = None;
    }
}
