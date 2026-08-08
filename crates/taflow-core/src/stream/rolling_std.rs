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

    // Same `TA_INT_VAR` accumulation order as `statistic::var_internal` and
    // `RollingMoments`, fused with `TA_STDDEV`'s post-processing. The order is
    // load-bearing for bitwise TA-Lib parity — see `RollingMoments`' docs.
    let lookback = timeperiod - 1;
    let period = timeperiod as f64;
    let mut output = vec![0.0_f64; len];
    output[..lookback].fill(f64::NAN);

    let mut sum = 0.0_f64;
    let mut sum_sq = 0.0_f64;
    for &value in &input[..lookback] {
        sum += value;
        sum_sq += value * value;
    }

    for i in lookback..len {
        let value = input[i];
        sum += value;
        sum_sq += value * value;
        let mean1 = sum / period;
        let mean2 = sum_sq / period;
        output[i] = super::rolling_statistics::stddev_from_variance(mean2 - mean1 * mean1, nbdev);
        let old = input[i - lookback];
        sum -= old;
        sum_sq -= old * old;
    }

    Ok(output)
}
