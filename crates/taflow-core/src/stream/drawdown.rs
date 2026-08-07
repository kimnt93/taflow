//! Batch implementation for `drawdown`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `drawdown` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the drawdown result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn drawdown(input: &[f64]) -> Vec<f64> {
    let mut maximum = f64::NEG_INFINITY;
    input
        .iter()
        .map(|&value| {
            maximum = maximum.max(value);
            if maximum != 0.0 {
                value / maximum - 1.0
            } else {
                0.0
            }
        })
        .collect()
}
