//! Batch implementation for `rolling_winsorize`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `rolling_winsorize` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the rolling winsorize result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
/// * `lower` - Input series or configuration value.
/// * `upper` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_winsorize(
    input: &[f64],
    timeperiod: usize,
    lower: f64,
    upper: f64,
) -> TaResult<Vec<f64>> {
    let mut state = RollingWinsorize::new(timeperiod, lower, upper)?;
    Ok(input
        .iter()
        .map(|&value| state.append(value).unwrap_or(f64::NAN))
        .collect())
}
