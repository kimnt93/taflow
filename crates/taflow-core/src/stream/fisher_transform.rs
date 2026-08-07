//! Batch implementation for `fisher_transform`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Compute the Fisher transform from aligned high and low prices.
///
/// `timeperiod` controls the trailing normalization window; warm-up output
/// Compute the fisher transform result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn fisher_transform(high: &[f64], low: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if high.len() != low.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len(),
        });
    }
    let mut state = FisherTransform::new(timeperiod)?;
    Ok(high
        .iter()
        .zip(low)
        .map(|(&h, &l)| state.append(h, l).unwrap_or(f64::NAN))
        .collect())
}
