//! Batch implementation for `rolling_sharpe`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};
use std::collections::VecDeque;

rolling_risk_operator!(RollingSharpe, |values: &VecDeque<f64>| {
    let average = mean(values);
    let variance = values
        .iter()
        .map(|&value| (value - average).powi(2))
        .sum::<f64>()
        / values.len() as f64;
    if variance > 0.0 {
        average / variance.sqrt()
    } else {
        0.0
    }
});

/// Computes or updates `rolling_sharpe` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the rolling sharpe result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_sharpe(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = RollingSharpe::new(timeperiod)?;
    Ok(input
        .iter()
        .map(|&value| state.append(value).unwrap_or(f64::NAN))
        .collect())
}
