use crate::error::TaResult;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DemandIndexValue {
    pub demand: f64,
}
#[derive(Debug, Clone)]
pub struct DemandIndex {
    previous_close: Option<f64>,
    value: Option<DemandIndexValue>,
}
impl DemandIndex {
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            previous_close: None,
            value: None,
        })
    }
    pub fn append(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
    ) -> Option<DemandIndexValue> {
        let range = (high - low).abs();
        let demand = if range > 0.0 {
            (close - low) / range * volume
        } else {
            0.0
        };
        self.previous_close = Some(close);
        let result = DemandIndexValue { demand };
        self.value = Some(result);
        self.value
    }
    pub fn value(&self) -> Option<DemandIndexValue> {
        self.value
    }
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.value = None;
    }
}
