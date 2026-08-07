//! Batch implementation for `frac_diff`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `frac_diff` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn frac_diff(input: &[f64], d: f64, threshold: f64) -> TaResult<Vec<f64>> {
    let mut state = FracDiff::new(d, threshold)?;
    Ok(input
        .iter()
        .map(|&value| state.append(value).unwrap_or(f64::NAN))
        .collect())
}
