//! Batch implementation for `cumulative_sum_control_chart`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes the cumulative-sum control-chart signal.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn cumulative_sum_control_chart(input: &[f64], threshold: f64) -> TaResult<Vec<f64>> {
    let mut state = CumulativeSumControlChart::new(threshold)?;
    Ok(input.iter().map(|&change| state.append(change)).collect())
}
