//! Batch implementation for `roll_spread`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `roll_spread` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn roll_spread(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = RollSpread::new(timeperiod)?;
    Ok(input
        .iter()
        .map(|&price| state.append(price).unwrap_or(f64::NAN))
        .collect())
}
