//! Batch implementation for `fair_value_gap`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes the causal fair value gap series.
/// Parameters: aligned input slices followed by indicator parameters.
/// Returns: an aligned series, with NaN during warm-up, or a parameter error.
pub fn fair_value_gap(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> {
    if open.len() != high.len() || high.len() != low.len() || low.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: open.len(),
            got: high.len().max(low.len()).max(close.len()),
        });
    }
    let mut state = FairValueGap::new();
    let mut signal = Vec::with_capacity(open.len());
    let mut top = Vec::with_capacity(open.len());
    let mut bottom = Vec::with_capacity(open.len());
    let mut mitigated = Vec::with_capacity(open.len());
    for (((&open, &high), &low), &close) in open.iter().zip(high).zip(low).zip(close) {
        let value = state
            .append(open, high, low, close)
            .expect("FVG always emits an aligned value");
        signal.push(value.signal);
        top.push(value.top);
        bottom.push(value.bottom);
        mitigated.push(value.mitigated);
    }
    Ok((signal, top, bottom, mitigated))
}
