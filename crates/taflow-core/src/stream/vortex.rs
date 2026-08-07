//! Batch implementation for `vortex`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `vortex` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn vortex(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    period: usize,
) -> TaResult<(Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() || high.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().min(close.len()),
        });
    }
    let mut state = Vortex::new(period)?;
    let mut vp = Vec::with_capacity(high.len());
    let mut vn = Vec::with_capacity(high.len());
    for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
        let value = state.append(high, low, close);
        vp.push(value.vp);
        vn.push(value.vn);
    }
    Ok((vp, vn))
}
