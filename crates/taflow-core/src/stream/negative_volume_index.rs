//! Batch implementation for `negative_volume_index`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes the causal negative volume index series.
/// Parameters: aligned input slices followed by indicator parameters.
/// Compute the negative volume index result for the supplied aligned series.
///
/// # Parameters
///
/// * `close` - Input series or configuration value.
/// * `volume` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn negative_volume_index(close: &[f64], volume: &[f64]) -> TaResult<Vec<f64>> {
    if close.len() != volume.len() {
        return Err(TaError::LengthMismatch {
            expected: close.len(),
            got: volume.len(),
        });
    }
    let mut state = VolumeIndex::new(VolumeIndexMode::Negative);
    Ok(close
        .iter()
        .zip(volume)
        .map(|(&c, &v)| state.append(c, v))
        .collect())
}
