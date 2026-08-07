//! Batch implementation for `accumulation_distribution`.

use super::volume_states::*;
use crate::error::{TaError, TaResult};

/// Compute the accumulation distribution result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `close` - Input series or configuration value.
/// * `volume` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn accumulation_distribution(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    volume: &[f64],
) -> TaResult<Vec<f64>> {
    if high.len() != low.len() || high.len() != close.len() || high.len() != volume.len() {
        return Err(crate::TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().min(close.len()).min(volume.len()),
        });
    }
    let mut state = AccumulationDistribution::new();
    Ok(high
        .iter()
        .zip(low)
        .zip(close)
        .zip(volume)
        .map(|(((&high, &low), &close), &volume)| state.append(high, low, close, volume))
        .collect())
}
