//! Persistent cumulative minimum state.

/// Cumulative minimum of scalar observations.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `CumulativeMinimum`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CumulativeMinimum {
    extreme: f64,
    value: Option<f64>,
}

impl CumulativeMinimum {
    /// Create an empty cumulative minimum.
    pub fn new() -> Self {
        Self {
            extreme: f64::INFINITY,
            value: None,
        }
    }

    /// Append one observation and return the cumulative minimum.
    pub fn append(&mut self, input: f64) -> f64 {
        self.extreme = self.extreme.min(input);
        self.value = Some(self.extreme);
        self.extreme
    }

    /// Return the latest cumulative minimum.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Reset the accumulated minimum.
    pub fn reset(&mut self) {
        self.extreme = f64::INFINITY;
        self.value = None;
    }
}

impl Default for CumulativeMinimum {
    fn default() -> Self {
        Self::new()
    }
}

/// Computes the running minimum of an aligned numeric series.
///
/// # Parameters
/// `input`: numeric observations in chronological order.
///
/// # Returns
/// Compute the cumulative minimum result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn cumulative_minimum(input: &[f64]) -> Vec<f64> {
    let mut minimum = f64::INFINITY;
    input
        .iter()
        .map(|&value| {
            minimum = minimum.min(value);
            minimum
        })
        .collect()
}
