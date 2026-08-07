//! Batch implementation for `decay_linear`.

use crate::error::TaResult;

/// Canonical WorldQuant name for the linearly weighted moving-average state.
pub type DecayLinear = super::WeightedMovingAverage;

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
