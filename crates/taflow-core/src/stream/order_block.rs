//! Batch implementation for `order_block`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes the causal order block series.
/// Parameters: aligned input slices followed by indicator parameters.
/// Returns: an aligned series, with NaN during warm-up, or a parameter error.
pub fn order_block(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    volume: &[f64],
    swing_length: usize,
    internal_length: usize,
    atr_period: usize,
    threshold: f64,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() || low.len() != close.len() || close.len() != volume.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().max(close.len()).max(volume.len()),
        });
    }
    let mut state = OrderBlock::new(swing_length, internal_length, atr_period, threshold)?;
    let mut ob_out = Vec::with_capacity(high.len());
    let mut top = Vec::with_capacity(high.len());
    let mut bottom = Vec::with_capacity(high.len());
    let mut ob_volume = Vec::with_capacity(high.len());
    let mut mitigated = Vec::with_capacity(high.len());
    for ((((&high, &low), &close), &volume), _) in high
        .iter()
        .zip(low)
        .zip(close)
        .zip(volume)
        .zip(std::iter::repeat(()))
    {
        let value = state.append(high, low, close, volume);
        ob_out.push(value.ob);
        top.push(value.top);
        bottom.push(value.bottom);
        ob_volume.push(value.ob_volume);
        mitigated.push(value.mitigated);
    }
    Ok((ob_out, top, bottom, ob_volume, mitigated))
}
