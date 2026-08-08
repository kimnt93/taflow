//! Batch implementation for `rolling_corr`.

use super::rolling_statistics::CORREL_DENOMINATOR_EPSILON;
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

    // Seed the five sums over the first window, serially and in TA_CORREL's
    // per-statement order — identical to the streaming warm-up appends.
    let mut sx = 0.0_f64;
    let mut sy = 0.0_f64;
    let mut sxx = 0.0_f64;
    let mut syy = 0.0_f64;
    let mut sxy = 0.0_f64;
    for j in 0..timeperiod {
        let x = input0[j];
        let y = input1[j];
        sx += x;
        sxx += x * x;
        sxy += x * y;
        sy += y;
        syy += y * y;
    }

    let num = sxy - ((sx * sy) / n);
    let denom = (sxx - ((sx * sx) / n)) * (syy - ((sy * sy) / n));
    output[lookback] = if !(denom < CORREL_DENOMINATOR_EPSILON) {
        num / denom.sqrt()
    } else {
        0.0
    };

    // Steady loop, verbatim TA_CORREL: remove the trailing pair's five
    // contributions, then add the incoming pair's, then emit. No reseed — see
    // `RollingPairMoments` for why matching TA-Lib's drift beats correcting it.
    for i in timeperiod..len {
        let trailing_x = input0[i - timeperiod];
        let trailing_y = input1[i - timeperiod];
        sx -= trailing_x;
        sxx -= trailing_x * trailing_x;
        sxy -= trailing_x * trailing_y;
        sy -= trailing_y;
        syy -= trailing_y * trailing_y;

        let x = input0[i];
        let y = input1[i];
        sx += x;
        sxx += x * x;
        sxy += x * y;
        sy += y;
        syy += y * y;

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
