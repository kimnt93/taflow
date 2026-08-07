//! Batch implementation for `awesome_oscillator`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Compute the Awesome Oscillator from aligned high and low prices.
///
/// `fast` and `slow` are the oscillator windows. The returned series is
/// Compute the awesome oscillator result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `fast` - Input series or configuration value.
/// * `slow` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn awesome_oscillator(
    high: &[f64],
    low: &[f64],
    fast: usize,
    slow: usize,
) -> TaResult<Vec<f64>> {
    if high.len() != low.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len(),
        });
    }
    let mut state = AwesomeOscillator::new(fast, slow)?;
    Ok(high
        .iter()
        .zip(low)
        .map(|(&h, &l)| state.append(h, l).unwrap_or(f64::NAN))
        .collect())
}
