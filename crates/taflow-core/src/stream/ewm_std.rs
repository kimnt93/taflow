//! Batch implementation for `ewm_std`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `ewm_std` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the ewm std result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn ewm_std(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = ExponentiallyWeightedStandardDeviation::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value)).collect())
}
