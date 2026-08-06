//! Batch Fast Stochastic Oscillator.
//!
//! STOCHF calculates a rolling fast %K from high, low, and close, then applies
//! the selected TA-Lib moving-average type to produce fast %D.

use crate::error::{TaError, TaResult};
use crate::ma_type::{compute_ma, MaType};

/// Computes aligned fast %K and fast %D output arrays.
pub fn stochf(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    fastk_period: usize,
    fastd_period: usize,
    fastd_matype: MaType,
) -> TaResult<(Vec<f64>, Vec<f64>)> {
    let len = high.len();
    if len != low.len() || len != close.len() {
        return Err(TaError::LengthMismatch {
            expected: len,
            got: low.len().min(close.len()),
        });
    }
    if fastk_period == 0 || fastd_period == 0 {
        return Err(TaError::InvalidParameter {
            name: "fastk_period/fastd_period",
            value: format!("{fastk_period}/{fastd_period}"),
            reason: "periods must be >= 1",
        });
    }

    let fastd_lookback = fastd_matype.lookback(fastd_period);
    let lookback = fastk_period - 1 + fastd_lookback;
    if len <= lookback {
        return Err(TaError::InsufficientData {
            need: lookback + 1,
            got: len,
        });
    }

    let mut fastk_values = Vec::with_capacity(len - (fastk_period - 1));
    for today in (fastk_period - 1)..len {
        let start = today + 1 - fastk_period;
        let highest = high[start..=today]
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        let lowest = low[start..=today]
            .iter()
            .copied()
            .fold(f64::INFINITY, f64::min);
        let range = highest - lowest;
        fastk_values.push(if range > 0.0 {
            100.0 * (close[today] - lowest) / range
        } else {
            50.0
        });
    }

    let fastd_values = compute_ma(&fastk_values, fastd_period, fastd_matype)?;
    let mut fastk = vec![f64::NAN; len];
    let mut fastd = vec![f64::NAN; len];
    for (offset, bar) in (lookback..len).enumerate() {
        let value_index = fastd_lookback + offset;
        fastk[bar] = fastk_values[value_index];
        fastd[bar] = fastd_values[value_index];
    }
    Ok((fastk, fastd))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_selected_ma_lookback() {
        let close: Vec<f64> = (0..200)
            .map(|index| 100.0 + (index as f64 * 0.17).sin() * 8.0)
            .collect();
        let high: Vec<f64> = close.iter().map(|value| value + 1.5).collect();
        let low: Vec<f64> = close.iter().map(|value| value - 1.2).collect();
        for code in 0..=8 {
            let ma_type = MaType::try_from(code).unwrap();
            let (fastk, fastd) = stochf(&high, &low, &close, 5, 13, ma_type).unwrap();
            let expected_start = 4 + ma_type.lookback(13);
            assert!(fastk[..expected_start].iter().all(|value| value.is_nan()));
            assert!(fastd[..expected_start].iter().all(|value| value.is_nan()));
            assert!(!fastk[expected_start].is_nan());
            assert!(!fastd[expected_start].is_nan());
        }
    }
}
