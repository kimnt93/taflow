//! Persistent cumulative sum state.

/// Cumulative sum of scalar observations.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `CumulativeSum`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CumulativeSum {
    total: f64,
    value: Option<f64>,
}

impl CumulativeSum {
    /// Create an empty cumulative sum.
    pub fn new() -> Self {
        Self {
            total: 0.0,
            value: None,
        }
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
    fn default() -> Self {
        Self::new()
    }
}

/// Computes the prefix sum of an aligned numeric series.
///
/// `input` is consumed in chronological order and every output element is
/// the sum through the corresponding bar. The output has the same length as
/// `input`; no warm-up values are required.
///
/// # Parameters
/// `input`: numeric observations in chronological order.
///
/// # Returns
/// Compute the cumulative sum result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn cumulative_sum(input: &[f64]) -> Vec<f64> {
    let mut total = 0.0;
    input
        .iter()
        .map(|&value| {
            total += value;
            total
        })
        .collect()
}
