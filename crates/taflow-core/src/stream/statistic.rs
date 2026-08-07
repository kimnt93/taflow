use crate::error::{TaError, TaResult};

/// 内部: O(n) 方差计算 (滑动窗口 sum + sum_sq)
pub(crate) fn var_internal(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if timeperiod < 2 {
        return Err(TaError::InvalidParameter {
            name: "timeperiod",
            value: timeperiod.to_string(),
            reason: "must be >= 2",
        });
    }
    let len = input.len();
    if len < timeperiod {
        return Err(TaError::InsufficientData {
            need: timeperiod,
            got: len,
        });
    }

    let lookback = timeperiod - 1;
    let mut output = vec![0.0_f64; len];
    output[..lookback].fill(f64::NAN);
    let inv_n = 1.0 / timeperiod as f64; // precompute: replace 2 div/iter → 2 mul/iter

    // 初始窗口的 sum 和 sum_sq
    let mut sum = 0.0_f64;
    let mut sum_sq = 0.0_f64;
    for j in 0..timeperiod {
        sum += input[j];
        sum_sq = input[j].mul_add(input[j], sum_sq);
    }
    // RollingVariance = E(X²) - E(X)² = sum_sq*inv_n - (sum*inv_n)²
    let mean = sum * inv_n;
    output[lookback] = sum_sq * inv_n - mean * mean;

    // O(1) 滑动 — all multiplies, zero divisions
    for i in timeperiod..len {
        let old = input[i - timeperiod];
        let new_val = input[i];
        sum += new_val - old;
        sum_sq += (new_val - old).mul_add(new_val + old, 0.0); // (new²-old²) = (new-old)(new+old)
        let mean = sum * inv_n;
        output[i] = sum_sq * inv_n - mean * mean;
    }

    Ok(output)
}

/// 内部: 计算线性回归的斜率和截距 — O(n) 滑动窗口算法
///
/// 维护两个滑动和: sum_y (值的和) 和 ws (加权和, ws = Σ k*v[k] for k=0..p-1)
/// 当窗口右移一位时:
///   ws_new = ws_old - sum_y_old + period * input[new]
///   sum_y_new = sum_y_old - input[old] + input[new]
/// 其中 sum_x 和 sum_x2 为常量 (仅依赖 period)。
pub(crate) fn linearreg_components(
    input: &[f64],
    timeperiod: usize,
) -> TaResult<(Vec<f64>, Vec<f64>)> {
    if timeperiod < 2 {
        return Err(TaError::InvalidParameter {
            name: "timeperiod",
            value: timeperiod.to_string(),
            reason: "must be >= 2",
        });
    }
    let len = input.len();
    if len < timeperiod {
        return Err(TaError::InsufficientData {
            need: timeperiod,
            got: len,
        });
    }

    let lookback = timeperiod - 1;
    let mut slope = vec![0.0_f64; len];
    slope[..lookback].fill(f64::NAN);
    let mut intercept = vec![0.0_f64; len];
    intercept[..lookback].fill(f64::NAN);
    let n = timeperiod as f64;

    // 常量: sum_x = 0+1+...+(p-1), sum_x2 = 0²+1²+...+(p-1)²
    let sum_x = n * (n - 1.0) / 2.0;
    let sum_x2 = n * (n - 1.0) * (2.0 * n - 1.0) / 6.0;
    let denom = n * sum_x2 - sum_x * sum_x;

    // Recompute each bounded window in chronological order. This matches the
    // TA-Lib summation order and avoids drift from subtractive rolling sums.
    for i in lookback..len {
        let window = &input[i + 1 - timeperiod..=i];
        let mut sum_y = 0.0;
        let mut ws = 0.0;
        for (k, &value) in window.iter().enumerate() {
            sum_y += value;
            ws += k as f64 * value;
        }
        if denom != 0.0 {
            let m = (n * ws - sum_x * sum_y) / denom;
            let b = (sum_y - m * sum_x) / n;
            slope[i] = m;
            intercept[i] = b;
        }
    }

    Ok((slope, intercept))
}
