use super::swing_high_low::{SwingHighLow, SwingValue};
use crate::error::TaResult;

#[derive(Debug, Clone)]
/// Canonical swing-high adapter backed by the causal swing state.
pub struct SwingHigh {
    inner: SwingHighLow,
}

impl SwingHigh {
    pub fn new(length: usize) -> TaResult<Self> {
        Ok(Self {
            inner: SwingHighLow::new(length)?,
        })
    }
    pub fn append(&mut self, high: f64, low: f64) -> Option<SwingValue> {
        self.inner.append(high, low)
    }
    pub fn value(&self) -> Option<SwingValue> {
        self.inner.value()
    }
    pub fn reset(&mut self) {
        self.inner.reset();
    }
}
