//! Batch implementation for `fractal_dimension`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `fractal_dimension` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the fractal dimension result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn fractal_dimension(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = Hurst::new(timeperiod)?;
    Ok(input
        .iter()
        .map(|&value| {
            state
                .append(value)
                .map(|hurst| 2.0 - hurst)
                .unwrap_or(f64::NAN)
        })
        .collect())
}
