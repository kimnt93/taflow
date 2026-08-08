//! Batch implementation for `rolling_argmax`.

use super::math_operator::*;
use crate::error::{TaError, TaResult};

/// Index of the rolling maximum; ties keep the first occurrence, matching C TA-Lib.
///
/// Compute the rolling argmax result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_argmax(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_period(input, timeperiod)?;
    let len = input.len();
    let lookback = timeperiod - 1;
    let mut output = vec![0.0_f64; len]; // C fills lookback with 0, not NaN

    let mut highest = input[0];
    let mut highest_idx: usize = 0;
    for j in 1..timeperiod {
        if input[j] > highest {
            highest = input[j];
            highest_idx = j;
        }
    }
    output[lookback] = highest_idx as f64;

    let mut trailing_idx = 1;
    let mut today = timeperiod;

    while today < len {
        let v = input[today];
        if highest_idx < trailing_idx {
            highest_idx = trailing_idx;
            highest = input[trailing_idx];
            for (j, &val) in input[trailing_idx + 1..=today].iter().enumerate() {
                if val > highest {
                    highest = val;
                    highest_idx = trailing_idx + 1 + j;
                }
            }
        } else if v >= highest {
            // fast path: >= matches C (update on tie)
            highest_idx = today;
            highest = v;
        }
        output[today] = highest_idx as f64;
        trailing_idx += 1;
        today += 1;
    }
    Ok(output)
}
