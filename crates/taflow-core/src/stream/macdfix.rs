//! Stateful fixed-parameter Moving Average Convergence/Divergence.
//!
//! MACDFIX uses TA-Lib's fixed fast and slow smoothing constants (`0.15` and
//! `0.075`) rather than the ordinary period-derived MACD constants.

use crate::error::{TaError, TaResult};

use super::MovingAverageConvergenceDivergenceValue;

/// Incremental MACDFIX with fixed 12/26 smoothing and configurable signal EMA.
#[derive(Debug, Clone)]
pub struct MovingAverageConvergenceDivergenceFixed {
    signal_period: usize,
    warmup: Vec<f64>,
    fast_ema: Option<f64>,
    slow_ema: Option<f64>,
    signal_k: f64,
    signal_count: usize,
    signal_sum: f64,
    signal_ema: Option<f64>,
    value: Option<MovingAverageConvergenceDivergenceValue>,
}

impl MovingAverageConvergenceDivergenceFixed {
    const FAST_PERIOD: usize = 12;
    const SLOW_PERIOD: usize = 26;
    const FAST_K: f64 = 0.15;
    const SLOW_K: f64 = 0.075;

    /// Creates a MACDFIX state with a signal period of at least one.
    pub fn new(signal_period: usize) -> TaResult<Self> {
        if signal_period == 0 {
            return Err(TaError::InvalidParameter {
                name: "signalperiod",
                value: signal_period.to_string(),
                reason: "must be >= 1",
            });
        }
        Ok(Self {
            signal_period,
            warmup: Vec::with_capacity(Self::SLOW_PERIOD),
            fast_ema: None,
            slow_ema: None,
            signal_k: 2.0 / (signal_period as f64 + 1.0),
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
                let fast = Self::FAST_K.mul_add(input - fast, fast);
                let slow = Self::SLOW_K.mul_add(input - slow, slow);
                self.fast_ema = Some(fast);
                self.slow_ema = Some(slow);
                fast - slow
            }
            _ => {
                self.warmup.push(input);
                if self.warmup.len() < Self::SLOW_PERIOD {
                    return None;
                }
                let slow = self.warmup.iter().sum::<f64>() / Self::SLOW_PERIOD as f64;
                let fast = self.warmup[Self::SLOW_PERIOD - Self::FAST_PERIOD..]
                    .iter()
                    .sum::<f64>()
                    / Self::FAST_PERIOD as f64;
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
    fn matches_batch_fixed_constants_and_reset_replay() {
        let input: Vec<f64> = (0..300)
            .map(|index| 100.0 + (index as f64 * 0.21).sin() * 12.0 + index as f64 * 0.01)
            .collect();
        let expected = crate::stream::moving_average_convergence_divergence_fixed(&input, 9).unwrap();
        let mut state = MovingAverageConvergenceDivergenceFixed::new(9).unwrap();
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
// Batch fixed-parameter Moving Average Convergence/Divergence.
//
// MACDFIX uses fixed fast and slow smoothing constants of `0.15` and `0.075`
// with aligned 12/26 seeds, followed by a configurable signal EMA.

/// Compute the moving average convergence divergence fixed result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `signalperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
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
