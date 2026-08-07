//! Batch implementation for `ewm_cov`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `ewm_cov` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the ewm cov result for the supplied aligned series.
///
/// # Parameters
///
/// * `input0` - Input series or configuration value.
/// * `input1` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn ewm_cov(input0: &[f64], input1: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if input0.len() != input1.len() {
        return Err(TaError::LengthMismatch {
            expected: input0.len(),
            got: input1.len(),
        });
    }
    let mut state = ExponentiallyWeightedCovariance::new(timeperiod)?;
    Ok(input0
        .iter()
        .zip(input1)
        .map(|(&left, &right)| state.append(left, right))
        .collect())
}
