use crate::error::{TaError, TaResult};

/// Internal O(n) variance calculation using rolling sum and sum-of-squares.
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

    // Sum and sum-of-squares for the initial window.
    let mut sum = 0.0_f64;
    let mut sum_sq = 0.0_f64;
    for j in 0..timeperiod {
        sum += input[j];
        sum_sq = input[j].mul_add(input[j], sum_sq);
    }
    // RollingVariance = E(X²) - E(X)² = sum_sq*inv_n - (sum*inv_n)²
    let mean = sum * inv_n;
    output[lookback] = sum_sq * inv_n - mean * mean;

    // O(1) sliding update: multiplications only, no divisions.
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

/// Internal O(n) sliding-window calculation of regression slope and intercept.
///
/// Maintains two rolling sums: `sum_y` and `ws = Σ k*v[k]` for k=0..p-1.
/// When the window advances by one position:
///   ws_new = ws_old - sum_y_old + period * input[new]
///   sum_y_new = sum_y_old - input[old] + input[new]
/// `sum_x` and `sum_x2` are constants that depend only on the period.
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

    // Constants: sum_x = 0+1+...+(p-1), sum_x2 = 0²+1²+...+(p-1)².
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
