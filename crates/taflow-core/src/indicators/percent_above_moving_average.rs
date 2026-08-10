use crate::error::TaResult;
#[derive(Debug, Clone, Default)]
pub struct PercentAboveMovingAverage {
    value: Option<f64>,
}
impl PercentAboveMovingAverage {
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }
    pub fn append(
        &mut self,
        _change: f64,
        _volume: f64,
        _new_high: f64,
        _new_low: f64,
        above_moving_average: f64,
    ) -> Option<f64> {
        self.value = Some(above_moving_average.clamp(0.0, 1.0) * 100.0);
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.value = None;
    }
}
