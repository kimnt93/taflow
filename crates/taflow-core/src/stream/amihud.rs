//! Batch implementation for `amihud`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `amihud` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the amihud result for the supplied aligned series.
///
/// # Parameters
///
/// * `close` - Input series or configuration value.
/// * `volume` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn amihud(close: &[f64], volume: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if close.len() != volume.len() {
        return Err(TaError::LengthMismatch {
            expected: close.len(),
            got: volume.len(),
        });
    }
    let mut state = Amihud::new(timeperiod)?;
    Ok(close
        .iter()
        .zip(volume)
        .map(|(&close, &volume)| state.append(close, volume).unwrap_or(f64::NAN))
        .collect())
}
