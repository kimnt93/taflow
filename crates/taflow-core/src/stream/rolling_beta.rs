//! Batch implementation for `rolling_beta`.

use super::statistic::*;
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
    if len <= timeperiod {
        return Err(TaError::InsufficientData {
            need: timeperiod + 1,
            got: len,
        });
    }
    let mut output = vec![0.0_f64; len];
    output[..timeperiod].fill(f64::NAN);
    let n = timeperiod as f64;

    // Precompute percentage returns once to avoid repeated division per window.
    // rx[i] corresponds to original index i+1; the return series has length len - 1.
    let ret_len = len - 1;
    let mut rx = vec![0.0_f64; ret_len];
    let mut ry = vec![0.0_f64; ret_len];
    for j in 0..ret_len {
        rx[j] = (input0[j + 1] - input0[j]) / input0[j];
        ry[j] = (input1[j + 1] - input1[j]) / input1[j];
    }

    // For output index i (i >= timeperiod), use rx[i-timeperiod..i].
    let mut sx = 0.0_f64;
    let mut sy = 0.0_f64;
    let mut sxx = 0.0_f64;
    let mut sxy = 0.0_f64;

    // Initialize the first window: rx[0..timeperiod] produces output[timeperiod].
    for j in 0..timeperiod {
        let x = rx[j];
        let y = ry[j];
        sx += x;
        sy += y;
        sxx += x * x;
        sxy += x * y;
    }

    let denom = n * sxx - sx * sx;
    output[timeperiod] = if denom > 0.0 {
        (n * sxy - sx * sy) / denom
    } else {
        0.0
    };

    // Slide the window, reseeding the sums on the same absolute-append
    // cadence as the streaming state (returns are appended from bar 1, so
    // the streaming count at bar `i` is `i`) to bound drift. See
    // `PAIR_MOMENTS_RESEED_INTERVAL` — streaming and batch stay bitwise
    // equal. `syy` is not tracked here; its streaming reseed is independent.
    let reseed_interval = super::rolling_statistics::PAIR_MOMENTS_RESEED_INTERVAL as usize;
    for i in (timeperiod + 1)..len {
        // Remove rx[i - timeperiod - 1] and add rx[i - 1].
        let old_idx = i - timeperiod - 1;
        let new_idx = i - 1;
        let ox = rx[old_idx];
        let oy = ry[old_idx];
        let nx = rx[new_idx];
        let ny = ry[new_idx];
        sx += nx - ox;
        sy += ny - oy;
        sxx += nx * nx - ox * ox;
        sxy += nx * ny - ox * oy;

        if i % reseed_interval == 0 {
            // Serial oldest-to-newest recomputation over the current window,
            // identical to `RollingPairMoments::reseed_serial`.
            sx = 0.0;
            sy = 0.0;
            sxx = 0.0;
            sxy = 0.0;
            for j in i - timeperiod..i {
                let x = rx[j];
                let y = ry[j];
                sx += x;
                sy += y;
                sxx += x * x;
                sxy += x * y;
            }
        }

        let denom = n * sxx - sx * sx;
        output[i] = if denom > 0.0 {
            (n * sxy - sx * sy) / denom
        } else {
            0.0
        };
    }
    Ok(output)
}
