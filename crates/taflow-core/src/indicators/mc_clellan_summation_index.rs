use crate::error::TaResult;
use crate::indicators::McClellanOscillator;
#[derive(Debug, Clone)]
pub struct McClellanSummationIndex {
    oscillator: McClellanOscillator,
    total: f64,
    count: usize,
    value: Option<f64>,
}
impl McClellanSummationIndex {
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            oscillator: McClellanOscillator::new()?,
            total: 0.0,
            count: 0,
            value: None,
        })
    }
    pub fn append(&mut self, change: f64, volume: f64, new_high: f64, new_low: f64) -> Option<f64> {
        self.count += 1;
        if let Some(x) = self.oscillator.append(change, volume, new_high, new_low) {
            self.total += x;
            self.value = Some(self.total);
        }
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
        self.oscillator.reset();
        self.total = 0.0;
        self.count = 0;
        self.value = None;
    }
}
