//! Stateful Moving Average Convergence/Divergence.
//!
//! MACD aligns the fast EMA seed to the end of the slow EMA seed window, then
//! seeds the signal EMA from the first `signal_period` MACD observations.

use crate::error::{TaError, TaResult};

use super::moving_average_convergence_divergence_helpers::macd_ema_steady_loop;

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

    /// Bulk kernel: advances the fast, slow, and signal EMA recurrences in one
    /// loop with the scalar states held in locals, writing NaN during warm-up.
    /// Bit-identical to per-bar [`Self::append`] in outputs and post-run state.
    pub fn extend_slices_into(
        &mut self,
        inputs: &[f64],
        macd_out: &mut Vec<f64>,
        signal_out: &mut Vec<f64>,
        histogram_out: &mut Vec<f64>,
    ) {
        macd_out.reserve(inputs.len());
        signal_out.reserve(inputs.len());
        histogram_out.reserve(inputs.len());
        let mut index = 0;
        // Warm-up prologue: per-bar appends until the signal EMA is seeded.
        while index < inputs.len() && self.signal_ema.is_none() {
            match self.append(inputs[index]) {
                Some(value) => {
                    macd_out.push(value.macd);
                    signal_out.push(value.signal);
                    histogram_out.push(value.histogram);
                }
                None => {
                    macd_out.push(f64::NAN);
                    signal_out.push(f64::NAN);
                    histogram_out.push(f64::NAN);
                }
            }
            index += 1;
        }
        if index == inputs.len() {
            return;
        }

        let k = [self.fast_k, self.slow_k, self.signal_k];
        let mut state = [
            self.fast_ema.expect("warm fast EMA"),
            self.slow_ema.expect("warm slow EMA"),
            self.signal_ema.expect("warm signal EMA"),
        ];
        let last = macd_ema_steady_loop(
            &inputs[index..],
            k,
            &mut state,
            macd_out,
            signal_out,
            histogram_out,
        )
        .or(self.value);

        self.fast_ema = Some(state[0]);
        self.slow_ema = Some(state[1]);
        self.signal_ema = Some(state[2]);
        self.signal_count += inputs.len() - index;
        self.value = last;
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
