//! Batch implementation for `time_series_rank`.

/// WorldQuant Alpha101 time-series rank over the trailing window.
///
/// This canonical state delegates to the shared rolling-rank recurrence.
pub type TimeSeriesRank = super::RollingRank;
