//! Batch Stochastic Oscillator.
//!
//! STOCH calculates fast %K from a rolling high/low range, then applies two
//! independently selectable TA-Lib moving averages to produce slow %K and
//! slow %D with their shared output alignment.

use crate::error::{TaError, TaResult};
use crate::ma_type::{compute_ma, MaType};

/// Computes aligned slow %K and slow %D output arrays.
pub fn stochastic_oscillator(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    fastk_period: usize,
    slowk_period: usize,
    slowk_matype: MaType,
    slowd_period: usize,
    slowd_matype: MaType,
) -> TaResult<(Vec<f64>, Vec<f64>)> {
    let len = high.len();
    if len != low.len() || len != close.len() {
        return Err(TaError::LengthMismatch {
            expected: len,
            got: low.len().min(close.len()),
        });
    }
    if fastk_period < 1 || slowk_period < 1 || slowd_period < 1 {
        return Err(TaError::InvalidParameter {
            name: "periods",
            value: format!("{}/{}/{}", fastk_period, slowk_period, slowd_period),
            reason: "all periods must be >= 1",
        });
    }

    let slowk_lookback = slowk_matype.lookback(slowk_period);
    let slowd_lookback = slowd_matype.lookback(slowd_period);
    let lookback = fastk_period - 1 + slowk_lookback + slowd_lookback;
    if len <= lookback {
        return Err(TaError::InsufficientData {
            need: lookback + 1,
            got: len,
        });
    }

    let fastk_len = len - (fastk_period - 1);
    let mut fastk = Vec::with_capacity(fastk_len);
    for i in (fastk_period - 1)..len {
        let start = i + 1 - fastk_period;
        let mut hh = f64::NEG_INFINITY;
        let mut ll = f64::INFINITY;
        for j in start..=i {
            let h = high[j];
            let l = low[j];
            if h > hh {
                hh = h;
            }
            if l < ll {
                ll = l;
            }
        }
        let divisor = (hh - ll) / 100.0;
        if divisor.abs() >= 1.0e-14 {
            fastk.push((close[i] - ll) / divisor);
        } else {
            fastk.push(0.0);
        }
    }

    let slowk_arr = compute_ma(&fastk, slowk_period, slowk_matype)?;
    let slowk_valid = &slowk_arr[slowk_lookback..];
    let slowd_arr = compute_ma(slowk_valid, slowd_period, slowd_matype)?;

    let mut slowk_out = vec![f64::NAN; len];
    let mut slowd_out = vec![f64::NAN; len];
    for (offset, bar) in (lookback..len).enumerate() {
        let value_index = slowd_lookback + offset;
        slowk_out[bar] = slowk_valid[value_index];
        slowd_out[bar] = slowd_arr[value_index];
    }

    Ok((slowk_out, slowd_out))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_both_selected_ma_lookbacks() {
        let high: Vec<f64> = (0..500)
            .map(|i| 50.0 + (i as f64 * 0.3).sin() * 5.0 + 2.0)
            .collect();
        let low: Vec<f64> = (0..500)
            .map(|i| 50.0 + (i as f64 * 0.3).sin() * 5.0 - 2.0)
            .collect();
        let close: Vec<f64> = (0..500)
            .map(|i| 50.0 + (i as f64 * 0.3).sin() * 5.0)
            .collect();
        for slowk_code in 0..=8 {
            for slowd_code in 0..=8 {
                let slowk_type = MaType::try_from(slowk_code).unwrap();
                let slowd_type = MaType::try_from(slowd_code).unwrap();
                let (slowk, slowd) =
                    stochastic_oscillator(&high, &low, &close, 5, 13, slowk_type, 11, slowd_type).unwrap();
                let expected_start = 4 + slowk_type.lookback(13) + slowd_type.lookback(11);
                assert!(slowk[..expected_start].iter().all(|value| value.is_nan()));
                assert!(slowd[..expected_start].iter().all(|value| value.is_nan()));
                assert!(!slowk[expected_start].is_nan());
                assert!(!slowd[expected_start].is_nan());
            }
        }
    }
}
