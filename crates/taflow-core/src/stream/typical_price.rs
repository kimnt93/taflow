//! Batch implementation for `typical_price`.

use super::price_transform::*;
use crate::error::{TaError, TaResult};

/// Compute the typical price result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `close` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn typical_price(high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<f64>> {
    let len = high.len();
    if len != low.len() || len != close.len() {
        return Err(TaError::LengthMismatch {
            expected: len,
            got: low.len().min(close.len()),
        });
    }
    let one_third = 1.0 / 3.0;
    let mut output = Vec::with_capacity(len);
    output.extend(
        high.iter()
            .zip(low.iter())
            .zip(close.iter())
            .map(|((&h, &l), &c)| (h + l + c) * one_third),
    );
    Ok(output)
}
