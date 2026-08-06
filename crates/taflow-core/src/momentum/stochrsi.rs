//! Batch Stochastic Relative Strength Index.
//!
//! STOCHRSI applies a rolling stochastic range to RSI values, then smooths
//! fast %K with a selectable TA-Lib moving average to produce fast %D.

use crate::error::{TaError, TaResult};
use crate::ma_type::{compute_ma, MaType};

/// Computes aligned stochastic-RSI fast %K and fast %D arrays.
pub fn stochrsi(
    input: &[f64],
    timeperiod: usize,
    fastk_period: usize,
    fastd_period: usize,
    fastd_matype: MaType,
) -> TaResult<(Vec<f64>, Vec<f64>)> {
    let rsi_values = crate::momentum::rsi::rsi(input, timeperiod)?;
    let rsi_valid: Vec<f64> = rsi_values
        .iter()
        .copied()
        .filter(|value| !value.is_nan())
        .collect();

    if rsi_valid.len() <= fastk_period {
        return Err(TaError::InsufficientData {
            need: timeperiod + fastk_period + 1,
            got: input.len(),
        });
    }

    let rsi_len = rsi_valid.len();
    let mut fastk_values = Vec::new();
    for index in (fastk_period - 1)..rsi_len {
        let start = index + 1 - fastk_period;
        let highest = rsi_valid[start..=index]
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        let lowest = rsi_valid[start..=index]
            .iter()
            .copied()
            .fold(f64::INFINITY, f64::min);
        let range = highest - lowest;
        fastk_values.push(if range > 0.0 {
            100.0 * (rsi_valid[index] - lowest) / range
        } else {
            0.0
        });
    }

    let fastd_arr = compute_ma(&fastk_values, fastd_period, fastd_matype)?;
    let len = input.len();
    let fastd_lookback = fastd_matype.lookback(fastd_period);
    let aligned_start = timeperiod + fastk_period - 1 + fastd_lookback;
    let mut fastk_out = vec![f64::NAN; len];
    let mut fastd_out = vec![f64::NAN; len];
    for (offset, bar) in (aligned_start..len).enumerate() {
        let value_index = fastd_lookback + offset;
        fastk_out[bar] = fastk_values[value_index];
        fastd_out[bar] = fastd_arr[value_index];
    }
    Ok((fastk_out, fastd_out))
}
