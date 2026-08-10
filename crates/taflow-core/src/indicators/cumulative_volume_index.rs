use crate::error::TaResult;
#[derive(Debug, Clone, Default)]
pub struct CumulativeVolumeIndex {
    total: f64,
    value: Option<f64>,
}
impl CumulativeVolumeIndex {
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
        self.total += volume * change.signum();
        self.value = Some(self.total);
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.total = 0.0;
        self.value = None;
    }
}
