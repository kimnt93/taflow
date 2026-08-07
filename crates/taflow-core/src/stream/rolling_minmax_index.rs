//! Batch implementation for `rolling_minmax_index`.

use super::math_operator::*;
use crate::error::{TaError, TaResult};

/// Compute the rolling minmax index result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_minmax_index(input: &[f64], timeperiod: usize) -> TaResult<(Vec<f64>, Vec<f64>)> {
    validate_period(input, timeperiod)?;
    let len = input.len();
    let lookback = timeperiod - 1;
    let mut out_minidx = vec![0.0_f64; len]; // C fills lookback with 0, not NaN
    let mut out_maxidx = vec![0.0_f64; len];

    let mut highest = input[0];
    let mut highest_idx: usize = 0;
    let mut lowest = input[0];
    let mut lowest_idx: usize = 0;
    // Ties keep FIRST occurrence: use > for max, < for min
    for j in 1..timeperiod {
        if input[j] > highest {
            highest = input[j];
            highest_idx = j;
        }
        if input[j] < lowest {
            lowest = input[j];
            lowest_idx = j;
        }
    }
    out_maxidx[lookback] = highest_idx as f64;
    out_minidx[lookback] = lowest_idx as f64;

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
            // fast path: >= matches C
            highest_idx = today;
            highest = v;
        }

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
            // fast path: <= matches C
            lowest_idx = today;
            lowest = v;
        }

        out_maxidx[today] = highest_idx as f64;
        out_minidx[today] = lowest_idx as f64;
        trailing_idx += 1;
        today += 1;
    }

    Ok((out_minidx, out_maxidx))
}
