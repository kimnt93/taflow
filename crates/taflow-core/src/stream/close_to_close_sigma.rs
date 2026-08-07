//! Batch implementation for `close_to_close_sigma`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `close_to_close_sigma` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn close_to_close_sigma(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = CloseToCloseSigma::new(timeperiod)?;
    Ok(input
        .iter()
        .map(|&value| state.append(value).unwrap_or(f64::NAN))
        .collect())
}
