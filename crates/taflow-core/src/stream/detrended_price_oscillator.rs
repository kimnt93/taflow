//! Batch implementation for `detrended_price_oscillator`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes the causal detrended price oscillator series.
/// Parameters: aligned input slices followed by indicator parameters.
/// Compute the detrended price oscillator result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `period` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn detrended_price_oscillator(input: &[f64], period: usize) -> TaResult<Vec<f64>> {
    let mut state = DetrendedPriceOscillator::new(period)?;
    Ok(input
        .iter()
        .map(|&value| state.append(value).unwrap_or(f64::NAN))
        .collect())
}
