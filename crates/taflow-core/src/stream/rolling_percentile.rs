//! Batch implementation for `rolling_percentile`.

use super::operator_states::*;
use super::rolling_quantile::rolling_quantile;
use crate::error::{TaError, TaResult};

/// Computes or updates `rolling_percentile` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the rolling percentile result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
/// * `percentile` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_percentile(input: &[f64], timeperiod: usize, percentile: f64) -> TaResult<Vec<f64>> {
    if !(0.0..=100.0).contains(&percentile) {
        return Err(TaError::InvalidParameter {
            name: "percentile",
            value: percentile.to_string(),
            reason: "must be between 0 and 100",
        });
    }
    rolling_quantile(input, timeperiod, percentile / 100.0)
}
