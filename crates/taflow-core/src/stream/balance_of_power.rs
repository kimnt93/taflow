//! Batch implementation for `balance_of_power`.

use super::volume_states::*;
use crate::error::{TaError, TaResult};

/// Compute the balance of power result for the supplied aligned series.
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
pub fn balance_of_power(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<f64>> {
    if open.len() != high.len() || open.len() != low.len() || open.len() != close.len() {
        return Err(crate::TaError::LengthMismatch {
            expected: open.len(),
            got: high.len().min(low.len()).min(close.len()),
        });
    }
    let mut state = BalanceOfPower::new();
    Ok(open
        .iter()
        .zip(high)
        .zip(low)
        .zip(close)
        .map(|(((&open, &high), &low), &close)| state.append(open, high, low, close))
        .collect())
}
