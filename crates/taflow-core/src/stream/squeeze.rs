//! Batch implementation for `squeeze`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `squeeze` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn squeeze(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    bb_length: usize,
    bb_std: f64,
    kc_length: usize,
    kc_scalar: f64,
    mom_length: usize,
    mom_smooth: usize,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() || high.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().min(close.len()),
        });
    }
    let mut state = Squeeze::new(
        bb_length, bb_std, kc_length, kc_scalar, mom_length, mom_smooth,
    )?;
    let mut out = (0..4)
        .map(|_| Vec::with_capacity(high.len()))
        .collect::<Vec<_>>();
    for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
        let value = state.append(high, low, close);
        out[0].push(value.squeeze);
        out[1].push(value.on);
        out[2].push(value.off);
        out[3].push(value.no);
    }
    let mut out = out.into_iter();
    Ok((
        out.next().unwrap(),
        out.next().unwrap(),
        out.next().unwrap(),
        out.next().unwrap(),
    ))
}
