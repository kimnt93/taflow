//! Batch fixed-parameter Moving Average Convergence/Divergence.
//!
//! MACDFIX uses fixed fast and slow smoothing constants of `0.15` and `0.075`
//! with aligned 12/26 seeds, followed by a configurable signal EMA.

use crate::error::{TaError, TaResult};

/// Computes aligned MACD, signal, and histogram arrays for TA-Lib MACDFIX.
pub fn moving_average_convergence_divergence_fixed(input: &[f64], signalperiod: usize) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if signalperiod == 0 {
        return Err(TaError::InvalidParameter {
            name: "signalperiod",
            value: signalperiod.to_string(),
            reason: "must be >= 1",
        });
    }

    let len = input.len();
    let fast_period = 12usize;
    let slow_period = 26usize;
    let fast_k: f64 = 0.15;
    let slow_k: f64 = 0.075;
    let lookback = slow_period - 1 + signalperiod - 1;
    if len <= lookback {
        return Err(TaError::InsufficientData {
            need: lookback + 1,
            got: len,
        });
    }

    let signal_k = 2.0 / (signalperiod as f64 + 1.0);
    let slow_seed = input[..slow_period].iter().sum::<f64>() / slow_period as f64;
    let fast_seed = input[slow_period - fast_period..slow_period]
        .iter()
        .sum::<f64>()
        / fast_period as f64;

    let mut macd_values = Vec::with_capacity(len - slow_period + 1);
    macd_values.push(fast_seed - slow_seed);
    let mut slow_ema = slow_seed;
    let mut fast_ema = fast_seed;
    for input in &input[slow_period..] {
        slow_ema = slow_k.mul_add(*input - slow_ema, slow_ema);
        fast_ema = fast_k.mul_add(*input - fast_ema, fast_ema);
        macd_values.push(fast_ema - slow_ema);
    }

    let signal_seed = macd_values[..signalperiod].iter().sum::<f64>() / signalperiod as f64;
    let out_start = slow_period - 1 + signalperiod - 1;
    let mut macd_line = vec![f64::NAN; len];
    let mut signal_line = vec![f64::NAN; len];
    let mut histogram = vec![f64::NAN; len];

    let mut signal_ema = signal_seed;
    let macd_at_start = macd_values[signalperiod - 1];
    macd_line[out_start] = macd_at_start;
    signal_line[out_start] = signal_seed;
    histogram[out_start] = macd_at_start - signal_seed;

    for (index, macd) in macd_values.iter().copied().enumerate().skip(signalperiod) {
        let bar = slow_period - 1 + index;
        signal_ema = signal_k.mul_add(macd - signal_ema, signal_ema);
        macd_line[bar] = macd;
        signal_line[bar] = signal_ema;
        histogram[bar] = macd - signal_ema;
    }

    Ok((macd_line, signal_line, histogram))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_zero_signal_period() {
        assert!(moving_average_convergence_divergence_fixed(&[1.0; 30], 0).is_err());
    }
}
