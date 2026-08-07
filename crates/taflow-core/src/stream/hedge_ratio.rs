//! Batch implementation for `hedge_ratio`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Compute the hedge ratio result for the supplied aligned series.
///
/// # Parameters
///
/// * `x` - Input series or configuration value.
/// * `y` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn hedge_ratio(x: &[f64], y: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if x.len() != y.len() {
        return Err(TaError::LengthMismatch {
            expected: x.len(),
            got: y.len(),
        });
    }
    let mut state = HedgeRatio::new(timeperiod)?;
    Ok(x.iter()
        .zip(y)
        .map(|(&x, &y)| state.append(x, y).unwrap_or(f64::NAN))
        .collect())
}
