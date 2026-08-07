//! Batch implementation for `true_strength_index`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes the causal true strength index series.
/// Parameters: aligned input slices followed by indicator parameters.
/// Compute the true strength index result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `fast` - Input series or configuration value.
/// * `slow` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn true_strength_index(input: &[f64], fast: usize, slow: usize) -> TaResult<Vec<f64>> {
    let mut state = TrueStrengthIndex::new(fast, slow)?;
    Ok(input
        .iter()
        .map(|&v| state.append(v).unwrap_or(f64::NAN))
        .collect())
}
