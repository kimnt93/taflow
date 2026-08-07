//! Batch implementation for `rolling_linreg_angle`.

use super::statistic::*;
use crate::error::{TaError, TaResult};

/// Compute the rolling linreg angle result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_linreg_angle(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let (slope, _) = linearreg_components(input, timeperiod)?;
    let len = input.len();
    let lookback = timeperiod - 1;
    let mut output = vec![0.0_f64; len];
    output[..lookback].fill(f64::NAN);
    for i in 0..len {
        if !slope[i].is_nan() {
            output[i] = slope[i].atan().to_degrees();
        }
    }
    Ok(output)
}
