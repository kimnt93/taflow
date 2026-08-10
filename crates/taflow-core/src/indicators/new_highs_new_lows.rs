use crate::error::TaResult;
#[derive(Debug, Clone, Default)]
pub struct NewHighsNewLows {
    count: usize,
    value: Option<f64>,
}
impl NewHighsNewLows {
    pub fn new() -> TaResult<Self> {
        Ok(Self::default())
    }
    pub fn append(
        &mut self,
        _change: f64,
        _volume: f64,
        new_high: f64,
        new_low: f64,
    ) -> Option<f64> {
        self.count += 1;
        self.value = Some(new_high - new_low);
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn len(&self) -> usize {
        self.count
    }
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    pub fn reset(&mut self) {
        self.count = 0;
        self.value = None;
    }
}
