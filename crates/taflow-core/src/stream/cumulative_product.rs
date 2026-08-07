//! Persistent cumulative product state.

/// Cumulative product of scalar observations.
#[derive(Debug, Clone)]
pub struct CumulativeProduct {
    total: f64,
    value: Option<f64>,
}

impl CumulativeProduct {
    /// Create an empty cumulative product.
    pub fn new() -> Self {
        Self { total: 1.0, value: None }
    }

    /// Append one observation and return the cumulative product.
    pub fn append(&mut self, input: f64) -> f64 {
        self.total *= input;
        self.value = Some(self.total);
        self.total
    }

    /// Return the latest cumulative product.
    pub fn value(&self) -> Option<f64> { self.value }

    /// Reset the accumulated product.
    pub fn reset(&mut self) {
        self.total = 1.0;
        self.value = None;
    }
}

impl Default for CumulativeProduct {
    fn default() -> Self { Self::new() }
}
