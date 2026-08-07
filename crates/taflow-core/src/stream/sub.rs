//! Batch implementation for `sub`.

use super::math_operator::*;
use crate::error::{TaError, TaResult};

/// Compute the sub result for the supplied aligned series.
///
/// # Parameters
///
/// * `input0` - Input series or configuration value.
/// * `input1` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn sub(input0: &[f64], input1: &[f64]) -> TaResult<Vec<f64>> {
    validate_pair(input0, input1)?;
    Ok(input0
        .iter()
        .zip(input1.iter())
        .map(|(a, b)| a - b)
        .collect())
}
