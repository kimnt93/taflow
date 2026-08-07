//! Batch implementation for `mass_index`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `mass_index` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn mass_index(
    high: &[f64],
    low: &[f64],
    ema_period: usize,
    sum_period: usize,
) -> TaResult<Vec<f64>> {
    if high.len() != low.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len(),
        });
    }
    let mut state = MassIndex::new(ema_period, sum_period)?;
    Ok(high
        .iter()
        .zip(low)
        .map(|(&h, &l)| state.append(h, l).unwrap_or(f64::NAN))
        .collect())
}
