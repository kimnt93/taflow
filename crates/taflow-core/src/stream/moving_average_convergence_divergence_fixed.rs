//! Stateful fixed-parameter Moving Average Convergence/Divergence.
//!
//! MACDFIX uses TA-Lib's fixed fast and slow smoothing constants (`0.15` and
//! `0.075`) rather than the ordinary period-derived MACD constants.

use crate::error::{TaError, TaResult};

use super::moving_average_convergence_divergence::MovingAverageConvergenceDivergenceValue;
use super::moving_average_convergence_divergence_helpers::macd_ema_steady_loop;

/// Incremental MACDFIX with fixed 12/26 smoothing and configurable signal EMA.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `MovingAverageConvergenceDivergenceFixed`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
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

    /// Bulk kernel: advances the fixed fast/slow EMAs and the signal EMA in
    /// one loop with the scalar states held in locals, writing NaN during
    /// warm-up. Bit-identical to per-bar [`Self::append`] in outputs and
    /// post-run state.
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

        let k = [Self::FAST_K, Self::SLOW_K, self.signal_k];
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
