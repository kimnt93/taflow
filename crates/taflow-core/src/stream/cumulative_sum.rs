//! Persistent cumulative sum state.

/// Cumulative sum of scalar observations.
#[derive(Debug, Clone)]
pub struct CumulativeSum {
    total: f64,
    value: Option<f64>,
}

impl CumulativeSum {
    /// Create an empty cumulative sum.
    pub fn new() -> Self {
        Self { total: 0.0, value: None }
    }

    /// Append one observation and return the cumulative sum.
    pub fn append(&mut self, input: f64) -> f64 {
        self.total += input;
        self.value = Some(self.total);
        self.total
    }

    /// Return the latest cumulative sum.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Reset the accumulated total.
    pub fn reset(&mut self) {
        self.total = 0.0;
        self.value = None;
    }
}

impl Default for CumulativeSum {
    fn default() -> Self { Self::new() }
}
