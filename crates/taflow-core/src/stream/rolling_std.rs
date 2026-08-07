//! Batch implementation for `rolling_std`.

use super::statistic::*;
use crate::error::{TaError, TaResult};

/// Standard Deviation (STDDEV) — fused single-pass var+sqrt
///
/// Eliminates intermediate var Vec (8MB at 1M bars) by computing
/// Compute the rolling std result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
/// * `nbdev` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_std(input: &[f64], timeperiod: usize, nbdev: f64) -> TaResult<Vec<f64>> {
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
    let inv_n = 1.0 / timeperiod as f64;
    let mut output = vec![0.0_f64; len];
    output[..lookback].fill(f64::NAN);

    let mut sum = 0.0_f64;
    let mut sum_sq = 0.0_f64;
    for j in 0..timeperiod {
        sum += input[j];
        sum_sq = input[j].mul_add(input[j], sum_sq);
    }
    let mean = sum * inv_n;
    output[lookback] = (sum_sq * inv_n - mean * mean).max(0.0).sqrt() * nbdev;

    for i in timeperiod..len {
        let old = input[i - timeperiod];
        let new_val = input[i];
        sum += new_val - old;
        sum_sq += (new_val - old).mul_add(new_val + old, 0.0);
        let mean = sum * inv_n;
        output[i] = (sum_sq * inv_n - mean * mean).max(0.0).sqrt() * nbdev;
    }

    Ok(output)
}
