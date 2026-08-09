//! Stateful Kaufman Adaptive Moving Average.
//!
//! KAMA adapts its smoothing constant from the ratio between net direction
//! and total absolute movement over the configured lookback window.

use multiversion::multiversion;

use crate::error::TaResult;

use super::{invalid_period, StreamingIndicator};

const SLOW: f64 = 2.0 / 31.0;
const FAST_MINUS_SLOW: f64 = 2.0 / 3.0 - SLOW;

/// Steady-state kernel of [`KaufmanAdaptiveMovingAverage::extend_slice_into`].
///
/// Split into a free function so it can carry `#[multiversion]`: it runs two
/// `mul_add`s per bar, which a portable build without runtime dispatch lowers
/// to libm `fma()` calls. `mul_add` is explicitly fused in both cases, so the
/// dispatched variants produce bit-identical output.
#[allow(unexpected_cfgs)]
#[multiversion(targets("x86_64+avx2+fma", "x86_64+avx", "x86_64+sse4.2"))]
fn kama_steady_loop(
    inputs: &[f64],
    prologue: usize,
    period: usize,
    volatility: &mut f64,
    previous_kama: &mut Option<f64>,
    output: &mut Vec<f64>,
) {
    let mut volatility_acc = *volatility;
    let mut previous_kama_acc = *previous_kama;
    for t in prologue..inputs.len() {
        let input = inputs[t];
        let change = (input - inputs[t - 1]).abs();
        let evicted = (inputs[t - period] - inputs[t - period - 1]).abs();
        volatility_acc -= evicted;
        volatility_acc += change;

        let oldest = inputs[t - period];
        let direction = input - oldest;
        let efficiency = if volatility_acc <= direction || volatility_acc.abs() < 1.0e-14 {
            1.0
        } else {
            (direction / volatility_acc).abs()
        };
        let smoothing = efficiency.mul_add(FAST_MINUS_SLOW, SLOW);
        let previous = previous_kama_acc.unwrap_or(inputs[t - 1]);
        let next = (input - previous).mul_add(smoothing * smoothing, previous);
        previous_kama_acc = Some(next);
        output.push(next);
    }
    *volatility = volatility_acc;
    *previous_kama = previous_kama_acc;
}

/// Compute the kaufman adaptive moving average result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
/// Incremental KAMA with the same seed and recurrence as TA-Lib.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `KaufmanAdaptiveMovingAverage`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct KaufmanAdaptiveMovingAverage {
    period: usize,
    /// Ring of the last `period` inputs; slot `i % period` holds bar `i`, so the
    /// slot read before the write is exactly `inputs[i - period]`.
    prices: Box<[f64]>,
    /// Ring of the last `period` absolute bar-to-bar changes.
    changes: Box<[f64]>,
    changes_head: usize,
    changes_len: usize,
    /// Next write slot in `prices` (`index % period`, maintained by hand).
    price_slot: usize,
    /// Number of bars consumed so far.
    index: usize,
    previous_input: f64,
    volatility: f64,
    previous_kama: Option<f64>,
    value: Option<f64>,
}

impl KaufmanAdaptiveMovingAverage {
    /// Creates a KAMA state with a positive period.
    pub fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("timeperiod", period, 1));
        }
        Ok(Self {
            period,
            prices: vec![0.0; period].into_boxed_slice(),
            changes: vec![0.0; period].into_boxed_slice(),
            changes_head: 0,
            changes_len: 0,
            price_slot: 0,
            index: 0,
            previous_input: 0.0,
            volatility: 0.0,
            previous_kama: None,
            value: None,
        })
    }

    /// Folds one absolute change into the running volatility window.
    #[inline]
    fn push_change(&mut self, change: f64) {
        if self.changes_len == self.period {
            let old = self.changes[self.changes_head];
            self.volatility -= old;
            self.volatility += change;
            self.changes[self.changes_head] = change;
            self.changes_head += 1;
            if self.changes_head == self.period {
                self.changes_head = 0;
            }
        } else {
            self.volatility += change;
            // The head only starts moving once the window is full.
            self.changes[self.changes_len] = change;
            self.changes_len += 1;
        }
    }

    /// The shared KAMA step, given the lagged input and the previous close.
    #[inline]
    fn step(&self, input: f64, oldest: f64, previous: f64) -> f64 {
        let direction = input - oldest;
        let efficiency = if self.volatility <= direction || self.volatility.abs() < 1.0e-14 {
            1.0
        } else {
            (direction / self.volatility).abs()
        };
        let smoothing = efficiency.mul_add(FAST_MINUS_SLOW, SLOW);
        (input - previous).mul_add(smoothing * smoothing, previous)
    }
}

impl StreamingIndicator for KaufmanAdaptiveMovingAverage {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        if self.period == 1 {
            self.value = Some(input);
            return self.value;
        }
        let today = self.index;
        let previous_input = self.previous_input;
        if today > 0 {
            let change = (input - previous_input).abs();
            self.push_change(change);
        }
        let slot = self.price_slot;
        let oldest = self.prices[slot];
        self.prices[slot] = input;
        self.price_slot = slot + 1;
        if self.price_slot == self.period {
            self.price_slot = 0;
        }
        self.previous_input = input;
        self.index += 1;
        if today < self.period {
            return None;
        }

        // TA-Lib seeds the recurrence with the prior close.
        let previous = self.previous_kama.unwrap_or(previous_input);
        let next = self.step(input, oldest, previous);
        self.previous_kama = Some(next);
        self.value = Some(next);
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.prices.fill(0.0);
        self.changes.fill(0.0);
        self.changes_head = 0;
        self.changes_len = 0;
        self.price_slot = 0;
        self.index = 0;
        self.previous_input = 0.0;
        self.volatility = 0.0;
        self.previous_kama = None;
        self.value = None;
    }

    /// Bulk kernel: after a warm-up prologue the lagged input is just
    /// `inputs[t - period]`, so the steady loop reads the slice directly and
    /// keeps the volatility and KAMA recurrences in registers.
    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        output.reserve(inputs.len());
        if self.period == 1 {
            output.extend_from_slice(inputs);
            if let Some(&last) = inputs.last() {
                self.value = Some(last);
            }
            return;
        }
        let period = self.period;
        // Prologue: the first `period + 1` bars of the slice may still need
        // ring history from previous batches.
        let prologue = (period + 1).min(inputs.len());
        for &input in &inputs[..prologue] {
            output.push(self.append(input).unwrap_or(f64::NAN));
        }
        if inputs.len() <= prologue {
            return;
        }

        let mut volatility = self.volatility;
        let mut previous_kama = self.previous_kama;
        debug_assert_eq!(self.changes_len, period);
        kama_steady_loop(
            inputs,
            prologue,
            period,
            &mut volatility,
            &mut previous_kama,
            output,
        );

        // Exact state writeback: rings are normalized to chronological order,
        // which is behaviourally identical for every subsequent `append`.
        self.volatility = volatility;
        self.previous_kama = previous_kama;
        self.value = previous_kama;
        self.index += inputs.len() - prologue;
        self.previous_input = inputs[inputs.len() - 1];
        let end = inputs.len();
        for offset in 0..period {
            let bar = self.index - period + offset;
            self.prices[bar % period] = inputs[end - period + offset];
        }
        for offset in 0..period {
            self.changes[offset] =
                (inputs[end - period + offset] - inputs[end - period + offset - 1]).abs();
        }
        self.changes_head = 0;
        self.changes_len = period;
        self.price_slot = self.index % period;
    }
}
