//! Batch implementation for `mcginley_dynamic`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes the causal mcginley dynamic series.
/// Parameters: aligned input slices followed by indicator parameters.
/// Compute the mcginley dynamic result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `length` - Input series or configuration value.
/// * `c` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn mcginley_dynamic(input: &[f64], length: usize, c: f64) -> TaResult<Vec<f64>> {
    let mut state = McGinleyDynamic::new(length, c)?;
    Ok(input.iter().map(|&v| state.append(v).unwrap()).collect())
}
