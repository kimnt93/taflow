use super::swing_high_low::{SwingHighLow, SwingValue};
use crate::error::TaResult;

/// Canonical plural swing-highs/lows state.
#[derive(Debug, Clone)]
pub struct SwingHighsLows {
    inner: SwingHighLow,
}

impl SwingHighsLows {
    /// Create a validated empty state.
    pub fn new(length: usize) -> TaResult<Self> {
        Ok(Self {
            inner: SwingHighLow::new(length)?,
        })
    }

    /// Append one high/low bar.
    pub fn append(&mut self, high: f64, low: f64) -> Option<SwingValue> {
        self.inner.append(high, low)
    }

    /// Return the latest confirmed swing value.
    pub fn value(&self) -> Option<SwingValue> {
        self.inner.value()
    }

    /// Reset the wrapped state.
    pub fn reset(&mut self) {
        self.inner.reset();
    }
}
