//! Batch implementation for `rolling_linreg_slope`.

use super::statistic::*;
use crate::error::{TaError, TaResult};

/// Compute the rolling linreg slope result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_linreg_slope(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let (slope, _) = linearreg_components(input, timeperiod)?;
    Ok(slope)
}
