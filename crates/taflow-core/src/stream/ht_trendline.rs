//! Stateful Hilbert Transform instantaneous trendline.
//!
//! The state advances TA-Lib's four-bar price smoother, alternating Hilbert
//! transforms, dominant-cycle estimate, cycle-length price average, and final
//! four-value weighted trendline without recomputing prior bars.

use std::collections::VecDeque;

use crate::cycle::{do_hilbert_even, do_hilbert_odd, HilbertVars};

const RAD2DEG: f64 = 180.0 / std::f64::consts::PI;
const LOOKBACK: usize = 63;

/// Incremental HT_TRENDLINE state.
pub struct HtTrendline {
    index: usize,
    prices: VecDeque<f64>,
    wma_prices: VecDeque<f64>,
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

impl Default for HtTrendline {
    fn default() -> Self {
        Self::new()
    }
}

impl HtTrendline {
    /// Creates an empty HT_TRENDLINE state.
    pub fn new() -> Self {
        Self {
            index: 0,
            prices: VecDeque::with_capacity(50),
            wma_prices: VecDeque::with_capacity(4),
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
            self.wma_prices.push_back(input);
            return None;
        }
        if self.index == 2 {
            self.wma_prices.push_back(input);
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
        self.trailing_wma_value = self
            .wma_prices
            .pop_front()
            .expect("initialized WMA has a trailing price");
        self.wma_prices.push_back(input);
        let smoothed = self.period_wma_sum * 0.1;
        self.period_wma_sum -= self.period_wma_sub;
        Some(smoothed)
    }

    /// Appends one price value.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        let today = self.index;
        let smoothed = self.next_smoothed(input);
        self.index += 1;
        if self.prices.len() == 50 {
            self.prices.pop_front();
        }
        self.prices.push_back(input);

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
        for price in self.prices.iter().rev().take(dc_period) {
            average += price;
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

    /// Restores the post-construction state.
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::overlap;

    #[test]
    fn matches_batch_and_reset_replay() {
        let input: Vec<f64> = (0..700)
            .map(|index| 100.0 + (index as f64 * 0.17).sin() * 8.0 + index as f64 * 0.01)
            .collect();
        let expected = overlap::hilbert_transform_trendline(&input).unwrap();
        let mut state = HtTrendline::new();
        for (&input, &expected) in input.iter().zip(&expected) {
            match state.append(input) {
                Some(actual) => assert!((actual - expected).abs() < 1e-12),
                None => assert!(expected.is_nan()),
            }
        }
        let final_value = state.value();
        state.reset();
        for input in input {
            state.append(input);
        }
        assert_eq!(state.value(), final_value);
    }
}
