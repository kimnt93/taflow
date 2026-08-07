//! Stateful Moving Average Convergence/Divergence.
//!
//! MACD aligns the fast EMA seed to the end of the slow EMA seed window, then
//! seeds the signal EMA from the first `signal_period` MACD observations.

use crate::error::{TaError, TaResult};

/// The three values produced by a warmed MACD state machine.
#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `MovingAverageConvergenceDivergenceValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct MovingAverageConvergenceDivergenceValue {
    pub macd: f64,
    pub signal: f64,
    pub histogram: f64,
}

/// Stateful MACD matching the batch function's aligned EMA seeds.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `MovingAverageConvergenceDivergence`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct MovingAverageConvergenceDivergence {
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
    warmup: Vec<f64>,
    fast_k: f64,
    slow_k: f64,
    signal_k: f64,
    fast_ema: Option<f64>,
    slow_ema: Option<f64>,
    signal_count: usize,
    signal_sum: f64,
    signal_ema: Option<f64>,
    value: Option<MovingAverageConvergenceDivergenceValue>,
}

impl MovingAverageConvergenceDivergence {
    /// Creates a MACD state with TA-Lib-compatible periods.
    pub fn new(fast_period: usize, slow_period: usize, signal_period: usize) -> TaResult<Self> {
        if fast_period < 2 || slow_period < 2 || signal_period == 0 {
            return Err(TaError::InvalidParameter {
                name: "fastperiod/slowperiod/signalperiod",
                value: format!("{fast_period}/{slow_period}/{signal_period}"),
                reason: "fastperiod >= 2, slowperiod >= 2, signalperiod >= 1",
            });
        }
        let (fast_period, slow_period) = if fast_period < slow_period {
            (fast_period, slow_period)
        } else {
            (slow_period, fast_period)
        };
        Ok(Self {
            fast_period,
            slow_period,
            signal_period,
            warmup: Vec::with_capacity(slow_period),
            fast_k: 2.0 / (fast_period as f64 + 1.0),
            slow_k: 2.0 / (slow_period as f64 + 1.0),
            signal_k: 2.0 / (signal_period as f64 + 1.0),
            fast_ema: None,
            slow_ema: None,
            signal_count: 0,
            signal_sum: 0.0,
            signal_ema: None,
            value: None,
        })
    }

    /// Appends one close value.
    pub fn append(&mut self, input: f64) -> Option<MovingAverageConvergenceDivergenceValue> {
        let macd = match (self.fast_ema, self.slow_ema) {
            (Some(fast), Some(slow)) => {
                let fast = self.fast_k.mul_add(input - fast, fast);
                let slow = self.slow_k.mul_add(input - slow, slow);
                self.fast_ema = Some(fast);
                self.slow_ema = Some(slow);
                fast - slow
            }
            _ => {
                self.warmup.push(input);
                if self.warmup.len() < self.slow_period {
                    return None;
                }
                let slow = self.warmup.iter().sum::<f64>() / self.slow_period as f64;
                let fast = self.warmup[self.slow_period - self.fast_period..]
                    .iter()
                    .sum::<f64>()
                    / self.fast_period as f64;
                self.fast_ema = Some(fast);
                self.slow_ema = Some(slow);
                fast - slow
            }
        };

        self.signal_count += 1;
        let signal = if self.signal_count < self.signal_period {
            self.signal_sum += macd;
            return None;
        } else if self.signal_count == self.signal_period {
            let seed = (self.signal_sum + macd) / self.signal_period as f64;
            self.signal_ema = Some(seed);
            seed
        } else {
            let previous = self.signal_ema.expect("signal EMA is seeded before use");
            let next = self.signal_k.mul_add(macd - previous, previous);
            self.signal_ema = Some(next);
            next
        };
        self.value = Some(MovingAverageConvergenceDivergenceValue {
            macd,
            signal,
            histogram: macd - signal,
        });
        self.value
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<MovingAverageConvergenceDivergenceValue> {
        self.value
    }

    /// Restores the post-construction state while retaining warm-up capacity.
    pub fn reset(&mut self) {
        self.warmup.clear();
        self.fast_ema = None;
        self.slow_ema = None;
        self.signal_count = 0;
        self.signal_sum = 0.0;
        self.signal_ema = None;
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_batch_and_reset_replay() {
        let input: Vec<f64> = (0..300)
            .map(|index| 100.0 + (index as f64 * 0.21).sin() * 12.0 + index as f64 * 0.01)
            .collect();
        let expected =
            crate::stream::moving_average_convergence_divergence(&input, 12, 26, 9).unwrap();
        let mut state = MovingAverageConvergenceDivergence::new(12, 26, 9).unwrap();
        for (index, input) in input.iter().copied().enumerate() {
            match state.append(input) {
                Some(actual) => {
                    assert!((actual.macd - expected.0[index]).abs() < 1e-12);
                    assert!((actual.signal - expected.1[index]).abs() < 1e-12);
                    assert!((actual.histogram - expected.2[index]).abs() < 1e-12);
                }
                None => assert!(expected.0[index].is_nan()),
            }
        }
        let expected_final = state.value();
        state.reset();
        for input in input {
            state.append(input);
        }
        assert_eq!(state.value(), expected_final);
    }
}
// Batch Moving Average Convergence/Divergence.
//
// MACD aligns the fast EMA seed to the slow EMA seed window before applying
// the signal EMA, matching TA-Lib's dedicated MACD seeding convention.

/// Computes aligned MACD, signal, and histogram arrays.
///
/// # Parameters
///
/// * `input` - Chronological close-price series.
/// * Period parameters configure fast, slow, and signal averages.
///
/// # Returns
///
/// Three aligned arrays containing MACD, signal, and histogram values.
pub fn moving_average_convergence_divergence(
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
