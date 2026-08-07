//! Persistent cumulative maximum state.

/// Cumulative maximum of scalar observations.
#[derive(Debug, Clone)]
pub struct CumulativeMaximum {
    extreme: f64,
    value: Option<f64>,
}

impl CumulativeMaximum {
    /// Create an empty cumulative maximum.
    pub fn new() -> Self {
        Self { extreme: f64::NEG_INFINITY, value: None }
    }

    /// Append one observation and return the cumulative maximum.
    pub fn append(&mut self, input: f64) -> f64 {
        self.extreme = self.extreme.max(input);
        self.value = Some(self.extreme);
        self.extreme
    }

    /// Return the latest cumulative maximum.
    pub fn value(&self) -> Option<f64> { self.value }

    /// Reset the accumulated maximum.
    pub fn reset(&mut self) {
        self.extreme = f64::NEG_INFINITY;
        self.value = None;
    }
}

impl Default for CumulativeMaximum {
    fn default() -> Self { Self::new() }
}
