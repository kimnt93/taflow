//! Batch implementation for `rolling_tsf`.

use super::statistic::*;
use crate::error::{TaError, TaResult};

/// Compute the rolling tsf result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_tsf(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let (slope, intercept) = linearreg_components(input, timeperiod)?;
    let len = input.len();
    let lookback = timeperiod - 1;
    let mut output = vec![0.0_f64; len];
    output[..lookback].fill(f64::NAN);
    for i in lookback..len {
        output[i] = intercept[i] + slope[i] * timeperiod as f64;
    }
    Ok(output)
}
