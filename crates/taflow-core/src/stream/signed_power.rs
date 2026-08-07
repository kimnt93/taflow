//! Batch implementation for `signed_power`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes pointwise signed power `sign(x)·|x|^a`.
/// Compute the signed power result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `exponent` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn signed_power(input: &[f64], exponent: f64) -> Vec<f64> {
    input
        .iter()
        .map(|&value| {
            if exponent == 2.0 {
                value * value.abs()
            } else {
                value.signum() * value.abs().powf(exponent)
            }
        })
        .collect()
}
