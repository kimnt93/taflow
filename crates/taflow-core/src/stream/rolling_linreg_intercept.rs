//! Batch implementation for `rolling_linreg_intercept`.

use super::statistic::*;
use crate::error::{TaError, TaResult};

/// Compute the rolling linreg intercept result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_linreg_intercept(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let (_, intercept) = linearreg_components(input, timeperiod)?;
    Ok(intercept)
}
