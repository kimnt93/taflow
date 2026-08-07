//! Batch implementation for `sessions`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `sessions` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn sessions(
    new_session: &[bool],
    high: &[f64],
    low: &[f64],
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if new_session.len() != high.len() || high.len() != low.len() {
        return Err(TaError::LengthMismatch {
            expected: new_session.len(),
            got: high.len().max(low.len()),
        });
    }
    let mut state = Sessions::new();
    let mut active = Vec::with_capacity(high.len());
    let mut session_high = Vec::with_capacity(high.len());
    let mut session_low = Vec::with_capacity(high.len());
    for ((&new_session, &high), &low) in new_session.iter().zip(high).zip(low) {
        let value = state.append(new_session, high, low);
        active.push(value.active);
        session_high.push(value.session_high);
        session_low.push(value.session_low);
    }
    Ok((active, session_high, session_low))
}
