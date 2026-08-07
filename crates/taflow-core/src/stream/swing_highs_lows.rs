//! Batch implementation for `swing_highs_lows`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Causal swing-point confirmation.
///
/// The center bar of a `2 * swing_length + 1` window is confirmed at the
/// current bar. A signal is emitted only after the required future bars have
/// arrived, so no output uses lookahead when it is observed.
pub fn swing_highs_lows(
    high: &[f64],
    low: &[f64],
    swing_length: usize,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len(),
        });
    }
    let mut state = SwingHighLow::new(swing_length)?;
    let mut signal = Vec::with_capacity(high.len());
    let mut level = Vec::with_capacity(high.len());
    let mut bars_since = Vec::with_capacity(high.len());
    for (&high, &low) in high.iter().zip(low) {
        let value = state.append(high, low);
        signal.push(value.map_or(f64::NAN, |value| value.signal));
        level.push(value.map_or(f64::NAN, |value| value.level));
        bars_since.push(value.map_or(f64::NAN, |value| value.bars_since));
    }
    Ok((signal, level, bars_since))
}
