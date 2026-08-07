//! Batch implementation for `liquidity`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `liquidity` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the parkinson result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn liquidity(
    high: &[f64],
    low: &[f64],
    swing_length: usize,
    range_percent: f64,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len(),
        });
    }
    let mut state = Liquidity::new(swing_length, range_percent)?;
    let mut liquidity_out = Vec::with_capacity(high.len());
    let mut level = Vec::with_capacity(high.len());
    let mut swept = Vec::with_capacity(high.len());
    for (&high, &low) in high.iter().zip(low) {
        let value = state.append(high, low, f64::NAN);
        liquidity_out.push(value.liquidity);
        level.push(value.level);
        swept.push(value.swept);
    }
    Ok((liquidity_out, level, swept))
}
