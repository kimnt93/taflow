//! Batch implementation for `session_extrema`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Running high and low values reset by an explicit session boundary.
///
/// The boundary is supplied as an aligned boolean input. The first bar is
/// treated as the beginning of a session when `new_session` is false.
pub fn session_extrema(
    new_session: &[bool],
    high: &[f64],
    low: &[f64],
) -> TaResult<(Vec<f64>, Vec<f64>)> {
    if new_session.len() != high.len() || high.len() != low.len() {
        return Err(TaError::LengthMismatch {
            expected: new_session.len(),
            got: high.len().max(low.len()),
        });
    }
    let mut state = SessionExtrema::new();
    let mut session_high = Vec::with_capacity(high.len());
    let mut session_low = Vec::with_capacity(low.len());
    for ((&new_session, &high), &low) in new_session.iter().zip(high).zip(low) {
        let value = state.append(new_session, high, low);
        session_high.push(value.high);
        session_low.push(value.low);
    }
    Ok((session_high, session_low))
}
