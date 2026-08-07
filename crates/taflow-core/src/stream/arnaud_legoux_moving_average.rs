//! Batch implementation for `arnaud_legoux_moving_average`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes the causal arnaud legoux moving average series.
/// Parameters: aligned input slices followed by indicator parameters.
/// Compute the arnaud legoux moving average result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
/// * `offset` - Input series or configuration value.
/// * `sigma` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn arnaud_legoux_moving_average(
    input: &[f64],
    timeperiod: usize,
    offset: f64,
    sigma: f64,
) -> TaResult<Vec<f64>> {
    let mut state = ArnaudLegouxMovingAverage::new(timeperiod, offset, sigma)?;
    Ok(input
        .iter()
        .map(|&v| state.append(v).unwrap_or(f64::NAN))
        .collect())
}
