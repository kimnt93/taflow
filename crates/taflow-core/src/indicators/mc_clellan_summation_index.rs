use crate::error::TaResult;
use crate::indicators::McClellanOscillator;
/// Running sum of the classic McClellan Oscillator.
#[derive(Debug, Clone)]
pub struct McClellanSummationIndex {
    oscillator: McClellanOscillator,
    total: f64,
    count: usize,
    value: Option<f64>,
}
impl McClellanSummationIndex {
    /// Create an empty oscillator and cumulative total.
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            oscillator: McClellanOscillator::new()?,
            total: 0.0,
            count: 0,
            value: None,
        })
    }
    /// Append aggregate advancing and declining issue counts.
    pub fn append(&mut self, advancers: f64, decliners: f64) -> Option<f64> {
        self.count += 1;
        if let Some(x) = self.oscillator.append(advancers, decliners) {
            self.total += x;
            self.value = Some(self.total);
        }
        self.value
    }
    /// Return the latest cumulative index value.
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Return the number of processed market ticks.
    pub fn len(&self) -> usize {
        self.count
    }
    /// Return whether no market ticks have been processed.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    /// Reset the embedded oscillator and cumulative total.
    pub fn reset(&mut self) {
        self.oscillator.reset();
        self.total = 0.0;
        self.count = 0;
        self.value = None;
    }
}
