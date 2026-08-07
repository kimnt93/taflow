//! Batch implementation for `rolling_autocorr`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `rolling_autocorr` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the rolling autocorr result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_autocorr(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = RollingAutocorr::new(timeperiod)?;
    Ok(input
        .iter()
        .map(|&value| state.append(value).unwrap_or(f64::NAN))
        .collect())
}
