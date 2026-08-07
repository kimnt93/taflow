//! Batch implementation for `volume_price_trend`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes the causal volume price trend series.
/// Parameters: aligned input slices followed by indicator parameters.
/// Compute the volume price trend result for the supplied aligned series.
///
/// # Parameters
///
/// * `close` - Input series or configuration value.
/// * `volume` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn volume_price_trend(close: &[f64], volume: &[f64]) -> TaResult<Vec<f64>> {
    if close.len() != volume.len() {
        return Err(TaError::LengthMismatch {
            expected: close.len(),
            got: volume.len(),
        });
    }
    let mut state = VolumePriceTrend::new();
    Ok(close
        .iter()
        .zip(volume)
        .map(|(&close, &volume)| state.append(close, volume).unwrap_or(f64::NAN))
        .collect())
}
