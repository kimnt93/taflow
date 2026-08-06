//! Incremental Plus Directional Indicator (+DI).
use super::directional::DirectionalMovement;
use crate::error::TaResult;

pub struct PlusDi {
    directional: DirectionalMovement,
    value: Option<f64>,
}
impl PlusDi {
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            directional: DirectionalMovement::new(period)?,
            value: None,
        })
    }
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        self.value = self.directional.append(high, low, close).map(|v| v.plus_di);
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
