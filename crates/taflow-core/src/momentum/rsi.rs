//! Batch Relative Strength Index.
//!
//! RSI uses Wilder smoothing and TA-Lib's exact output operation order and
//! epsilon convention, including zero for a flat gain/loss sum.

use crate::error::{TaError, TaResult};

/// Computes an aligned Wilder RSI output array.
pub fn relative_strength_index(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if timeperiod < 2 {
        return Err(TaError::InvalidParameter {
            name: "timeperiod",
            value: timeperiod.to_string(),
            reason: "must be >= 2",
        });
    }
    let len = input.len();
    let lookback = timeperiod;
    if len <= lookback {
        return Err(TaError::InsufficientData {
            need: lookback + 1,
            got: len,
        });
    }

    let mut output = vec![0.0_f64; len];
    output[..lookback].fill(f64::NAN);

    // 计算初始平均涨幅/跌幅
    let mut sum_gain = 0.0;
    let mut sum_loss = 0.0;
    for i in 1..=timeperiod {
        let change = input[i] - input[i - 1];
        if change > 0.0 {
            sum_gain += change;
        } else {
            sum_loss -= change; // 取绝对值
        }
    }

    let mut avg_gain = sum_gain / timeperiod as f64;
    let mut avg_loss = sum_loss / timeperiod as f64;

    // 第一个 RSI 值
    let sum = avg_gain + avg_loss;
    if sum.abs() < 1.0e-14 {
        output[timeperiod] = 0.0;
    } else {
        output[timeperiod] = 100.0 * (avg_gain / sum);
    }

    // Wilder 平滑递推
    let period_f = timeperiod as f64;
    for i in (timeperiod + 1)..len {
        let change = input[i] - input[i - 1];
        let (gain, loss) = if change > 0.0 {
            (change, 0.0)
        } else {
            (0.0, -change)
        };

        avg_gain = (avg_gain * (period_f - 1.0) + gain) / period_f;
        avg_loss = (avg_loss * (period_f - 1.0) + loss) / period_f;

        let sum = avg_gain + avg_loss;
        if sum.abs() < 1.0e-14 {
            output[i] = 0.0;
        } else {
            output[i] = 100.0 * (avg_gain / sum);
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsi_basic() {
        // 持续上涨应该 RSI 接近 100
        let input: Vec<f64> = (1..=30).map(|x| x as f64).collect();
        let result = relative_strength_index(&input, 14).unwrap();
        assert!(result[13].is_nan());
        assert!(!result[14].is_nan());
        assert!(result[14] > 90.0); // 持续上涨 RSI 应很高
    }

    #[test]
    fn test_rsi_range() {
        let input: Vec<f64> = (0..100)
            .map(|i| 50.0 + 10.0 * (i as f64 * 0.5).sin())
            .collect();
        let result = relative_strength_index(&input, 14).unwrap();
        for v in &result[14..] {
            assert!(*v >= 0.0 && *v <= 100.0, "RSI out of range: {}", v);
        }
    }

    #[test]
    fn flat_input_returns_zero() {
        let result = relative_strength_index(&vec![42.0; 100], 14).unwrap();
        assert!(result[14..].iter().all(|value| *value == 0.0));
    }
}
