//! Batch implementation for `rolling_entropy`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Compute rolling Shannon entropy over an aligned input series.
///
/// Parameters are the input values and window length; the result is aligned
/// Compute the rolling entropy result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_entropy(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = RollingEntropy::new(timeperiod)?;
    Ok(input
        .iter()
        .map(|&value| state.append(value).unwrap_or(f64::NAN))
        .collect())
}
