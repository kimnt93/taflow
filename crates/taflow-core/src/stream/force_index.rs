//! Batch implementation for `force_index`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Compute the force index from aligned close and volume series.
///
/// The result is input-aligned and uses `NaN` while the previous close is
/// Compute the force index result for the supplied aligned series.
///
/// # Parameters
///
/// * `close` - Input series or configuration value.
/// * `volume` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn force_index(close: &[f64], volume: &[f64]) -> TaResult<Vec<f64>> {
    if close.len() != volume.len() {
        return Err(TaError::LengthMismatch {
            expected: close.len(),
            got: volume.len(),
        });
    }
    let mut state = ForceIndex::new();
    Ok(close
        .iter()
        .zip(volume)
        .map(|(&c, &v)| state.append(c, v).unwrap_or(f64::NAN))
        .collect())
}
