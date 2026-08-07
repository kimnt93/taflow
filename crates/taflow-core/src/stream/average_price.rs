//! Batch implementation for `average_price`.

use super::price_transform::*;
use crate::error::{TaError, TaResult};

/// Compute the average price result for the supplied aligned series.
///
/// # Parameters
///
/// * `open` - Input series or configuration value.
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `close` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn average_price(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<f64>> {
    let len = open.len();
    validate_ohlc_len(len, high, low, close)?;
    let mut output = Vec::with_capacity(len);
    output.extend(
        open.iter()
            .zip(high.iter())
            .zip(low.iter())
            .zip(close.iter())
            .map(|(((&o, &h), &l), &c)| (o + h + l + c) * 0.25),
    );
    Ok(output)
}
