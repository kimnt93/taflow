//! Stateful Hilbert Transform instantaneous trendline.
//!
//! The state advances TA-Lib's four-bar price smoother, alternating Hilbert
//! transforms, dominant-cycle estimate, cycle-length price average, and final
//! four-value weighted trendline without recomputing prior bars.

use std::collections::VecDeque;

use crate::error::{TaError, TaResult};
use crate::stream::cycle::{do_hilbert_even, do_hilbert_odd, HilbertVars};

const RAD2DEG: f64 = 180.0 / std::f64::consts::PI;
const LOOKBACK: usize = 63;

/// Incremental HT_TRENDLINE state.
/// Persistent Rust state or aligned output type for `HilbertTransformTrendline`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct HilbertTransformTrendline {
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

    #[test]
    fn matches_batch_and_reset_replay() {
        let input: Vec<f64> = (0..700)
            .map(|index| 100.0 + (index as f64 * 0.17).sin() * 8.0 + index as f64 * 0.01)
            .collect();
        let expected = crate::stream::hilbert_transform_trendline(&input).unwrap();
        let mut state = HilbertTransformTrendline::new();
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
// Faithful port of C TA-Lib HT_TRENDLINE.
// Uses the same even/odd alternating Hilbert Transform as cycle/mod.rs.
// The trendline = WMA(4) of SMA(price, dcPeriod), where dcPeriod is the
// instantaneous dominant cycle period from the Hilbert Transform.
// lookback = 63

const BATCH_A: f64 = 0.0962;
const BATCH_B: f64 = 0.5769;

struct BatchHilbertVars {
    odd: [f64; 3],
    even: [f64; 3],
    prev_odd: f64,
    prev_even: f64,
    prev_input_odd: f64,
    prev_input_even: f64,
}

impl BatchHilbertVars {
    fn new() -> Self {
        Self {
            odd: [0.0; 3],
            even: [0.0; 3],
            prev_odd: 0.0,
            prev_even: 0.0,
            prev_input_odd: 0.0,
            prev_input_even: 0.0,
        }
    }
}

#[inline(always)]
fn batch_do_hilbert_even(
    vars: &mut BatchHilbertVars,
    input: f64,
    hilbert_idx: usize,
    adjusted_prev_period: f64,
) -> f64 {
    let hilbert_temp_real = BATCH_A * input;
    let mut result = -vars.even[hilbert_idx];
    vars.even[hilbert_idx] = hilbert_temp_real;
    result += hilbert_temp_real;
    result -= vars.prev_even;
    vars.prev_even = BATCH_B * vars.prev_input_even;
    result += vars.prev_even;
    vars.prev_input_even = input;
    result *= adjusted_prev_period;
    result
}

#[inline(always)]
fn batch_do_hilbert_odd(
    vars: &mut BatchHilbertVars,
    input: f64,
    hilbert_idx: usize,
    adjusted_prev_period: f64,
) -> f64 {
    let hilbert_temp_real = BATCH_A * input;
    let mut result = -vars.odd[hilbert_idx];
    vars.odd[hilbert_idx] = hilbert_temp_real;
    result += hilbert_temp_real;
    result -= vars.prev_odd;
    vars.prev_odd = BATCH_B * vars.prev_input_odd;
    result += vars.prev_odd;
    vars.prev_input_odd = input;
    result *= adjusted_prev_period;
    result
}

struct BatchWmaState {
    period_wma_sub: f64,
    period_wma_sum: f64,
    trailing_wma_value: f64,
    trailing_wma_idx: usize,
}

impl BatchWmaState {
    fn init(input: &[f64], start: usize) -> (Self, usize) {
        let p0 = input[start];
        let p1 = input[start + 1];
        let p2 = input[start + 2];

        let period_wma_sub = p0 + p1 + p2;
        let period_wma_sum = p0 + p1 * 2.0 + p2 * 3.0;

        let state = BatchWmaState {
            period_wma_sub,
            period_wma_sum,
            trailing_wma_value: 0.0,
            trailing_wma_idx: start,
        };
        (state, start + 3)
    }

    #[inline(always)]
    fn next(&mut self, input: &[f64], new_price: f64) -> f64 {
        self.period_wma_sub += new_price;
        self.period_wma_sub -= self.trailing_wma_value;
        self.period_wma_sum += new_price * 4.0;
        self.trailing_wma_value = input[self.trailing_wma_idx];
        self.trailing_wma_idx += 1;
        let smoothed = self.period_wma_sum * 0.1;
        self.period_wma_sum -= self.period_wma_sub;
        smoothed
    }
}

/// Hilbert Transform - Instantaneous Trendline
///
/// Faithful port of C TA-Lib ta_HT_TRENDLINE.c.
/// Compute the hilbert transform trendline result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn hilbert_transform_trendline(input: &[f64]) -> TaResult<Vec<f64>> {
    let len = input.len();
    let lookback: usize = 63;

    if len <= lookback {
        return Err(TaError::InsufficientData {
            need: lookback + 1,
            got: len,
        });
    }

    let start_idx = lookback;

    let trailing_wma_start = start_idx - lookback; // = 0
    let (mut wma, mut today) = BatchWmaState::init(input, trailing_wma_start);

    // Warm up WMA: 34 iterations (matching C TA-Lib for lookback 63 functions)
    for _ in 0..34 {
        let val = input[today];
        today += 1;
        let _ = wma.next(input, val);
    }

    let mut hilbert_idx: usize = 0;
    let mut detrender_vars = BatchHilbertVars::new();
    let mut q1_vars = BatchHilbertVars::new();
    let mut ji_vars = BatchHilbertVars::new();
    let mut jq_vars = BatchHilbertVars::new();

    let mut period: f64 = 0.0;
    let mut smooth_period: f64 = 0.0;

    let mut prev_i2: f64 = 0.0;
    let mut prev_q2: f64 = 0.0;
    let mut re: f64 = 0.0;
    let mut im: f64 = 0.0;

    let mut i1_for_odd_prev2: f64 = 0.0;
    let mut i1_for_odd_prev3: f64 = 0.0;
    let mut i1_for_even_prev2: f64 = 0.0;
    let mut i1_for_even_prev3: f64 = 0.0;

    // Trendline WMA(4) smoothing variables
    let mut i_trend1: f64 = 0.0;
    let mut i_trend2: f64 = 0.0;
    let mut i_trend3: f64 = 0.0;

    let mut output = vec![0.0_f64; len];
    output[..lookback].fill(f64::NAN);

    while today < len {
        let adjusted_prev_period = 0.075 * period + 0.54;

        let today_value = input[today];
        let smoothed_value = wma.next(input, today_value);

        let (detrender, q1, i2, q2);

        if today % 2 == 0 {
            detrender = batch_do_hilbert_even(
                &mut detrender_vars,
                smoothed_value,
                hilbert_idx,
                adjusted_prev_period,
            );
            q1 = batch_do_hilbert_even(&mut q1_vars, detrender, hilbert_idx, adjusted_prev_period);
            let _ji = batch_do_hilbert_even(
                &mut ji_vars,
                i1_for_even_prev3,
                hilbert_idx,
                adjusted_prev_period,
            );
            let _jq = batch_do_hilbert_even(&mut jq_vars, q1, hilbert_idx, adjusted_prev_period);
            hilbert_idx += 1;
            if hilbert_idx == 3 {
                hilbert_idx = 0;
            }

            q2 = 0.2 * (q1 + _ji) + 0.8 * prev_q2;
            i2 = 0.2 * (i1_for_even_prev3 - _jq) + 0.8 * prev_i2;

            i1_for_odd_prev3 = i1_for_odd_prev2;
            i1_for_odd_prev2 = detrender;
        } else {
            detrender = batch_do_hilbert_odd(
                &mut detrender_vars,
                smoothed_value,
                hilbert_idx,
                adjusted_prev_period,
            );
            q1 = batch_do_hilbert_odd(&mut q1_vars, detrender, hilbert_idx, adjusted_prev_period);
            let _ji = batch_do_hilbert_odd(
                &mut ji_vars,
                i1_for_odd_prev3,
                hilbert_idx,
                adjusted_prev_period,
            );
            let _jq = batch_do_hilbert_odd(&mut jq_vars, q1, hilbert_idx, adjusted_prev_period);

            q2 = 0.2 * (q1 + _ji) + 0.8 * prev_q2;
            i2 = 0.2 * (i1_for_odd_prev3 - _jq) + 0.8 * prev_i2;

            i1_for_even_prev3 = i1_for_even_prev2;
            i1_for_even_prev2 = detrender;
        }

        // Adjust period
        re = 0.2 * (i2 * prev_i2 + q2 * prev_q2) + 0.8 * re;
        im = 0.2 * (i2 * prev_q2 - q2 * prev_i2) + 0.8 * im;
        prev_q2 = q2;
        prev_i2 = i2;

        let temp_real = period;
        if im != 0.0 && re != 0.0 {
            period = 360.0 / ((im / re).atan() * (180.0 / std::f64::consts::PI));
        }
        let temp_real2 = 1.5 * temp_real;
        if period > temp_real2 {
            period = temp_real2;
        }
        let temp_real2 = 0.67 * temp_real;
        if period < temp_real2 {
            period = temp_real2;
        }
        if period < 6.0 {
            period = 6.0;
        } else if period > 50.0 {
            period = 50.0;
        }
        period = 0.2 * period + 0.8 * temp_real;

        smooth_period = 0.33 * period + 0.67 * smooth_period;

        // Compute trendline: SMA of input prices over dcPeriod, then WMA(4) smooth
        let dc_period = smooth_period + 0.5;
        let dc_period_int = dc_period as i32;

        let mut temp = 0.0_f64;
        let mut price_idx = today;
        for _ in 0..dc_period_int {
            temp += input[price_idx];
            if price_idx == 0 {
                break;
            }
            price_idx -= 1;
        }

        if dc_period_int > 0 {
            temp /= dc_period_int as f64;
        }

        // WMA(4) smoothing of the SMA result
        let trendline = (4.0 * temp + 3.0 * i_trend1 + 2.0 * i_trend2 + i_trend3) / 10.0;
        i_trend3 = i_trend2;
        i_trend2 = i_trend1;
        i_trend1 = temp;

        if today >= start_idx {
            output[today] = trendline;
        }

        today += 1;
    }

    Ok(output)
}
