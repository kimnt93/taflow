//! Persistent cumulative product state.

/// Cumulative product of scalar observations.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `CumulativeProduct`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CumulativeProduct {
    total: f64,
    value: Option<f64>,
}

impl CumulativeProduct {
    /// Create an empty cumulative product.
    pub fn new() -> Self {
        Self {
            total: 1.0,
            value: None,
        }
    }

    /// Append one observation and return the cumulative product.
    pub fn append(&mut self, input: f64) -> f64 {
        self.total *= input;
        self.value = Some(self.total);
        self.total
    }

    /// Return the latest cumulative product.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Reset the accumulated product.
    pub fn reset(&mut self) {
        self.total = 1.0;
        self.value = None;
    }
}

impl Default for CumulativeProduct {
    fn default() -> Self {
        Self::new()
    }
}

/// Computes the prefix product of an aligned numeric series.
///
/// `input` is consumed in chronological order and every output element is
/// the product through the corresponding bar. The output has the same length
/// as `input`; no warm-up values are required.
///
/// # Parameters
/// `input`: numeric observations in chronological order.
///
/// # Returns
/// Compute the cumulative product result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn cumulative_product(input: &[f64]) -> Vec<f64> {
    let mut total = 1.0;
    input
        .iter()
        .map(|&value| {
            total *= value;
            total
        })
        .collect()
}
