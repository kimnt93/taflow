//! Batch implementation for `rolling_argmin`.

use super::math_operator::*;
use crate::error::{TaError, TaResult};

/// Compute the rolling argmin result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_argmin(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_period(input, timeperiod)?;
    let len = input.len();
    let lookback = timeperiod - 1;
    let mut output = vec![0.0_f64; len]; // C fills lookback with 0, not NaN

    let mut lowest = input[0];
    let mut lowest_idx: usize = 0;
    for j in 1..timeperiod {
        if input[j] < lowest {
            lowest = input[j];
            lowest_idx = j;
        }
    }
    output[lookback] = lowest_idx as f64;

    let mut trailing_idx = 1;
    let mut today = timeperiod;

    while today < len {
        let v = input[today];
        if lowest_idx < trailing_idx {
            lowest_idx = trailing_idx;
            lowest = input[trailing_idx];
            for (j, &val) in input[trailing_idx + 1..=today].iter().enumerate() {
                if val < lowest {
                    lowest = val;
                    lowest_idx = trailing_idx + 1 + j;
                }
            }
        } else if v <= lowest {
            // fast path: <= matches C (update on tie)
            lowest_idx = today;
            lowest = v;
        }
        output[today] = lowest_idx as f64;
        trailing_idx += 1;
        today += 1;
    }
    Ok(output)
}
