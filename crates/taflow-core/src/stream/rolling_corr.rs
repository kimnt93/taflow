//! Batch implementation for `rolling_corr`.

use super::statistic::*;
use crate::error::{TaError, TaResult};

/// Pearson's Correlation Coefficient (CORREL) — O(n) 滑动窗口算法
///
/// 使用恒等式: correl = (n*sxy - sx*sy) / sqrt((n*sxx - sx²) * (n*syy - sy²))
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

    // 初始化第一个窗口 [0..timeperiod] 的滑动求和
    let init_x = &input0[0..timeperiod];
    let init_y = &input1[0..timeperiod];
    let mut sx = crate::simd::sum_f64(init_x);
    let mut sy = crate::simd::sum_f64(init_y);
    let mut sxx: f64 = init_x.iter().map(|&v| v * v).sum();
    let mut syy: f64 = init_y.iter().map(|&v| v * v).sum();
    let mut sxy: f64 = init_x.iter().zip(init_y.iter()).map(|(&x, &y)| x * y).sum();

    let num = n * sxy - sx * sy;
    let denom = ((n * sxx - sx * sx) * (n * syy - sy * sy)).sqrt();
    output[lookback] = if denom > 0.0 { num / denom } else { 0.0 };

    // 滑动窗口
    for i in timeperiod..len {
        let old_x = input0[i - timeperiod];
        let old_y = input1[i - timeperiod];
        let new_x = input0[i];
        let new_y = input1[i];

        sx += new_x - old_x;
        sy += new_y - old_y;
        sxx += new_x * new_x - old_x * old_x;
        syy += new_y * new_y - old_y * old_y;
        sxy += new_x * new_y - old_x * old_y;

        let num = n * sxy - sx * sy;
        let denom = ((n * sxx - sx * sx) * (n * syy - sy * sy)).sqrt();
        output[i] = if denom > 0.0 { num / denom } else { 0.0 };
    }
    Ok(output)
}
