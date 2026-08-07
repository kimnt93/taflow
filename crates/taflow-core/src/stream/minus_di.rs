//! Incremental Minus Directional Indicator (-DI).
use super::directional::DirectionalMovement;
use crate::error::TaResult;
pub struct MinusDirectionalIndicator {
    directional: DirectionalMovement,
    value: Option<f64>,
}
impl MinusDirectionalIndicator {
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            directional: DirectionalMovement::new(period)?,
            value: None,
        })
    }
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        self.value = self
            .directional
            .append(high, low, close)
            .map(|v| v.minus_di);
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.directional.reset();
        self.value = None;
    }
}
