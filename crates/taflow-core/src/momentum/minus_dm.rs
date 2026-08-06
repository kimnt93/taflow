use crate::error::{TaError, TaResult};

/// Minus Directional Movement (-DM).
pub fn minus_dm(high: &[f64], low: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let len = high.len();
    if len != low.len() {
        return Err(TaError::LengthMismatch {
            expected: len,
            got: low.len(),
        });
    }
    if timeperiod < 1 || len < timeperiod {
        return Err(TaError::InsufficientData {
            need: timeperiod.max(1),
            got: len,
        });
    }
    let mut output = vec![0.0; len];
    if timeperiod > 1 {
        output[..timeperiod - 1].fill(f64::NAN);
    }
    let mut sum = 0.0;
    for index in 1..timeperiod {
        let up = high[index] - high[index - 1];
        let down = low[index - 1] - low[index];
        if down > up && down > 0.0 {
            sum += down;
        }
    }
    output[timeperiod - 1] = sum;
    let period = timeperiod as f64;
    for index in timeperiod..len {
        let up = high[index] - high[index - 1];
        let down = low[index - 1] - low[index];
        sum = sum - sum / period + if down > up && down > 0.0 { down } else { 0.0 };
        output[index] = sum;
    }
    Ok(output)
}
