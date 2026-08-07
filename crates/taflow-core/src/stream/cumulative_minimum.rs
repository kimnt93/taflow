//! Persistent cumulative minimum state.

/// Cumulative minimum of scalar observations.
#[derive(Debug, Clone)]
pub struct CumulativeMinimum {
    extreme: f64,
    value: Option<f64>,
}

impl CumulativeMinimum {
    /// Create an empty cumulative minimum.
    pub fn new() -> Self {
        Self { extreme: f64::INFINITY, value: None }
    }

    /// Append one observation and return the cumulative minimum.
    pub fn append(&mut self, input: f64) -> f64 {
        self.extreme = self.extreme.min(input);
        self.value = Some(self.extreme);
        self.extreme
    }

    /// Return the latest cumulative minimum.
    pub fn value(&self) -> Option<f64> { self.value }

    /// Reset the accumulated minimum.
    pub fn reset(&mut self) {
        self.extreme = f64::INFINITY;
        self.value = None;
    }
}

impl Default for CumulativeMinimum {
    fn default() -> Self { Self::new() }
}
