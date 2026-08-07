//! Stateful MESA Adaptive Moving Average.
//!
//! MAMA uses TA-Lib's four-bar price smoother, alternating Hilbert transforms,
//! dominant-cycle estimate, and phase-controlled adaptive smoothing.  The
//! paired FAMA output is advanced from the same per-bar state.

use std::collections::VecDeque;

use crate::cycle::{do_hilbert_even, do_hilbert_odd, HilbertVars};
use crate::error::{TaError, TaResult};

use super::StreamingIndicator;

const RAD2DEG: f64 = 180.0 / std::f64::consts::PI;
const LOOKBACK: usize = 32;

/// One aligned MAMA/FAMA observation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MesaAdaptiveMovingAverageValue {
    pub mama: f64,
    pub fama: f64,
}

/// Incremental MAMA with the same warm-up and recurrence as TA-Lib.
pub struct MesaAdaptiveMovingAverage {
    fast_limit: f64,
    slow_limit: f64,
    index: usize,
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
    prev_i2: f64,
    prev_q2: f64,
    re: f64,
    im: f64,
    i1_for_odd_prev2: f64,
    i1_for_odd_prev3: f64,
    i1_for_even_prev2: f64,
    i1_for_even_prev3: f64,
    prev_phase: f64,
    prev_mama: f64,
    prev_fama: f64,
    value: Option<MesaAdaptiveMovingAverageValue>,
}

impl MesaAdaptiveMovingAverage {
    /// Creates a MAMA state using limits in TA-Lib's accepted ranges.
    pub fn new(fast_limit: f64, slow_limit: f64) -> TaResult<Self> {
        if fast_limit <= 0.0 || fast_limit > 1.0 {
            return Err(TaError::InvalidParameter {
                name: "fastlimit",
                value: fast_limit.to_string(),
                reason: "must be in (0, 1]",
            });
        }
        if slow_limit <= 0.0 || slow_limit >= fast_limit {
            return Err(TaError::InvalidParameter {
                name: "slowlimit",
                value: slow_limit.to_string(),
                reason: "must be in (0, fastlimit)",
            });
        }
        Ok(Self {
            fast_limit,
            slow_limit,
            index: 0,
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
            prev_i2: 0.0,
            prev_q2: 0.0,
            re: 0.0,
            im: 0.0,
            i1_for_odd_prev2: 0.0,
            i1_for_odd_prev3: 0.0,
            i1_for_even_prev2: 0.0,
            i1_for_even_prev3: 0.0,
            prev_phase: 0.0,
            prev_mama: 0.0,
            prev_fama: 0.0,
            value: None,
        })
    }

    fn next_smoothed(&mut self, input: f64) -> Option<f64> {
        if self.index < 2 {
            self.wma_prices.push_back(input);
            return None;
        }
        if self.index == 2 {
            self.wma_prices.push_back(input);
            let p0 = self.wma_prices[0];
            let p1 = self.wma_prices[1];
            let p2 = self.wma_prices[2];
            self.period_wma_sub = p0;
            self.period_wma_sub += p1;
            self.period_wma_sub += p2;
            self.period_wma_sum = p0;
            self.period_wma_sum += p1 * 2.0;
            self.period_wma_sum += p2 * 3.0;
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
}

impl StreamingIndicator for MesaAdaptiveMovingAverage {
    type Output = MesaAdaptiveMovingAverageValue;

    fn append(&mut self, input: f64) -> Option<MesaAdaptiveMovingAverageValue> {
        let today = self.index;
        let smoothed = self.next_smoothed(input);
        self.index += 1;

        // TA-Lib computes and discards nine WMA values at bars 3 through 11.
        if today < 12 {
            return None;
        }
        let smoothed = smoothed.expect("the WMA is initialized after bar 2");
        let adjusted_prev_period = 0.075 * self.period + 0.54;

        let (q1, i1, i2, q2);
        if today % 2 == 0 {
            let detrender = do_hilbert_even(
                &mut self.detrender_vars,
                smoothed,
                self.hilbert_idx,
                adjusted_prev_period,
            );
            q1 = do_hilbert_even(
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
            i1 = self.i1_for_even_prev3;
            self.i1_for_odd_prev3 = self.i1_for_odd_prev2;
            self.i1_for_odd_prev2 = detrender;
        } else {
            let detrender = do_hilbert_odd(
                &mut self.detrender_vars,
                smoothed,
                self.hilbert_idx,
                adjusted_prev_period,
            );
            q1 = do_hilbert_odd(
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
            i1 = self.i1_for_odd_prev3;
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

        let phase = if i1 != 0.0 {
            (q1 / i1).atan() * RAD2DEG
        } else {
            0.0
        };
        let delta_phase = (self.prev_phase - phase).max(1.0);
        let alpha = (self.fast_limit / delta_phase)
            .max(self.slow_limit)
            .min(self.fast_limit);
        self.prev_mama = alpha * input + (1.0 - alpha) * self.prev_mama;
        self.prev_fama = 0.5 * alpha * self.prev_mama + (1.0 - 0.5 * alpha) * self.prev_fama;
        self.prev_phase = phase;

        self.value = (today >= LOOKBACK).then_some(MesaAdaptiveMovingAverageValue {
            mama: self.prev_mama,
            fama: self.prev_fama,
        });
        self.value
    }

    fn value(&self) -> Option<MesaAdaptiveMovingAverageValue> {
        self.value
    }

    fn reset(&mut self) {
        let fast_limit = self.fast_limit;
        let slow_limit = self.slow_limit;
        *self = Self::new(fast_limit, slow_limit).expect("validated MAMA limits remain valid");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::overlap;

    #[test]
    fn matches_batch_and_reset_replay() {
        let input: Vec<f64> = (0..200)
            .map(|index| 100.0 + (index as f64 * 0.23).sin() * 9.0 + index as f64 * 0.04)
            .collect();
        let (expected_mama, expected_fama) = overlap::mesa_adaptive_moving_average(&input, 0.5, 0.05).unwrap();
        let mut state = MesaAdaptiveMovingAverage::new(0.5, 0.05).unwrap();
        for ((&input, expected_mama), expected_fama) in input
            .iter()
            .zip(expected_mama.iter())
            .zip(expected_fama.iter())
        {
            match state.append(input) {
                Some(actual) => {
                    assert!((actual.mama - expected_mama).abs() < 1e-12);
                    assert!((actual.fama - expected_fama).abs() < 1e-12);
                }
                None => {
                    assert!(expected_mama.is_nan());
                    assert!(expected_fama.is_nan());
                }
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
