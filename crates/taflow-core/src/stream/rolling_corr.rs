//! Batch implementation for `rolling_corr`.

use super::rolling_statistics::CORREL_DENOMINATOR_EPSILON;
use super::statistic::*;
use crate::error::{TaError, TaResult};

/// Pearson correlation coefficient (CORREL) — O(n) sliding-window algorithm.
///
/// Uses the identity: correl = (n*sxy - sx*sy) /
/// sqrt((sxx - sx²/n) * (syy - sy²/n)) — TA_CORREL's exact form.
/// Compute the rolling corr result for the supplied aligned series.
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
pub fn rolling_corr(input0: &[f64], input1: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let len = input0.len();
    if len != input1.len() {
        return Err(TaError::LengthMismatch {
            expected: len,
            got: input1.len(),
        });
    }
    if len < timeperiod {
        return Err(TaError::InsufficientData {
            need: timeperiod,
            got: len,
        });
    }
    let lookback = timeperiod - 1;
    let mut output = vec![0.0_f64; len];
    output[..lookback].fill(f64::NAN);
    let n = timeperiod as f64;

    // Initialize rolling sums for the first [0..timeperiod] window.
    let init_x = &input0[0..timeperiod];
    let init_y = &input1[0..timeperiod];
    let mut sx = crate::simd::sum_f64(init_x);
    let mut sy = crate::simd::sum_f64(init_y);
    let mut sxx: f64 = init_x.iter().map(|&v| v * v).sum();
    let mut syy: f64 = init_y.iter().map(|&v| v * v).sum();
    let mut sxy: f64 = init_x.iter().zip(init_y.iter()).map(|(&x, &y)| x * y).sum();

    let num = sxy - ((sx * sy) / n);
    let denom = (sxx - ((sx * sx) / n)) * (syy - ((sy * sy) / n));
    output[lookback] = if !(denom < CORREL_DENOMINATOR_EPSILON) {
        num / denom.sqrt()
    } else {
        0.0
    };

    // Slide the window, reseeding the sums on the same absolute-append
    // cadence as the streaming state (one append per bar, so the count at
    // bar `i` is `i + 1`) to bound subtractive-cancellation drift. See
    // `PAIR_MOMENTS_RESEED_INTERVAL` — streaming and batch stay bitwise equal.
    let reseed_interval = super::rolling_statistics::PAIR_MOMENTS_RESEED_INTERVAL as usize;
    for i in timeperiod..len {
        let old_x = input0[i - timeperiod];
        let old_y = input1[i - timeperiod];
        let new_x = input0[i];
        let new_y = input1[i];

        sx += new_x - old_x;
        sy += new_y - old_y;
        sxx += new_x * new_x - old_x * old_x;
        syy += new_y * new_y - old_y * old_y;
        sxy += new_x * new_y - old_x * old_y;

        if (i + 1) % reseed_interval == 0 {
            // Serial oldest-to-newest recomputation over the current window,
            // identical to `RollingPairMoments::reseed_serial`.
            sx = 0.0;
            sy = 0.0;
            sxx = 0.0;
            syy = 0.0;
            sxy = 0.0;
            for j in i + 1 - timeperiod..=i {
                let x = input0[j];
                let y = input1[j];
                sx += x;
                sy += y;
                sxx += x * x;
                syy += y * y;
                sxy += x * y;
            }
        }

        let num = sxy - ((sx * sy) / n);
        let denom = (sxx - ((sx * sx) / n)) * (syy - ((sy * sy) / n));
        output[i] = if !(denom < CORREL_DENOMINATOR_EPSILON) {
            num / denom.sqrt()
        } else {
            0.0
        };
    }
    Ok(output)
}
