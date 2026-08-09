//! Batch implementation for `time_series_rank`.

use super::rolling_rank::rolling_rank;
use crate::error::TaResult;

/// Canonical WorldQuant name for the existing causal rolling-rank state.
pub type TimeSeriesRank = super::RollingRank;

/// WorldQuant Alpha101 time-series rank: the rank of the current value within
/// the trailing `d`-bar window as a fraction in `(0, 1]`. Shares the rolling
/// Compute the time series rank result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn time_series_rank(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    rolling_rank(input, timeperiod)
}
