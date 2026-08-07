//! Batch implementation for `parkinson`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `parkinson` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn parkinson(high: &[f64], low: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if high.len() != low.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len(),
        });
    }
    let mut state = Parkinson::new(timeperiod)?;
    Ok(high
        .iter()
        .zip(low)
        .map(|(&high, &low)| state.append(high, low).unwrap_or(f64::NAN))
        .collect())
}
