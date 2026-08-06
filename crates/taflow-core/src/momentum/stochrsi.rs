//! Batch Stochastic Relative Strength Index.
//!
//! STOCHRSI applies a rolling stochastic range to RSI values, then smooths
//! fast %K with a selectable TA-Lib moving average to produce fast %D.

use crate::error::TaResult;
use crate::ma_type::MaType;

/// Computes aligned stochastic-RSI fast %K and fast %D arrays.
pub fn stochrsi(
    input: &[f64],
    timeperiod: usize,
    fastk_period: usize,
    fastd_period: usize,
    fastd_matype: MaType,
) -> TaResult<(Vec<f64>, Vec<f64>)> {
    let rsi_values = super::rsi::rsi(input, timeperiod)?;
    let rsi_valid = &rsi_values[timeperiod..];
    let (stochastic_k, stochastic_d) = super::stochf::stochf(
        rsi_valid,
        rsi_valid,
        rsi_valid,
        fastk_period,
        fastd_period,
        fastd_matype,
    )?;
    let len = input.len();
    let mut fastk_out = vec![f64::NAN; len];
    let mut fastd_out = vec![f64::NAN; len];
    fastk_out[timeperiod..].copy_from_slice(&stochastic_k);
    fastd_out[timeperiod..].copy_from_slice(&stochastic_d);
    Ok((fastk_out, fastd_out))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_selected_ma_lookback() {
        let input: Vec<f64> = (0..500)
            .map(|index| 100.0 + (index as f64 * 0.17).sin() * 8.0)
            .collect();
        for code in 0..=8 {
            let ma_type = MaType::try_from(code).unwrap();
            let (fastk, fastd) = stochrsi(&input, 14, 5, 13, ma_type).unwrap();
            let expected_start = 14 + 4 + ma_type.lookback(13);
            assert!(fastk[..expected_start].iter().all(|value| value.is_nan()));
            assert!(fastd[..expected_start].iter().all(|value| value.is_nan()));
            assert!(!fastk[expected_start].is_nan());
            assert!(!fastd[expected_start].is_nan());
        }
    }
}
