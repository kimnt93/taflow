//! Stateful Kaufman Adaptive Moving Average.
//!
//! KAMA adapts its smoothing constant from the ratio between net direction
//! and total absolute movement over the configured lookback window.

use crate::error::TaResult;

use super::{invalid_period, StreamingIndicator};

const SLOW: f64 = 2.0 / 31.0;
const FAST_MINUS_SLOW: f64 = 2.0 / 3.0 - SLOW;

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
pub fn kaufman_adaptive_moving_average(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = KaufmanAdaptiveMovingAverage::new(timeperiod)?;
    let mut output = Vec::with_capacity(input.len());
    state.extend_slice_into(input, &mut output);
    Ok(output)
}

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
        for t in prologue..inputs.len() {
            let input = inputs[t];
            let change = (input - inputs[t - 1]).abs();
            let evicted = (inputs[t - period] - inputs[t - period - 1]).abs();
            volatility -= evicted;
            volatility += change;

            let oldest = inputs[t - period];
            let direction = input - oldest;
            let efficiency = if volatility <= direction || volatility.abs() < 1.0e-14 {
                1.0
            } else {
                (direction / volatility).abs()
            };
            let smoothing = efficiency.mul_add(FAST_MINUS_SLOW, SLOW);
            let previous = previous_kama.unwrap_or(inputs[t - 1]);
            let next = (input - previous).mul_add(smoothing * smoothing, previous);
            previous_kama = Some(next);
            output.push(next);
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    fn lcg_series(len: usize, mut seed: u64) -> Vec<f64> {
        (0..len)
            .map(|_| {
                seed = seed
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                100.0 + ((seed >> 11) as f64 / (1u64 << 53) as f64 - 0.5) * 20.0
            })
            .collect()
    }

    /// Verbatim copy of the pre-optimization `VecDeque` implementation.
    mod oracle {
        use std::collections::VecDeque;

        pub struct Kama {
            period: usize,
            prices: VecDeque<f64>,
            changes: VecDeque<f64>,
            volatility: f64,
            previous_kama: Option<f64>,
            pub value: Option<f64>,
        }

        impl Kama {
            pub fn new(period: usize) -> Self {
                Self {
                    period,
                    prices: VecDeque::with_capacity(period + 1),
                    changes: VecDeque::with_capacity(period),
                    volatility: 0.0,
                    previous_kama: None,
                    value: None,
                }
            }

            pub fn append(&mut self, input: f64) -> Option<f64> {
                if self.period == 1 {
                    self.value = Some(input);
                    return self.value;
                }
                if let Some(previous) = self.prices.back().copied() {
                    let change = (input - previous).abs();
                    if self.changes.len() == self.period {
                        let old = self.changes.pop_front().expect("change window is full");
                        self.volatility -= old;
                        self.volatility += change;
                    } else {
                        self.volatility += change;
                    }
                    self.changes.push_back(change);
                }
                if self.prices.len() == self.period + 1 {
                    self.prices.pop_front();
                }
                self.prices.push_back(input);
                if self.prices.len() < self.period + 1 {
                    return None;
                }
                let oldest = *self.prices.front().expect("full price window has a front");
                let direction = input - oldest;
                let efficiency = if self.volatility <= direction || self.volatility.abs() < 1.0e-14
                {
                    1.0
                } else {
                    (direction / self.volatility).abs()
                };
                let slow = 2.0 / 31.0;
                let smoothing = efficiency.mul_add(2.0 / 3.0 - slow, slow);
                let previous = self
                    .previous_kama
                    .unwrap_or_else(|| self.prices[self.period - 1]);
                let next = (input - previous).mul_add(smoothing * smoothing, previous);
                self.previous_kama = Some(next);
                self.value = Some(next);
                self.value
            }
        }
    }

    fn oracle_outputs(input: &[f64], period: usize) -> Vec<Option<f64>> {
        let mut state = oracle::Kama::new(period);
        input.iter().map(|&bar| state.append(bar)).collect()
    }

    #[test]
    fn append_matches_oracle_bitwise() {
        let input = lcg_series(5_000, 0x51ed_1234_9876_0001);
        for period in [1_usize, 2, 3, 10, 30] {
            let expected = oracle_outputs(&input, period);
            let mut state = KaufmanAdaptiveMovingAverage::new(period).unwrap();
            for (index, (&bar, want)) in input.iter().zip(&expected).enumerate() {
                let got = state.append(bar);
                assert_eq!(
                    got.map(f64::to_bits),
                    want.map(f64::to_bits),
                    "period {period} bar {index}"
                );
            }
        }
    }

    #[test]
    fn bulk_and_chunked_match_append_bitwise() {
        let input = lcg_series(5_000, 0x51ed_1234_9876_0002);
        for period in [1_usize, 2, 3, 10, 30] {
            let expected: Vec<f64> = oracle_outputs(&input, period)
                .into_iter()
                .map(|value| value.unwrap_or(f64::NAN))
                .collect();

            let bulk = kaufman_adaptive_moving_average(&input, period).unwrap();
            for (index, (got, want)) in bulk.iter().zip(&expected).enumerate() {
                assert_eq!(
                    got.to_bits(),
                    want.to_bits(),
                    "bulk period {period} @{index}"
                );
            }

            for chunk in [1_usize, 7, 97] {
                let mut state = KaufmanAdaptiveMovingAverage::new(period).unwrap();
                let mut output = Vec::new();
                for window in input.chunks(chunk) {
                    state.extend_slice_into(window, &mut output);
                }
                for (index, (got, want)) in output.iter().zip(&expected).enumerate() {
                    assert_eq!(
                        got.to_bits(),
                        want.to_bits(),
                        "chunk {chunk} period {period} @{index}"
                    );
                }
                assert_eq!(
                    state.value().map(f64::to_bits),
                    expected.last().map(|value| value.to_bits())
                );
            }
        }
    }

    #[test]
    fn continue_after_bulk_matches_append() {
        let input = lcg_series(5_000, 0x51ed_1234_9876_0003);
        for period in [2_usize, 5, 30] {
            let expected = oracle_outputs(&input, period);
            let split = 3_001;
            let mut state = KaufmanAdaptiveMovingAverage::new(period).unwrap();
            let mut output = Vec::new();
            state.extend_slice_into(&input[..split], &mut output);
            for (index, (&bar, want)) in input[split..].iter().zip(&expected[split..]).enumerate() {
                let got = state.append(bar);
                assert_eq!(
                    got.map(f64::to_bits),
                    want.map(f64::to_bits),
                    "period {period} continuation bar {index}"
                );
            }
        }
    }

    #[test]
    fn reset_restores_initial_behaviour() {
        let input = lcg_series(500, 0x51ed_1234_9876_0004);
        let mut state = KaufmanAdaptiveMovingAverage::new(9).unwrap();
        let first: Vec<Option<f64>> = input.iter().map(|&bar| state.append(bar)).collect();
        state.reset();
        assert!(state.value().is_none());
        let second: Vec<Option<f64>> = input.iter().map(|&bar| state.append(bar)).collect();
        for (a, b) in first.iter().zip(&second) {
            assert_eq!(a.map(f64::to_bits), b.map(f64::to_bits));
        }
    }
}
