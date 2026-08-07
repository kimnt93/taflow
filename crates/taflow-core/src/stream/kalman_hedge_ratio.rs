//! Batch implementation for `kalman_hedge_ratio`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `kalman_hedge_ratio` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn kalman_hedge_ratio(
    x: &[f64],
    y: &[f64],
    delta: f64,
    observation_variance: f64,
) -> TaResult<Vec<f64>> {
    if x.len() != y.len() {
        return Err(TaError::LengthMismatch {
            expected: x.len(),
            got: y.len(),
        });
    }
    let mut state = KalmanHedgeRatio::new(delta, observation_variance)?;
    Ok(x.iter()
        .zip(y)
        .map(|(&x, &y)| state.append(x, y).unwrap_or(f64::NAN))
        .collect())
}
