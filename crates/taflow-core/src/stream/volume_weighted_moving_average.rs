//! Batch implementation for `volume_weighted_moving_average`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes the causal volume weighted moving average series.
/// Parameters: aligned input slices followed by indicator parameters.
/// Compute the volume weighted moving average result for the supplied aligned series.
///
/// # Parameters
///
/// * `price` - Input series or configuration value.
/// * `volume` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn volume_weighted_moving_average(
    price: &[f64],
    volume: &[f64],
    timeperiod: usize,
) -> TaResult<Vec<f64>> {
    if price.len() != volume.len() {
        return Err(TaError::LengthMismatch {
            expected: price.len(),
            got: volume.len(),
        });
    }
    let mut state = VolumeWeightedMovingAverage::new(timeperiod)?;
    Ok(price
        .iter()
        .zip(volume)
        .map(|(&p, &v)| state.append(p, v).unwrap_or(f64::NAN))
        .collect())
}
