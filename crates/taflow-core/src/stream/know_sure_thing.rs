//! Batch implementation for `know_sure_thing`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes the causal know sure thing series.
/// Parameters: aligned input slices followed by indicator parameters.
/// Returns: an aligned series, with NaN during warm-up, or a parameter error.
pub fn know_sure_thing(
    close: &[f64],
    roc1: usize,
    roc2: usize,
    roc3: usize,
    roc4: usize,
    sma1: usize,
    sma2: usize,
    sma3: usize,
    sma4: usize,
    nsig: usize,
) -> TaResult<(Vec<f64>, Vec<f64>)> {
    let mut state = KnowSureThing::new(roc1, roc2, roc3, roc4, sma1, sma2, sma3, sma4, nsig)?;
    let mut kst_out = Vec::with_capacity(close.len());
    let mut signal = Vec::with_capacity(close.len());
    for &close in close {
        let value = state.append(close);
        kst_out.push(value.kst);
        signal.push(value.signal);
    }
    Ok((kst_out, signal))
}
