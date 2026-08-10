use crate::error::TaResult;
#[derive(Debug, Clone, Default)]
pub struct AbsoluteBreadthIndex {
    value: Option<f64>,
}
impl AbsoluteBreadthIndex {
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }
    pub fn append(
        &mut self,
        change: f64,
        _volume: f64,
        _new_high: f64,
        _new_low: f64,
    ) -> Option<f64> {
        self.value = Some(change.abs());
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.value = None;
    }
}
