//! Batch implementation for `falling`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};
use std::collections::VecDeque;

direction_operator!(Falling, |current: f64, previous: f64| current < previous);

/// Compute the causal falling predicate over an aligned input series.
///
/// `timeperiod` is the comparison horizon. The returned values are aligned
/// Compute the falling result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn falling(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = Falling::new(timeperiod)?;
    Ok(input
        .iter()
        .map(|&v| state.append(v).unwrap_or(f64::NAN))
        .collect())
}
