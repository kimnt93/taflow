use crate::error::{TaError, TaResult};

/// Acceleration Bands (ACCBANDS).
///
/// TA-Lib defines three SMA windows:
/// - upper: `high * (1 + 4 * (high - low) / (high + low))`
/// - middle: `close`
/// - lower: `low * (1 - 4 * (high - low) / (high + low))`
pub fn accbands(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    timeperiod: usize,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    let len = high.len();
    if len != low.len() || len != close.len() {
        return Err(TaError::LengthMismatch {
            expected: len,
            got: low.len().min(close.len()),
        });
    }
    if timeperiod < 2 {
        return Err(TaError::InvalidParameter {
            name: "timeperiod",
            value: timeperiod.to_string(),
            reason: "must be >= 2",
        });
    }
    if len < timeperiod {
        return Err(TaError::InsufficientData {
            need: timeperiod,
            got: len,
        });
    }

    fn bands_input(high: f64, low: f64) -> (f64, f64) {
        let denominator = high + low;
        if denominator == 0.0 {
            (high, low)
        } else {
            let adjustment = 4.0 * (high - low) / denominator;
            (high * (1.0 + adjustment), low * (1.0 - adjustment))
        }
    }

    let lookback = timeperiod - 1;
    let mut upper = vec![f64::NAN; len];
    let mut middle = vec![f64::NAN; len];
    let mut lower = vec![f64::NAN; len];
    let mut upper_sum = 0.0;
    let mut middle_sum = 0.0;
    let mut lower_sum = 0.0;

    for index in 0..timeperiod {
        let (upper_input, lower_input) = bands_input(high[index], low[index]);
        upper_sum += upper_input;
        middle_sum += close[index];
        lower_sum += lower_input;
    }
    let period = timeperiod as f64;
    upper[lookback] = upper_sum / period;
    middle[lookback] = middle_sum / period;
    lower[lookback] = lower_sum / period;
    let (old_upper, old_lower) = bands_input(high[0], low[0]);
    upper_sum -= old_upper;
    middle_sum -= close[0];
    lower_sum -= old_lower;

    for index in timeperiod..len {
        let (new_upper, new_lower) = bands_input(high[index], low[index]);
        let old_index = index + 1 - timeperiod;
        let (old_upper, old_lower) = bands_input(high[old_index], low[old_index]);
        upper_sum += new_upper;
        middle_sum += close[index];
        lower_sum += new_lower;
        upper[index] = upper_sum / period;
        middle[index] = middle_sum / period;
        lower[index] = lower_sum / period;
        upper_sum -= old_upper;
        middle_sum -= close[old_index];
        lower_sum -= old_lower;
    }
    Ok((upper, middle, lower))
}
