//! Batch implementation for `supertrend`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `supertrend` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn supertrend(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    length: usize,
    multiplier: f64,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() || high.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().min(close.len()),
        });
    }
    let mut state = Supertrend::new(length, multiplier)?;
    let mut trend = Vec::with_capacity(high.len());
    let mut direction = Vec::with_capacity(high.len());
    let mut long = Vec::with_capacity(high.len());
    let mut short = Vec::with_capacity(high.len());
    for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
        match state.append(high, low, close) {
            Some(value) => {
                trend.push(value.trend);
                direction.push(value.direction);
                long.push(value.long);
                short.push(value.short);
            }
            None => {
                trend.push(f64::NAN);
                direction.push(f64::NAN);
                long.push(f64::NAN);
                short.push(f64::NAN);
            }
        }
    }
    Ok((trend, direction, long, short))
}
