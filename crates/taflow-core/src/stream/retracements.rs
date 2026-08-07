//! Batch implementation for `retracements`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `retracements` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn retracements(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    swing_length: usize,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() || low.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().max(close.len()),
        });
    }
    let mut state = Retracements::new(swing_length)?;
    let mut direction = Vec::with_capacity(high.len());
    let mut current_retracement_pct = Vec::with_capacity(high.len());
    let mut deepest_retracement_pct = Vec::with_capacity(high.len());
    for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
        let value = state.append(high, low, close);
        direction.push(value.direction);
        current_retracement_pct.push(value.current_retracement_pct);
        deepest_retracement_pct.push(value.deepest_retracement_pct);
    }
    Ok((direction, current_retracement_pct, deepest_retracement_pct))
}
