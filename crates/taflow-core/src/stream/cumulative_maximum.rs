//! Persistent cumulative maximum state.

/// Cumulative maximum of scalar observations.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `CumulativeMaximum`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CumulativeMaximum {
    extreme: f64,
    value: Option<f64>,
}

impl CumulativeMaximum {
    /// Create an empty cumulative maximum.
    pub fn new() -> Self {
        Self {
            extreme: f64::NEG_INFINITY,
            value: None,
        }
    }

    /// Append one observation and return the cumulative maximum.
    pub fn append(&mut self, input: f64) -> f64 {
        self.extreme = self.extreme.max(input);
        self.value = Some(self.extreme);
        self.extreme
    }

    /// Return the latest cumulative maximum.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Reset the accumulated maximum.
    pub fn reset(&mut self) {
        self.extreme = f64::NEG_INFINITY;
        self.value = None;
    }
}

impl Default for CumulativeMaximum {
    fn default() -> Self {
        Self::new()
    }
}

/// Computes the running maximum of an aligned numeric series.
///
/// # Parameters
/// `input`: numeric observations in chronological order.
///
/// # Returns
/// Compute the cumulative maximum result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn cumulative_maximum(input: &[f64]) -> Vec<f64> {
    let mut maximum = f64::NEG_INFINITY;
    input
        .iter()
        .map(|&value| {
            maximum = maximum.max(value);
            maximum
        })
        .collect()
}
