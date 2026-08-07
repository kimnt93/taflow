//! Batch implementation for `median_price`.

use super::price_transform::*;
use crate::error::{TaError, TaResult};

/// Compute the median price result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn median_price(high: &[f64], low: &[f64]) -> TaResult<Vec<f64>> {
    let len = high.len();
    if len != low.len() {
        return Err(TaError::LengthMismatch {
            expected: len,
            got: low.len(),
        });
    }
    let mut output = Vec::with_capacity(len);
    output.extend(high.iter().zip(low.iter()).map(|(&h, &l)| (h + l) * 0.5));
    Ok(output)
}
