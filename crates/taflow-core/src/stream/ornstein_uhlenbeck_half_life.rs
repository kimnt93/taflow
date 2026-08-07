//! Batch implementation for `ornstein_uhlenbeck_half_life`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes the Ornstein-Uhlenbeck mean-reversion half-life.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn ornstein_uhlenbeck_half_life(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = OrnsteinUhlenbeckHalfLife::new(timeperiod)?;
    Ok(input
        .iter()
        .map(|&price| state.append(price).unwrap_or(f64::NAN))
        .collect())
}
