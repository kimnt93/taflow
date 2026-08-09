//! Batch implementation for `rolling_kurtosis`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};
use std::collections::VecDeque;

rolling_moment_operator!(RollingKurtosis, |n: f64, m2: f64, _m3: f64, m4: f64| {
    if m2 > 0.0 {
        n * m4 / m2.powi(2) - 3.0
    } else {
        0.0
    }
});

/// Computes or updates `rolling_kurtosis` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the rolling kurtosis result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_kurtosis(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = RollingKurtosis::new(timeperiod)?;
    Ok(input
        .iter()
        .map(|&value| state.append(value).unwrap_or(f64::NAN))
        .collect())
}
