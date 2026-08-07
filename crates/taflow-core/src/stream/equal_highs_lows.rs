//! Batch implementation for `equal_highs_lows`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `equal_highs_lows` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn equal_highs_lows(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    eq_len: usize,
    atr_period: usize,
    eq_threshold: f64,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() || low.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().max(close.len()),
        });
    }
    let mut state = EqualHighsLows::new(eq_len, atr_period, eq_threshold)?;
    let mut eqh = Vec::with_capacity(high.len());
    let mut eql = Vec::with_capacity(high.len());
    let mut level = Vec::with_capacity(high.len());
    for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
        let value = state.append(high, low, close);
        eqh.push(value.eqh);
        eql.push(value.eql);
        level.push(value.level);
    }
    Ok((eqh, eql, level))
}
