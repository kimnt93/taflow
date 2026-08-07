//! Stateful Moving Average Convergence/Divergence.
//!
//! MACD aligns the fast EMA seed to the end of the slow EMA seed window, then
//! seeds the signal EMA from the first `signal_period` MACD observations.

use crate::error::{TaError, TaResult};

/// The three values produced by a warmed MACD state machine.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MacdValue {
    pub macd: f64,
    pub signal: f64,
    pub histogram: f64,
}

/// Stateful MACD matching the batch function's aligned EMA seeds.
#[derive(Debug, Clone)]
pub struct Macd {
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
    value: Option<MacdValue>,
}

impl Macd {
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
    pub fn append(&mut self, input: f64) -> Option<MacdValue> {
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
        self.value = Some(MacdValue {
            macd,
            signal,
            histogram: macd - signal,
        });
        self.value
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<MacdValue> {
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
    use crate::momentum;

    #[test]
    fn matches_batch_and_reset_replay() {
        let input: Vec<f64> = (0..300)
            .map(|index| 100.0 + (index as f64 * 0.21).sin() * 12.0 + index as f64 * 0.01)
            .collect();
        let expected = momentum::moving_average_convergence_divergence(&input, 12, 26, 9).unwrap();
        let mut state = Macd::new(12, 26, 9).unwrap();
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
