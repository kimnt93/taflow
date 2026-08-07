//! Batch implementation for `previous_high_low`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `previous_high_low` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn previous_high_low(
    new_session: &[bool],
    high: &[f64],
    low: &[f64],
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> {
    if new_session.len() != high.len() || high.len() != low.len() {
        return Err(TaError::LengthMismatch {
            expected: new_session.len(),
            got: high.len().max(low.len()),
        });
    }
    let mut state = PreviousHighLow::new();
    let mut prev_high = Vec::with_capacity(high.len());
    let mut prev_low = Vec::with_capacity(high.len());
    let mut broken_high = Vec::with_capacity(high.len());
    let mut broken_low = Vec::with_capacity(high.len());
    for ((&new_session, &high), &low) in new_session.iter().zip(high).zip(low) {
        let value = state.append(new_session, high, low);
        prev_high.push(value.prev_high);
        prev_low.push(value.prev_low);
        broken_high.push(value.broken_high);
        broken_low.push(value.broken_low);
    }
    Ok((prev_high, prev_low, broken_high, broken_low))
}
