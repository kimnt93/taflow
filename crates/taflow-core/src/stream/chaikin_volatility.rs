//! Batch implementation for `chaikin_volatility`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `chaikin_volatility` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the chaikin volatility result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
/// * `roc_period` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn chaikin_volatility(
    high: &[f64],
    low: &[f64],
    timeperiod: usize,
    roc_period: usize,
) -> TaResult<Vec<f64>> {
    if high.len() != low.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len(),
        });
    }
    let mut state = ChaikinVolatility::new(timeperiod, roc_period)?;
    Ok(high
        .iter()
        .zip(low)
        .map(|(&h, &l)| state.append(h, l).unwrap_or(f64::NAN))
        .collect())
}
