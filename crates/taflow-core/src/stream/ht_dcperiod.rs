//! Incremental Hilbert Transform dominant cycle period (HT_DCPERIOD).

use std::collections::VecDeque;

use crate::cycle::{do_hilbert_even, do_hilbert_odd, HilbertVars};

const RAD2DEG: f64 = 180.0 / std::f64::consts::PI;
const LOOKBACK: usize = 32;

/// Incremental HT_DCPERIOD state.
pub struct HilbertTransformDominantCyclePeriod {
    index: usize,
    prices: VecDeque<f64>,
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
    value: Option<f64>,
}

impl Default for HilbertTransformDominantCyclePeriod {
    fn default() -> Self {
        Self::new()
    }
}

impl HilbertTransformDominantCyclePeriod {
    pub fn new() -> Self {
        Self {
            index: 0,
            prices: VecDeque::with_capacity(4),
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
            value: None,
        }
    }

    fn next_smoothed(&mut self, input: f64) -> Option<f64> {
        if self.index < 3 {
            self.prices.push_back(input);
            if self.index == 2 {
                self.period_wma_sub = self.prices[0];
                self.period_wma_sub += self.prices[1];
                self.period_wma_sub += self.prices[2];
                self.period_wma_sum = self.prices[0];
                self.period_wma_sum += 2.0 * self.prices[1];
                self.period_wma_sum += 3.0 * self.prices[2];
            }
            return None;
        }
        self.period_wma_sub += input;
        self.period_wma_sub -= self.trailing_wma_value;
        self.period_wma_sum += input * 4.0;
        self.trailing_wma_value = self.prices.pop_front().expect("WMA is initialized");
        self.prices.push_back(input);
        let smoothed = self.period_wma_sum * 0.1;
        self.period_wma_sum -= self.period_wma_sub;
        Some(smoothed)
    }

    /// Appends one price and returns a value after the 32-bar warmup.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        let today = self.index;
        let smoothed = self.next_smoothed(input);
        self.index += 1;
        if today < 12 {
            return None;
        }
        let smoothed = smoothed.expect("WMA is initialized after bar two");
        let adjusted = 0.075 * self.period + 0.54;
        let (detrender, q1, i2, q2);
        if today % 2 == 0 {
            detrender = do_hilbert_even(
                &mut self.detrender_vars,
                smoothed,
                self.hilbert_idx,
                adjusted,
            );
            q1 = do_hilbert_even(&mut self.q1_vars, detrender, self.hilbert_idx, adjusted);
            let ji = do_hilbert_even(
                &mut self.ji_vars,
                self.i1_for_even_prev3,
                self.hilbert_idx,
                adjusted,
            );
            let jq = do_hilbert_even(&mut self.jq_vars, q1, self.hilbert_idx, adjusted);
            self.hilbert_idx = (self.hilbert_idx + 1) % 3;
            q2 = 0.2 * (q1 + ji) + 0.8 * self.prev_q2;
            i2 = 0.2 * (self.i1_for_even_prev3 - jq) + 0.8 * self.prev_i2;
            self.i1_for_odd_prev3 = self.i1_for_odd_prev2;
            self.i1_for_odd_prev2 = detrender;
        } else {
            detrender = do_hilbert_odd(
                &mut self.detrender_vars,
                smoothed,
                self.hilbert_idx,
                adjusted,
            );
            q1 = do_hilbert_odd(&mut self.q1_vars, detrender, self.hilbert_idx, adjusted);
            let ji = do_hilbert_odd(
                &mut self.ji_vars,
                self.i1_for_odd_prev3,
                self.hilbert_idx,
                adjusted,
            );
            let jq = do_hilbert_odd(&mut self.jq_vars, q1, self.hilbert_idx, adjusted);
            q2 = 0.2 * (q1 + ji) + 0.8 * self.prev_q2;
            i2 = 0.2 * (self.i1_for_odd_prev3 - jq) + 0.8 * self.prev_i2;
            self.i1_for_even_prev3 = self.i1_for_even_prev2;
            self.i1_for_even_prev2 = detrender;
        }
        self.re = 0.2 * (i2 * self.prev_i2 + q2 * self.prev_q2) + 0.8 * self.re;
        self.im = 0.2 * (i2 * self.prev_q2 - q2 * self.prev_i2) + 0.8 * self.im;
        self.prev_i2 = i2;
        self.prev_q2 = q2;
        let previous = self.period;
        if self.im != 0.0 && self.re != 0.0 {
            self.period = 360.0 / ((self.im / self.re).atan() * RAD2DEG);
        }
        let upper = 1.5 * previous;
        if self.period > upper {
            self.period = upper;
        }
        let lower = 0.67 * previous;
        if self.period < lower {
            self.period = lower;
        }
        if self.period < 6.0 {
            self.period = 6.0;
        } else if self.period > 50.0 {
            self.period = 50.0;
        }
        self.period = 0.2 * self.period + 0.8 * previous;
        self.smooth_period = 0.33 * self.period + 0.67 * self.smooth_period;
        self.value = (today >= LOOKBACK).then_some(self.smooth_period);
        self.value
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let input: Vec<f64> = (0..300)
            .map(|i| 100.0 + (i as f64 * 0.11).sin() * 8.0)
            .collect();
        let expected = crate::cycle::hilbert_transform_dominant_cycle_period(&input).unwrap();
        let mut state = HilbertTransformDominantCyclePeriod::new();
        for (&input, &expected) in input.iter().zip(&expected) {
            match state.append(input) {
                Some(value) => assert!(
                    (value - expected).abs() < 1e-12,
                    "actual={value}, expected={expected}"
                ),
                None => assert!(expected.is_nan()),
            }
        }
    }
}
