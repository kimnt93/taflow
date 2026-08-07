//! Batch implementation for `ichimoku`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `ichimoku` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn ichimoku(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    tenkan: usize,
    kijun: usize,
    senkou: usize,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() || high.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().min(close.len()),
        });
    }
    let mut state = Ichimoku::new(tenkan, kijun, senkou)?;
    let mut tenkan_sen = Vec::with_capacity(high.len());
    let mut kijun_sen = Vec::with_capacity(high.len());
    let mut span_a = Vec::with_capacity(high.len());
    let mut span_b = Vec::with_capacity(high.len());
    let mut chikou_span = Vec::with_capacity(high.len());
    for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
        let value = state.append(high, low, close);
        tenkan_sen.push(value.tenkan_sen);
        kijun_sen.push(value.kijun_sen);
        span_a.push(value.span_a);
        span_b.push(value.span_b);
        chikou_span.push(value.chikou_span);
    }
    Ok((tenkan_sen, kijun_sen, span_a, span_b, chikou_span))
}
