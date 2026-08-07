//! Batch implementation for `rolling_beta`.

use super::statistic::*;
use crate::error::{TaError, TaResult};

/// RollingBeta — O(n) 滑动窗口算法
///
/// C TA-Lib BETA 使用百分比收益率:
///   x[i] = (input0[i] - input0[i-1]) / input0[i-1]  (input0 的百分比收益)
///   y[i] = (input1[i] - input1[i-1]) / input1[i-1]  (input1 的百分比收益)
///   beta = (n*sxy - sx*sy) / (n*sxx - sx*sx)
///
/// 其中 input0 为基准 (market)，input1 为标的 (stock)，
/// 分母是 rolling_var(x)=rolling_var(input0 的收益率)。
/// Compute the rolling beta result for the supplied aligned series.
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
pub fn rolling_beta(input0: &[f64], input1: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let len = input0.len();
    if len != input1.len() {
        return Err(TaError::LengthMismatch {
            expected: len,
            got: input1.len(),
        });
    }
    if len <= timeperiod {
        return Err(TaError::InsufficientData {
            need: timeperiod + 1,
            got: len,
        });
    }
    let mut output = vec![0.0_f64; len];
    output[..timeperiod].fill(f64::NAN);
    let n = timeperiod as f64;

    // 预计算百分比收益率（一次分配，避免每个窗口重复除法）
    // rx[i] 对应原始 index i+1 的收益率 (即 rx[i] = (input0[i+1]-input0[i])/input0[i])
    // 收益率序列长度 = len - 1
    let ret_len = len - 1;
    let mut rx = vec![0.0_f64; ret_len];
    let mut ry = vec![0.0_f64; ret_len];
    for j in 0..ret_len {
        rx[j] = (input0[j + 1] - input0[j]) / input0[j];
        ry[j] = (input1[j + 1] - input1[j]) / input1[j];
    }

    // 滑动窗口: 对于输出 index i (i >= timeperiod)，
    // 使用收益率 rx[i-timeperiod..i] (即原始 indices (i-timeperiod+1)..=i 的收益率)
    let mut sx = 0.0_f64;
    let mut sy = 0.0_f64;
    let mut sxx = 0.0_f64;
    let mut sxy = 0.0_f64;

    // 初始化第一个窗口: rx[0..timeperiod] 对应原始 output[timeperiod]
    for j in 0..timeperiod {
        let x = rx[j];
        let y = ry[j];
        sx += x;
        sy += y;
        sxx += x * x;
        sxy += x * y;
    }

    let denom = n * sxx - sx * sx;
    output[timeperiod] = if denom > 0.0 {
        (n * sxy - sx * sy) / denom
    } else {
        0.0
    };

    // 滑动
    for i in (timeperiod + 1)..len {
        // 移除 rx[i - timeperiod - 1]，加入 rx[i - 1]
        let old_idx = i - timeperiod - 1;
        let new_idx = i - 1;
        let ox = rx[old_idx];
        let oy = ry[old_idx];
        let nx = rx[new_idx];
        let ny = ry[new_idx];
        sx += nx - ox;
        sy += ny - oy;
        sxx += nx * nx - ox * ox;
        sxy += nx * ny - ox * oy;

        let denom = n * sxx - sx * sx;
        output[i] = if denom > 0.0 {
            (n * sxy - sx * sy) / denom
        } else {
            0.0
        };
    }
    Ok(output)
}
