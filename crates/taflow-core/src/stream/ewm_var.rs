//! Batch implementation for `ewm_var`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `ewm_var` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the ewm var result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn ewm_var(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = ExponentiallyWeightedVariance::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value)).collect())
}
