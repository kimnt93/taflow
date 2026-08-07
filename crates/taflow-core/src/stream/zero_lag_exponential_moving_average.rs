//! Batch implementation for `zero_lag_exponential_moving_average`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes the causal zero lag exponential moving average series.
/// Parameters: aligned input slices followed by indicator parameters.
/// Compute the zero lag exponential moving average result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn zero_lag_exponential_moving_average(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = ZeroLagExponentialMovingAverage::new(timeperiod)?;
    Ok(input
        .iter()
        .map(|&v| state.append(v).unwrap_or(f64::NAN))
        .collect())
}
