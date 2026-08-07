//! Batch Average Directional Movement Index Rating.

use crate::error::TaResult;

/// Computes ADXR as the mean of current ADX and ADX from `period - 1` bars ago.
pub fn average_directional_index_rating(high: &[f64], low: &[f64], close: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let adx_values = super::adx::average_directional_index(high, low, close, timeperiod)?;
    let len = adx_values.len();
    let lookback = 3 * timeperiod - 2;
    let mut output = vec![f64::NAN; len];
    for index in lookback..len {
        output[index] = (adx_values[index] + adx_values[index - timeperiod + 1]) / 2.0;
    }
    Ok(output)
}
