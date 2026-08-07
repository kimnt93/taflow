//! Batch implementation for `rolling_avgdev`.

use super::statistic::*;
use crate::error::{TaError, TaResult};

/// Average Deviation (AVGDEV), measured from each window's arithmetic mean.
///
/// TA-Lib intentionally recomputes the absolute deviations for every window;
/// Compute the rolling avgdev result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_avgdev(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if timeperiod < 2 {
        return Err(TaError::InvalidParameter {
            name: "timeperiod",
            value: timeperiod.to_string(),
            reason: "must be >= 2",
        });
    }
    let len = input.len();
    if len < timeperiod {
        return Err(TaError::InsufficientData {
            need: timeperiod,
            got: len,
        });
    }

    let lookback = timeperiod - 1;
    let mut output = vec![f64::NAN; len];
    let period = timeperiod as f64;
    for today in lookback..len {
        let mut sum = 0.0;
        for offset in 0..timeperiod {
            sum += input[today - offset];
        }
        let mean = sum / period;
        let mut deviation = 0.0;
        for offset in 0..timeperiod {
            deviation += (input[today - offset] - mean).abs();
        }
        output[today] = deviation / period;
    }
    Ok(output)
}
