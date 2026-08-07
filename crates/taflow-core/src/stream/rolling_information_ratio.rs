//! Batch implementation for `rolling_information_ratio`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `rolling_information_ratio` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the rolling information ratio result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `benchmark` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_information_ratio(
    input: &[f64],
    benchmark: &[f64],
    timeperiod: usize,
) -> TaResult<Vec<f64>> {
    if input.len() != benchmark.len() {
        return Err(TaError::LengthMismatch {
            expected: input.len(),
            got: benchmark.len(),
        });
    }
    let mut state = RollingInformationRatio::new(timeperiod)?;
    Ok(input
        .iter()
        .zip(benchmark)
        .map(|(&input, &benchmark)| state.append(input, benchmark).unwrap_or(f64::NAN))
        .collect())
}
