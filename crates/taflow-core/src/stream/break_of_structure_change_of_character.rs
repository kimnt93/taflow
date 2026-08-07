//! Batch implementation for `break_of_structure_change_of_character`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes break-of-structure and change-of-character events.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the close to close sigma result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn break_of_structure_change_of_character(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    swing_length: usize,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> {
    if high.len() != low.len() || low.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().max(close.len()),
        });
    }
    let mut state = BreakOfStructureChangeOfCharacter::new(swing_length)?;
    let mut bos = Vec::with_capacity(high.len());
    let mut choch = Vec::with_capacity(high.len());
    let mut level = Vec::with_capacity(high.len());
    let mut broken = Vec::with_capacity(high.len());
    for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
        let value = state.append(high, low, close);
        bos.push(value.bos);
        choch.push(value.choch);
        level.push(value.level);
        broken.push(value.broken);
    }
    Ok((bos, choch, level, broken))
}
