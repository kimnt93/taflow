//! Batch implementation for `decay_linear`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// WorldQuant Alpha101 `decay_linear(x, d)`: verified alias of the weighted
/// Compute the decay linear result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn decay_linear(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    crate::stream::weighted_moving_average(input, timeperiod)
}
