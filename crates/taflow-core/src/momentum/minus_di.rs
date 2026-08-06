use crate::error::{TaError, TaResult};

/// Minus Directional Indicator (-DI).
pub fn minus_di(high: &[f64], low: &[f64], close: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let len = high.len();
    if len != low.len() || len != close.len() {
        return Err(TaError::LengthMismatch {
            expected: len,
            got: low.len().min(close.len()),
        });
    }
    if timeperiod < 1 || len <= timeperiod {
        return Err(TaError::InsufficientData {
            need: timeperiod + 1,
            got: len,
        });
    }
    let mut output = vec![0.0; len];
    output[..timeperiod].fill(f64::NAN);
    let period = timeperiod as f64;
    let (mut true_range, mut movement) = (0.0, 0.0);
    for index in 1..timeperiod {
        true_range += (high[index] - low[index])
            .max((high[index] - close[index - 1]).abs())
            .max((low[index] - close[index - 1]).abs());
        let up = high[index] - high[index - 1];
        let down = low[index - 1] - low[index];
        if down > up && down > 0.0 {
            movement += down;
        }
    }
    for index in timeperiod..len {
        let range = (high[index] - low[index])
            .max((high[index] - close[index - 1]).abs())
            .max((low[index] - close[index - 1]).abs());
        let up = high[index] - high[index - 1];
        let down = low[index - 1] - low[index];
        true_range = true_range - true_range / period + range;
        movement = movement - movement / period + if down > up && down > 0.0 { down } else { 0.0 };
        output[index] = if true_range > 0.0 {
            100.0 * movement / true_range
        } else {
            0.0
        };
    }
    Ok(output)
}
