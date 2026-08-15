//! Stateful Hilbert Transform instantaneous trendline.
//!
//! The state advances TA-Lib's four-bar price smoother, alternating Hilbert
//! transforms, dominant-cycle estimate, cycle-length price average, and final
//! four-value weighted trendline without recomputing prior bars.

use crate::stream::cycle::{do_hilbert_even, do_hilbert_odd, HilbertVars};

const RAD2DEG: f64 = 180.0 / std::f64::consts::PI;
const LOOKBACK: usize = 63;
const PRICE_RING: usize = 50;

/// Incremental HT_TRENDLINE state.
/// Persistent Rust state or aligned output type for `HilbertTransformTrendline`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct HilbertTransformTrendline {
    index: usize,
    /// Fixed ring of the last 50 raw prices; `price_head` is the next write slot.
    prices: [f64; PRICE_RING],
    price_head: usize,
    price_count: usize,
    /// Three-slot delay line feeding the running WMA(4) smoother.
    wma_prices: [f64; 3],
    period_wma_sub: f64,
    period_wma_sum: f64,
    trailing_wma_value: f64,
    hilbert_idx: usize,
    detrender_vars: HilbertVars,
    q1_vars: HilbertVars,
    ji_vars: HilbertVars,
    jq_vars: HilbertVars,
    period: f64,
    smooth_period: f64,
    prev_i2: f64,
    prev_q2: f64,
    re: f64,
    im: f64,
    i1_for_odd_prev2: f64,
    i1_for_odd_prev3: f64,
    i1_for_even_prev2: f64,
    i1_for_even_prev3: f64,
    trend1: f64,
    trend2: f64,
    trend3: f64,
    value: Option<f64>,
}

impl Default for HilbertTransformTrendline {
    fn default() -> Self {
        Self::new()
    }
}

impl HilbertTransformTrendline {
    /// Creates an empty HT_TRENDLINE state.
    pub fn new() -> Self {
        Self {
            index: 0,
            prices: [0.0; PRICE_RING],
            price_head: 0,
            price_count: 0,
            wma_prices: [0.0; 3],
            period_wma_sub: 0.0,
            period_wma_sum: 0.0,
            trailing_wma_value: 0.0,
            hilbert_idx: 0,
            detrender_vars: HilbertVars::new(),
            q1_vars: HilbertVars::new(),
            ji_vars: HilbertVars::new(),
            jq_vars: HilbertVars::new(),
            period: 0.0,
            smooth_period: 0.0,
            prev_i2: 0.0,
            prev_q2: 0.0,
            re: 0.0,
            im: 0.0,
            i1_for_odd_prev2: 0.0,
            i1_for_odd_prev3: 0.0,
            i1_for_even_prev2: 0.0,
            i1_for_even_prev3: 0.0,
            trend1: 0.0,
            trend2: 0.0,
            trend3: 0.0,
            value: None,
        }
    }

    fn next_smoothed(&mut self, input: f64) -> Option<f64> {
        if self.index < 2 {
            self.wma_prices[self.index] = input;
            return None;
        }
        if self.index == 2 {
            self.wma_prices[2] = input;
            self.period_wma_sub = self.wma_prices[0];
            self.period_wma_sub += self.wma_prices[1];
            self.period_wma_sub += self.wma_prices[2];
            self.period_wma_sum = self.wma_prices[0];
            self.period_wma_sum += self.wma_prices[1] * 2.0;
            self.period_wma_sum += self.wma_prices[2] * 3.0;
            return None;
        }

        self.period_wma_sub += input;
        self.period_wma_sub -= self.trailing_wma_value;
        self.period_wma_sum += input * 4.0;
        let slot = self.index % 3;
        self.trailing_wma_value = self.wma_prices[slot];
        self.wma_prices[slot] = input;
        let smoothed = self.period_wma_sum * 0.1;
        self.period_wma_sum -= self.period_wma_sub;
        Some(smoothed)
    }

    /// Appends one price value.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        let today = self.index;
        let smoothed = self.next_smoothed(input);
        self.index += 1;
        self.prices[self.price_head] = input;
        self.price_head = (self.price_head + 1) % PRICE_RING;
        if self.price_count < PRICE_RING {
            self.price_count += 1;
        }

        // TA-Lib computes and discards 34 WMA values at bars 3 through 36.
        if today < 37 {
            return None;
        }
        let smoothed = smoothed.expect("the WMA is initialized after bar 2");
        let adjusted_prev_period = 0.075 * self.period + 0.54;

        let (i2, q2);
        if today % 2 == 0 {
            let detrender = do_hilbert_even(
                &mut self.detrender_vars,
                smoothed,
                self.hilbert_idx,
                adjusted_prev_period,
            );
            let q1 = do_hilbert_even(
                &mut self.q1_vars,
                detrender,
                self.hilbert_idx,
                adjusted_prev_period,
            );
            let ji = do_hilbert_even(
                &mut self.ji_vars,
                self.i1_for_even_prev3,
                self.hilbert_idx,
                adjusted_prev_period,
            );
            let jq = do_hilbert_even(
                &mut self.jq_vars,
                q1,
                self.hilbert_idx,
                adjusted_prev_period,
            );
            self.hilbert_idx = (self.hilbert_idx + 1) % 3;
            q2 = 0.2 * (q1 + ji) + 0.8 * self.prev_q2;
            i2 = 0.2 * (self.i1_for_even_prev3 - jq) + 0.8 * self.prev_i2;
            self.i1_for_odd_prev3 = self.i1_for_odd_prev2;
            self.i1_for_odd_prev2 = detrender;
        } else {
            let detrender = do_hilbert_odd(
                &mut self.detrender_vars,
                smoothed,
                self.hilbert_idx,
                adjusted_prev_period,
            );
            let q1 = do_hilbert_odd(
                &mut self.q1_vars,
                detrender,
                self.hilbert_idx,
                adjusted_prev_period,
            );
            let ji = do_hilbert_odd(
                &mut self.ji_vars,
                self.i1_for_odd_prev3,
                self.hilbert_idx,
                adjusted_prev_period,
            );
            let jq = do_hilbert_odd(
                &mut self.jq_vars,
                q1,
                self.hilbert_idx,
                adjusted_prev_period,
            );
            q2 = 0.2 * (q1 + ji) + 0.8 * self.prev_q2;
            i2 = 0.2 * (self.i1_for_odd_prev3 - jq) + 0.8 * self.prev_i2;
            self.i1_for_even_prev3 = self.i1_for_even_prev2;
            self.i1_for_even_prev2 = detrender;
        }

        self.re = 0.2 * (i2 * self.prev_i2 + q2 * self.prev_q2) + 0.8 * self.re;
        self.im = 0.2 * (i2 * self.prev_q2 - q2 * self.prev_i2) + 0.8 * self.im;
        self.prev_q2 = q2;
        self.prev_i2 = i2;

        let previous_period = self.period;
        if self.im != 0.0 && self.re != 0.0 {
            self.period = 360.0 / ((self.im / self.re).atan() * RAD2DEG);
        }
        self.period = self
            .period
            .min(1.5 * previous_period)
            .max(0.67 * previous_period)
            .clamp(6.0, 50.0);
        self.period = 0.2 * self.period + 0.8 * previous_period;
        self.smooth_period = 0.33 * self.period + 0.67 * self.smooth_period;

        let dc_period = (self.smooth_period + 0.5) as usize;
        let mut average = 0.0;
        // Newest-to-oldest scan, identical accumulation order to the previous
        // `prices.iter().rev().take(dc_period)` fold.
        let mut idx = self.price_head;
        for _ in 0..dc_period.min(self.price_count) {
            idx = if idx == 0 { PRICE_RING - 1 } else { idx - 1 };
            average += self.prices[idx];
        }
        if dc_period > 0 {
            average /= dc_period as f64;
        }
        let trendline =
            (4.0 * average + 3.0 * self.trend1 + 2.0 * self.trend2 + self.trend3) / 10.0;
        self.trend3 = self.trend2;
        self.trend2 = self.trend1;
        self.trend1 = average;

        self.value = (today >= LOOKBACK).then_some(trendline);
        self.value
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Append a chronological slice and emit aligned NaN warm-up values.
    pub fn extend_slice_into(&mut self, input: &[f64], output: &mut Vec<f64>) {
        output.reserve(input.len());
        for value in input {
            output.push(self.append(*value).unwrap_or(f64::NAN));
        }
    }

    /// Restores the post-construction state.
    pub fn reset(&mut self) {
        self.index = 0;
        self.prices = [0.0; PRICE_RING];
        self.price_head = 0;
        self.price_count = 0;
        self.wma_prices = [0.0; 3];
        self.period_wma_sub = 0.0;
        self.period_wma_sum = 0.0;
        self.trailing_wma_value = 0.0;
        self.hilbert_idx = 0;
        self.detrender_vars = HilbertVars::new();
        self.q1_vars = HilbertVars::new();
        self.ji_vars = HilbertVars::new();
        self.jq_vars = HilbertVars::new();
        self.period = 0.0;
        self.smooth_period = 0.0;
        self.prev_i2 = 0.0;
        self.prev_q2 = 0.0;
        self.re = 0.0;
        self.im = 0.0;
        self.i1_for_odd_prev2 = 0.0;
        self.i1_for_odd_prev3 = 0.0;
        self.i1_for_even_prev2 = 0.0;
        self.i1_for_even_prev3 = 0.0;
        self.trend1 = 0.0;
        self.trend2 = 0.0;
        self.trend3 = 0.0;
        self.value = None;
    }
}
