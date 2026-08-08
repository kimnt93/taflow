//! Stateful MESA Adaptive Moving Average.
//!
//! MAMA uses TA-Lib's four-bar price smoother, alternating Hilbert transforms,
//! dominant-cycle estimate, and phase-controlled adaptive smoothing.  The
//! paired FAMA output is advanced from the same per-bar state.

use crate::error::{TaError, TaResult};
use crate::stream::cycle::{do_hilbert_even, do_hilbert_odd, HilbertVars, WmaState};

use super::StreamingIndicator;

const RAD2DEG: f64 = 180.0 / std::f64::consts::PI;
const LOOKBACK: usize = 32;

/// One aligned MAMA/FAMA observation.
#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `MesaAdaptiveMovingAverageValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct MesaAdaptiveMovingAverageValue {
    pub mama: f64,
    pub fama: f64,
}

/// Incremental MAMA with the same warm-up and recurrence as TA-Lib.
/// Persistent Rust state or aligned output type for `MesaAdaptiveMovingAverage`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct MesaAdaptiveMovingAverage {
    fast_limit: f64,
    slow_limit: f64,
    index: usize,
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
            self.wma_prices[self.index] = input;
            return None;
        }
        if self.index == 2 {
            self.wma_prices[2] = input;
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
        let slot = self.index % 3;
        self.trailing_wma_value = self.wma_prices[slot];
        self.wma_prices[slot] = input;
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
        self.index = 0;
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
        self.prev_i2 = 0.0;
        self.prev_q2 = 0.0;
        self.re = 0.0;
        self.im = 0.0;
        self.i1_for_odd_prev2 = 0.0;
        self.i1_for_odd_prev3 = 0.0;
        self.i1_for_even_prev2 = 0.0;
        self.i1_for_even_prev3 = 0.0;
        self.prev_phase = 0.0;
        self.prev_mama = 0.0;
        self.prev_fama = 0.0;
        self.value = None;
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

    #[test]
    fn bitwise_matches_batch_and_reset_replay_on_lcg_bars() {
        let input = lcg_series(5_000, 0x3a3a_1111_2222_3333);
        let (expected_mama, expected_fama) =
            crate::stream::mesa_adaptive_moving_average(&input, 0.5, 0.05).unwrap();
        let mut state = MesaAdaptiveMovingAverage::new(0.5, 0.05).unwrap();
        let mut outputs = Vec::with_capacity(input.len());
        for (index, ((&bar, &mama), &fama)) in input
            .iter()
            .zip(&expected_mama)
            .zip(&expected_fama)
            .enumerate()
        {
            let actual = state.append(bar);
            match actual {
                Some(value) => {
                    assert_eq!(value.mama.to_bits(), mama.to_bits(), "mama @{index}");
                    assert_eq!(value.fama.to_bits(), fama.to_bits(), "fama @{index}");
                }
                None => {
                    assert!(mama.is_nan());
                    assert!(fama.is_nan());
                }
            }
            outputs.push(actual);
        }
        state.reset();
        assert!(state.value().is_none());
        for (&bar, expected) in input.iter().zip(&outputs) {
            let replay = state.append(bar);
            match (replay, expected) {
                (Some(got), Some(want)) => {
                    assert_eq!(got.mama.to_bits(), want.mama.to_bits());
                    assert_eq!(got.fama.to_bits(), want.fama.to_bits());
                }
                (None, None) => {}
                _ => panic!("warm-up boundary moved after reset"),
            }
        }
    }

    #[test]
    fn matches_batch_and_reset_replay() {
        let input: Vec<f64> = (0..200)
            .map(|index| 100.0 + (index as f64 * 0.23).sin() * 9.0 + index as f64 * 0.04)
            .collect();
        let (expected_mama, expected_fama) =
            crate::stream::mesa_adaptive_moving_average(&input, 0.5, 0.05).unwrap();
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
/// MESA Adaptive Moving Average (MAMA)
///
/// Faithful port of C TA-Lib TA_MAMA using the same even/odd alternating
/// Hilbert Transform approach as the cycle indicators (HT_DCPERIOD etc.).
///
/// Returns (mama, fama).
/// fastlimit default 0.5, slowlimit default 0.05.
/// Compute the mesa adaptive moving average result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `fastlimit` - Input series or configuration value.
/// * `slowlimit` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn mesa_adaptive_moving_average(
    input: &[f64],
    fastlimit: f64,
    slowlimit: f64,
) -> TaResult<(Vec<f64>, Vec<f64>)> {
    let len = input.len();
    let lookback = 32;

    if len <= lookback {
        return Err(TaError::InsufficientData {
            need: lookback + 1,
            got: len,
        });
    }
    if fastlimit <= 0.0 || fastlimit > 1.0 {
        return Err(TaError::InvalidParameter {
            name: "fastlimit",
            value: fastlimit.to_string(),
            reason: "must be in (0, 1]",
        });
    }
    if slowlimit <= 0.0 || slowlimit >= fastlimit {
        return Err(TaError::InvalidParameter {
            name: "slowlimit",
            value: slowlimit.to_string(),
            reason: "must be in (0, fastlimit)",
        });
    }

    let start_idx = lookback;

    let mut mama_out = vec![0.0_f64; len];
    mama_out[..lookback].fill(f64::NAN);
    let mut fama_out = vec![0.0_f64; len];
    fama_out[..lookback].fill(f64::NAN);

    // --- WMA initialization (identical to HT_DCPERIOD) ---
    let trailing_wma_start = start_idx - lookback; // = 0
    let (mut wma, mut today) = WmaState::init(input, trailing_wma_start);

    // Warm up WMA: 9 iterations (matching C TA-Lib)
    for _ in 0..9 {
        let val = input[today];
        today += 1;
        let _ = wma.next(input, val);
    }

    // --- Hilbert Transform state ---
    let mut hilbert_idx: usize = 0;
    let mut detrender_vars = HilbertVars::new();
    let mut q1_vars = HilbertVars::new();
    let mut ji_vars = HilbertVars::new();
    let mut jq_vars = HilbertVars::new();

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

    // --- MAMA-specific state ---
    let mut prev_phase: f64 = 0.0;
    let mut prev_mama = 0.0_f64;
    let mut prev_fama = 0.0_f64;

    let mut out_idx = start_idx;

    while today < len {
        let adjusted_prev_period = 0.075 * period + 0.54;

        let today_value = input[today];
        let smoothed_value = wma.next(input, today_value);

        let (detrender, q1, i1, i2, q2);

        if today % 2 == 0 {
            // Even bar
            detrender = do_hilbert_even(
                &mut detrender_vars,
                smoothed_value,
                hilbert_idx,
                adjusted_prev_period,
            );
            q1 = do_hilbert_even(&mut q1_vars, detrender, hilbert_idx, adjusted_prev_period);
            let _ji = do_hilbert_even(
                &mut ji_vars,
                i1_for_even_prev3,
                hilbert_idx,
                adjusted_prev_period,
            );
            let _jq = do_hilbert_even(&mut jq_vars, q1, hilbert_idx, adjusted_prev_period);
            hilbert_idx += 1;
            if hilbert_idx == 3 {
                hilbert_idx = 0;
            }

            q2 = 0.2 * (q1 + _ji) + 0.8 * prev_q2;
            i2 = 0.2 * (i1_for_even_prev3 - _jq) + 0.8 * prev_i2;

            // I1 for MAMA phase = delayed detrender (i1_for_even_prev3)
            i1 = i1_for_even_prev3;

            i1_for_odd_prev3 = i1_for_odd_prev2;
            i1_for_odd_prev2 = detrender;
        } else {
            // Odd bar
            detrender = do_hilbert_odd(
                &mut detrender_vars,
                smoothed_value,
                hilbert_idx,
                adjusted_prev_period,
            );
            q1 = do_hilbert_odd(&mut q1_vars, detrender, hilbert_idx, adjusted_prev_period);
            let _ji = do_hilbert_odd(
                &mut ji_vars,
                i1_for_odd_prev3,
                hilbert_idx,
                adjusted_prev_period,
            );
            let _jq = do_hilbert_odd(&mut jq_vars, q1, hilbert_idx, adjusted_prev_period);

            q2 = 0.2 * (q1 + _ji) + 0.8 * prev_q2;
            i2 = 0.2 * (i1_for_odd_prev3 - _jq) + 0.8 * prev_i2;

            // I1 for MAMA phase = delayed detrender (i1_for_odd_prev3)
            i1 = i1_for_odd_prev3;

            i1_for_even_prev3 = i1_for_even_prev2;
            i1_for_even_prev2 = detrender;
        }

        // --- Period estimation (identical to HT_DCPERIOD) ---
        re = 0.2 * (i2 * prev_i2 + q2 * prev_q2) + 0.8 * re;
        im = 0.2 * (i2 * prev_q2 - q2 * prev_i2) + 0.8 * im;
        prev_q2 = q2;
        prev_i2 = i2;

        let temp_real = period;
        if im != 0.0 && re != 0.0 {
            period = 360.0 / ((im / re).atan() * RAD2DEG);
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

        // --- MAMA-specific: phase and adaptive alpha ---
        let phase = if i1 != 0.0 {
            (q1 / i1).atan() * RAD2DEG
        } else {
            0.0
        };

        let mut delta_phase = prev_phase - phase;
        if delta_phase < 1.0 {
            delta_phase = 1.0;
        }

        let mut alpha = fastlimit / delta_phase;
        if alpha < slowlimit {
            alpha = slowlimit;
        }
        if alpha > fastlimit {
            alpha = fastlimit;
        }

        prev_mama = alpha * today_value + (1.0 - alpha) * prev_mama;
        prev_fama = 0.5 * alpha * prev_mama + (1.0 - 0.5 * alpha) * prev_fama;

        if today >= start_idx {
            mama_out[out_idx] = prev_mama;
            fama_out[out_idx] = prev_fama;
            out_idx += 1;
        }

        prev_phase = phase;
        today += 1;
    }

    Ok((mama_out, fama_out))
}
