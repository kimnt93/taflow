//! Batch implementation for `schaff_trend_cycle`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes the causal schaff trend cycle series.
/// Parameters: aligned input slices followed by indicator parameters.
/// Returns: an aligned series, with NaN during warm-up, or a parameter error.
pub fn schaff_trend_cycle(
    close: &[f64],
    tclength: usize,
    fast: usize,
    slow: usize,
    factor: f64,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    let mut state = SchaffTrendCycle::new(tclength, fast, slow, factor)?;
    let mut stc_out = Vec::with_capacity(close.len());
    let mut macd = Vec::with_capacity(close.len());
    let mut stoch = Vec::with_capacity(close.len());
    state.extend_slices_into(close, &mut stc_out, &mut macd, &mut stoch);
    Ok((stc_out, macd, stoch))
}
