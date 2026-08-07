//! Batch implementation for `ulcer_index`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Compute the causal ulcer index for an aligned price series.
///
/// Parameters are the input prices and rolling period; the returned vector
/// Compute the ulcer index result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn ulcer_index(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = UlcerIndex::new(timeperiod)?;
    Ok(input
        .iter()
        .map(|&value| state.append(value).unwrap_or(f64::NAN))
        .collect())
}
