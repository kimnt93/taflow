use crate::error::TaResult;
use crate::stream::StreamingIndicator;
#[derive(Debug, Clone)]
pub struct DecyclerOscillator {
    fast_alpha: f64,
    slow_alpha: f64,
    fast: Option<f64>,
    slow: Option<f64>,
    value: Option<f64>,
}
impl DecyclerOscillator {
    pub fn new(fast: usize, slow: usize) -> TaResult<Self> {
        Ok(Self {
            fast_alpha: 2.0 / (fast as f64 + 1.0),
            slow_alpha: 2.0 / (slow as f64 + 1.0),
            fast: None,
            slow: None,
            value: None,
        })
    }
    pub fn append(&mut self, x: f64) -> Option<f64> {
        let f = self.fast.map_or(x, |v| v + self.fast_alpha * (x - v));
        let s = self.slow.map_or(x, |v| v + self.slow_alpha * (x - v));
        self.fast = Some(f);
        self.slow = Some(s);
        self.value = Some(f - s);
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.fast = None;
        self.slow = None;
        self.value = None;
    }
}
impl StreamingIndicator for DecyclerOscillator {
    type Output = f64;
    fn append(&mut self, x: f64) -> Option<f64> {
        Self::append(self, x)
    }
    fn value(&self) -> Option<f64> {
        self.value
    }
    fn reset(&mut self) {
        Self::reset(self)
    }
}
