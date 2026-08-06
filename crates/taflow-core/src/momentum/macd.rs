//! Batch Moving Average Convergence/Divergence.
//!
//! MACD aligns the fast EMA seed to the slow EMA seed window before applying
//! the signal EMA, matching TA-Lib's dedicated MACD seeding convention.

use crate::error::{TaError, TaResult};

/// Computes aligned MACD, signal, and histogram arrays.
pub fn macd(
    input: &[f64],
    fastperiod: usize,
    slowperiod: usize,
    signalperiod: usize,
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if fastperiod < 2 || slowperiod < 2 || signalperiod < 1 {
        return Err(TaError::InvalidParameter {
            name: "fastperiod/slowperiod/signalperiod",
            value: format!("{}/{}/{}", fastperiod, slowperiod, signalperiod),
            reason: "fastperiod >= 2, slowperiod >= 2, signalperiod >= 1",
        });
    }

    // 确保 slow > fast
    let (fp, sp) = if fastperiod < slowperiod {
        (fastperiod, slowperiod)
    } else {
        (slowperiod, fastperiod)
    };

    let len = input.len();
    let lookback = sp - 1 + signalperiod - 1;
    if len <= lookback {
        return Err(TaError::InsufficientData {
            need: lookback + 1,
            got: len,
        });
    }

    let k_fast = 2.0 / (fp as f64 + 1.0);
    let k_slow = 2.0 / (sp as f64 + 1.0);
    let k_signal = 2.0 / (signalperiod as f64 + 1.0);

    // C TA-Lib MACD 内部 EMA 计算：
    // slow seed = SMA(close[0..sp]), fast seed = SMA(close[sp-fp..sp])
    let slow_seed: f64 = input[..sp].iter().sum::<f64>() / sp as f64;
    let fast_seed: f64 = input[sp - fp..sp].iter().sum::<f64>() / fp as f64;

    // MACD line: 第一个值 (对应 bar sp-1) = fast_seed - slow_seed
    // 后续从 bar sp 开始递推
    let mut macd_values = Vec::with_capacity(len - sp + 1);
    macd_values.push(fast_seed - slow_seed);

    let mut slow_ema = slow_seed;
    let mut fast_ema = fast_seed;
    for i in sp..len {
        slow_ema = k_slow.mul_add(input[i] - slow_ema, slow_ema);
        fast_ema = k_fast.mul_add(input[i] - fast_ema, fast_ema);
        macd_values.push(fast_ema - slow_ema);
    }

    // Signal line = EMA(macd_values, signalperiod)
    // seed = SMA(macd_values[0..signalperiod])
    let signal_seed: f64 = macd_values[..signalperiod].iter().sum::<f64>() / signalperiod as f64;

    // 构建输出
    let out_start = sp - 1 + signalperiod - 1; // = lookback
    let mut macd_line = vec![0.0_f64; len];
    macd_line[..out_start].fill(f64::NAN);
    let mut signal_line = vec![0.0_f64; len];
    signal_line[..out_start].fill(f64::NAN);
    let mut histogram = vec![0.0_f64; len];
    histogram[..out_start].fill(f64::NAN);

    // signal 第一个值对应 macd_values[signalperiod-1]，即 bar out_start
    let mut signal_ema = signal_seed;
    let macd_at_out_start = macd_values[signalperiod - 1];
    macd_line[out_start] = macd_at_out_start;
    signal_line[out_start] = signal_seed;
    histogram[out_start] = macd_at_out_start - signal_seed;

    for i in signalperiod..macd_values.len() {
        let bar = sp - 1 + i;
        signal_ema = k_signal.mul_add(macd_values[i] - signal_ema, signal_ema);
        macd_line[bar] = macd_values[i];
        signal_line[bar] = signal_ema;
        histogram[bar] = macd_values[i] - signal_ema;
    }

    Ok((macd_line, signal_line, histogram))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macd_basic() {
        let input: Vec<f64> = (1..=50).map(|x| x as f64).collect();
        let (macd_line, signal, hist) = macd(&input, 12, 26, 9).unwrap();
        // C TA-Lib: all three outputs start at index slowperiod-1 + signalperiod-1 = 25+8 = 33
        assert!(macd_line[32].is_nan());
        assert!(!macd_line[33].is_nan());
        assert!(signal[32].is_nan());
        assert!(!signal[33].is_nan());
        assert!(hist[32].is_nan());
        assert!(!hist[33].is_nan());
    }
}
