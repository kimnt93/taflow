//! Batch implementation for `rolling_beta`.

use super::rolling_statistics::{beta_return, ta_is_zero};
use crate::error::{TaError, TaResult};

/// RollingBeta — O(n) sliding-window algorithm.
///
/// C TA-Lib BETA uses percentage returns:
///   x[i] = (input0[i] - input0[i-1]) / input0[i-1]  (input0 return)
///   y[i] = (input1[i] - input1[i-1]) / input1[i-1]  (input1 return)
///   beta = (n*sxy - sx*sy) / (n*sxx - sx*sx)
///
/// `input0` is the market benchmark and `input1` is the stock;
/// the denominator is `rolling_var(x)`, the variance of benchmark returns.
/// Compute the rolling beta result for the supplied aligned series.
///
/// # Parameters
///
/// * `input0` - Input series or configuration value.
/// * `input1` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_beta(input0: &[f64], input1: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let len = input0.len();
    if len != input1.len() {
        return Err(TaError::LengthMismatch {
            expected: len,
            got: input1.len(),
        });
    }
    if timeperiod < 2 {
        return Err(TaError::InvalidParameter {
            name: "timeperiod",
            value: timeperiod.to_string(),
            reason: "must be >= 2",
        });
    }
    if len <= timeperiod {
        return Err(TaError::InsufficientData {
            need: timeperiod + 1,
            got: len,
        });
    }
    let mut output = vec![0.0_f64; len];
    output[..timeperiod].fill(f64::NAN);
    let n = timeperiod as f64;

    // Returns are recomputed straight from the price slices — exactly what
    // the C loop does through its `lastPrice`/`trailingLastPrice` cursors —
    // so no `len`-sized return series is materialized. `rx[j]` below denotes
    // the return between bars `j` and `j + 1`.
    let mut sx = 0.0_f64;
    let mut sy = 0.0_f64;
    let mut sxx = 0.0_f64;
    let mut sxy = 0.0_f64;

    // TA_BETA seeds `timeperiod - 1` returns before the emit loop, then per
    // bar adds the incoming return, emits, and only then removes the trailing
    // one. That order — the mirror of TA_CORREL's — is load-bearing: it makes
    // BETA bitwise equal to TA-Lib. See `RollingReturnMoments`. No reseed.
    for j in 0..(timeperiod - 1) {
        let x = beta_return(input0[j + 1], input0[j]);
        let y = beta_return(input1[j + 1], input1[j]);
        sxx += x * x;
        sxy += x * y;
        sx += x;
        sy += y;
    }

    // Branch-free steady loop over zipped slices; output slot `k` is bar
    // `k + timeperiod`, whose incoming return spans bars `k + timeperiod - 1`
    // and `k + timeperiod`, and whose trailing return spans `k` and `k + 1`.
    let steady = len - timeperiod;
    let out = &mut output[timeperiod..];
    let new0 = &input0[timeperiod..];
    let new1 = &input1[timeperiod..];
    let previous0 = &input0[timeperiod - 1..len - 1];
    let previous1 = &input1[timeperiod - 1..len - 1];
    let trailing0 = &input0[1..steady + 1];
    let trailing1 = &input1[1..steady + 1];
    let trailing_previous0 = &input0[..steady];
    let trailing_previous1 = &input1[..steady];

    for (k, slot) in out.iter_mut().enumerate() {
        let x = beta_return(new0[k], previous0[k]);
        let y = beta_return(new1[k], previous1[k]);
        sxx += x * x;
        sxy += x * y;
        sx += x;
        sy += y;

        // TA-Lib reads the trailing return before writing the output.
        let trailing_x = beta_return(trailing0[k], trailing_previous0[k]);
        let trailing_y = beta_return(trailing1[k], trailing_previous1[k]);

        let denom = (n * sxx) - (sx * sx);
        *slot = if !ta_is_zero(denom) {
            ((n * sxy) - (sx * sy)) / denom
        } else {
            0.0
        };

        // Remove the trailing return, after the emit.
        sxx -= trailing_x * trailing_x;
        sxy -= trailing_x * trailing_y;
        sx -= trailing_x;
        sy -= trailing_y;
    }
    Ok(output)
}
