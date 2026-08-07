//! Batch implementation for `accumulation_distribution_oscillator`.

use super::volume_states::*;
use crate::error::{TaError, TaResult};

/// Compute the accumulation distribution oscillator result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `close` - Input series or configuration value.
/// * `volume` - Input series or configuration value.
/// * `fastperiod` - Input series or configuration value.
/// * `slowperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn accumulation_distribution_oscillator(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    volume: &[f64],
    fastperiod: usize,
    slowperiod: usize,
) -> TaResult<Vec<f64>> {
    if high.len() != low.len() || high.len() != close.len() || high.len() != volume.len() {
        return Err(crate::TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().min(close.len()).min(volume.len()),
        });
    }
    let mut state = AccumulationDistributionOscillator::new(fastperiod, slowperiod)?;
    Ok(high
        .iter()
        .zip(low)
        .zip(close)
        .zip(volume)
        .map(|(((&high, &low), &close), &volume)| {
            state.append(high, low, close, volume).unwrap_or(f64::NAN)
        })
        .collect())
}
