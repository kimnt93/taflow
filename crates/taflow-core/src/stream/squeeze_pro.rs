//! Batch implementation for `squeeze_pro`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `squeeze_pro` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn squeeze_pro(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    bb_length: usize,
    bb_std: f64,
    kc_length: usize,
    kc_scalar_wide: f64,
    kc_scalar_normal: f64,
    kc_scalar_narrow: f64,
    mom_length: usize,
    mom_smooth: usize,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() || high.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().min(close.len()),
        });
    }
    let mut state = SqueezePro::new(
        bb_length,
        bb_std,
        kc_length,
        kc_scalar_wide,
        kc_scalar_normal,
        kc_scalar_narrow,
        mom_length,
        mom_smooth,
    )?;
    let mut squeeze = Vec::with_capacity(high.len());
    let mut on_wide = Vec::with_capacity(high.len());
    let mut on_normal = Vec::with_capacity(high.len());
    let mut on_narrow = Vec::with_capacity(high.len());
    let mut off = Vec::with_capacity(high.len());
    let mut no = Vec::with_capacity(high.len());
    for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
        let value = state.append(high, low, close);
        squeeze.push(value.squeeze);
        on_wide.push(value.on_wide);
        on_normal.push(value.on_normal);
        on_narrow.push(value.on_narrow);
        off.push(value.off);
        no.push(value.no);
    }
    Ok((squeeze, on_wide, on_normal, on_narrow, off, no))
}
