//! Stateful Absolute Price Oscillator.
//!
//! APO subtracts a slow moving average from a fast moving average and supports
//! all nine TA-Lib moving-average types with their native warm-up semantics.

use multiversion::multiversion;

use crate::error::TaResult;
use crate::ma_type::MaType;

use super::{moving_average_dispatcher::MovingAverageDispatcher, StreamingIndicator};

/// Steady-state kernel for the fused fast/slow EMA legs.
///
/// Extracted from [`AbsolutePriceOscillator::extend_slice_into`] so it can
/// carry `#[multiversion]`; a portable build without runtime dispatch lowers
/// each `mul_add` to a libm `fma()` call. `mul_add` is explicitly fused in both
/// cases, so the dispatched variants are bit-identical.
#[allow(unexpected_cfgs)]
#[multiversion(targets("x86_64+avx2+fma", "x86_64+avx", "x86_64+sse4.2"))]
fn apo_ema_steady_loop(
    inputs: &[f64],
    fast_k: f64,
    slow_k: f64,
    state: &mut (f64, f64),
    output: &mut Vec<f64>,
) -> f64 {
    let (mut fast, mut slow) = *state;
    let mut last = f64::NAN;
    for &input in inputs {
        fast = fast_k.mul_add(input - fast, fast);
        slow = slow_k.mul_add(input - slow, slow);
        last = fast - slow;
        output.push(last);
    }
    *state = (fast, slow);
    last
}

/// Incremental APO driven by two selected moving-average states.
/// Persistent Rust state or aligned output type for `AbsolutePriceOscillator`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct AbsolutePriceOscillator {
    fast: MovingAverageDispatcher,
    slow: MovingAverageDispatcher,
    value: Option<f64>,
}

impl AbsolutePriceOscillator {
    /// Creates an APO state; period order is normalized to fast then slow.
    pub fn new(fast_period: usize, slow_period: usize, ma_type: MaType) -> TaResult<Self> {
        let (fast_period, slow_period) = if fast_period < slow_period {
            (fast_period, slow_period)
        } else {
            (slow_period, fast_period)
        };
        Ok(Self {
            fast: MovingAverageDispatcher::new(fast_period, ma_type)?,
            slow: MovingAverageDispatcher::new(slow_period, ma_type)?,
            value: None,
        })
    }
}

impl StreamingIndicator for AbsolutePriceOscillator {
    type Output = f64;

    /// Bulk kernel. For the EMA and SMA MA types (SMA is TA-Lib's `APO`
    /// default) the warm steady state advances both recurrences in one loop
    /// with the scalar states held in locals; other MA types fall back to a
    /// per-bar loop with no per-bar allocation. Bit-identical to per-bar
    /// [`Self::append`] in outputs and post-run state.
    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        output.reserve(inputs.len());
        let mut index = 0;
        if self.fast.is_sma() && self.slow.is_sma() {
            let fast_period = self.fast.as_sma_mut().expect("SMA fast state").period();
            let slow_period = self.slow.as_sma_mut().expect("SMA slow state").period();
            // `new` normalizes the periods, so `fast_period <= slow_period` and
            // `slow_period` per-bar appends leave both rings holding nothing
            // but bars of this slice - after which the evicted element of each
            // is just `inputs[i - period]`.
            let n = inputs.len();
            let prologue = n.min(slow_period);
            for &input in &inputs[..prologue] {
                output.push(self.append(input).unwrap_or(f64::NAN));
            }
            if n <= slow_period {
                return;
            }
            let mut fast_sum = self.fast.as_sma_mut().expect("SMA fast state").raw_sum();
            let mut slow_sum = self.slow.as_sma_mut().expect("SMA slow state").raw_sum();
            let fast_len = fast_period as f64;
            let slow_len = slow_period as f64;
            let mut last = f64::NAN;
            for i in slow_period..n {
                // Same statement order as `SimpleMovingAverage::append`
                // (`sum -= old` then `sum += input`) on both legs.
                fast_sum -= inputs[i - fast_period];
                fast_sum += inputs[i];
                slow_sum -= inputs[i - slow_period];
                slow_sum += inputs[i];
                last = fast_sum / fast_len - slow_sum / slow_len;
                output.push(last);
            }
            MovingAverageDispatcher::restore_sma_leg(
                self.fast.as_sma_mut().expect("SMA fast state"),
                inputs,
                fast_sum,
            );
            MovingAverageDispatcher::restore_sma_leg(
                self.slow.as_sma_mut().expect("SMA slow state"),
                inputs,
                slow_sum,
            );
            self.value = Some(last);
            return;
        }
        if self.fast.is_ema() && self.slow.is_ema() {
            // Warm-up prologue: per-bar appends until the slow EMA is seeded.
            while index < inputs.len() && self.value.is_none() {
                output.push(self.append(inputs[index]).unwrap_or(f64::NAN));
                index += 1;
            }
            if index < inputs.len() {
                let (fast_k, mut fast) = {
                    let state = self.fast.as_ema_mut().expect("EMA fast state");
                    (state.smoothing(), state.current().expect("warm fast EMA"))
                };
                let (slow_k, mut slow) = {
                    let state = self.slow.as_ema_mut().expect("EMA slow state");
                    (state.smoothing(), state.current().expect("warm slow EMA"))
                };
                let mut ema_state = (fast, slow);
                let last =
                    apo_ema_steady_loop(&inputs[index..], fast_k, slow_k, &mut ema_state, output);
                (fast, slow) = ema_state;
                let appended = inputs.len() - index;
                self.fast
                    .as_ema_mut()
                    .expect("EMA fast state")
                    .store_bulk_state(fast, appended);
                self.slow
                    .as_ema_mut()
                    .expect("EMA slow state")
                    .store_bulk_state(slow, appended);
                self.value = Some(last);
            }
            return;
        }
        for &input in &inputs[index..] {
            output.push(self.append(input).unwrap_or(f64::NAN));
        }
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        let fast = self.fast.append(input);
        let slow = self.slow.append(input);
        self.value = fast.zip(slow).map(|(fast, slow)| fast - slow);
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.fast.reset();
        self.slow.reset();
        self.value = None;
    }
}
