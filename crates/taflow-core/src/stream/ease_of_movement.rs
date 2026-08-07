//! Batch implementation for `ease_of_movement`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Compute ease of movement from aligned high, low, and volume series.
///
/// The returned series preserves input length and reports `NaN` during its
/// Compute the ease of movement result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `volume` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn ease_of_movement(high: &[f64], low: &[f64], volume: &[f64]) -> TaResult<Vec<f64>> {
    if high.len() != low.len() || high.len() != volume.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len(),
        });
    }
    let mut state = EaseOfMovement::new();
    Ok(high
        .iter()
        .zip(low)
        .zip(volume)
        .map(|((&h, &l), &v)| state.append(h, l, v).unwrap_or(f64::NAN))
        .collect())
}
