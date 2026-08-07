//! Batch implementation for `rolling_var`.

use super::statistic::*;
use crate::error::{TaError, TaResult};

/// Compute the rolling var result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_var(input: &[f64], timeperiod: usize, _nbdev: f64) -> TaResult<Vec<f64>> {
    var_internal(input, timeperiod)
}
