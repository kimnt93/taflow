//! Incremental Hilbert Transform dominant cycle phase (HT_DCPHASE).

use crate::stream::cycle::{dc_sin_cos, do_hilbert_even, do_hilbert_odd, HilbertVars};

const RAD2DEG: f64 = 180.0 / std::f64::consts::PI;
const LOOKBACK: usize = 63;

/// Incremental HT_DCPHASE state.
/// Persistent Rust state or aligned output type for `HilbertTransformDominantCyclePhase`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct HilbertTransformDominantCyclePhase {
    index: usize,
    /// Three-slot delay line feeding the running WMA(4) smoother.
    prices: [f64; 3],
    wma_sub: f64,
    wma_sum: f64,
    trailing: f64,
    hilbert_idx: usize,
    detrender: HilbertVars,
    q1: HilbertVars,
    ji: HilbertVars,
    jq: HilbertVars,
    period: f64,
    smooth_period: f64,
    prev_i2: f64,
    prev_q2: f64,
    re: f64,
    im: f64,
    odd2: f64,
    odd3: f64,
    even2: f64,
    even3: f64,
    smooth_prices: [f64; 50],
    smooth_idx: usize,
    phase: f64,
    value: Option<f64>,
}

impl Default for HilbertTransformDominantCyclePhase {
    fn default() -> Self {
        Self::new()
    }
}
impl HilbertTransformDominantCyclePhase {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            index: 0,
            prices: [0.0; 3],
            wma_sub: 0.0,
            wma_sum: 0.0,
            trailing: 0.0,
            hilbert_idx: 0,
            detrender: HilbertVars::new(),
            q1: HilbertVars::new(),
            ji: HilbertVars::new(),
            jq: HilbertVars::new(),
            period: 0.0,
            smooth_period: 0.0,
            prev_i2: 0.0,
            prev_q2: 0.0,
            re: 0.0,
            im: 0.0,
            odd2: 0.0,
            odd3: 0.0,
            even2: 0.0,
            even3: 0.0,
            smooth_prices: [0.0; 50],
            smooth_idx: 0,
            phase: 0.0,
            value: None,
        }
    }
    fn smooth(&mut self, input: f64) -> Option<f64> {
        if self.index < 3 {
            self.prices[self.index] = input;
            if self.index == 2 {
                self.wma_sub = self.prices[0];
                self.wma_sub += self.prices[1];
                self.wma_sub += self.prices[2];
                self.wma_sum = self.prices[0];
                self.wma_sum += self.prices[1] * 2.0;
                self.wma_sum += self.prices[2] * 3.0;
            }
            return None;
        }
        self.wma_sub += input;
        self.wma_sub -= self.trailing;
        self.wma_sum += input * 4.0;
        let slot = self.index % 3;
        self.trailing = self.prices[slot];
        self.prices[slot] = input;
        let value = self.wma_sum * 0.1;
        self.wma_sum -= self.wma_sub;
        Some(value)
    }
    /// Appends one price and returns phase after TA-Lib's 63-bar warmup.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        let today = self.index;
        let smoothed = self.smooth(input);
        self.index += 1;
        if today < 37 {
            return None;
        }
        let smoothed = smoothed.expect("WMA is initialized");
        self.smooth_prices[self.smooth_idx] = smoothed;
        let adjusted = 0.075 * self.period + 0.54;
        let (detrender, q1, i2, q2);
        if today % 2 == 0 {
            detrender = do_hilbert_even(&mut self.detrender, smoothed, self.hilbert_idx, adjusted);
            q1 = do_hilbert_even(&mut self.q1, detrender, self.hilbert_idx, adjusted);
            let ji = do_hilbert_even(&mut self.ji, self.even3, self.hilbert_idx, adjusted);
            let jq = do_hilbert_even(&mut self.jq, q1, self.hilbert_idx, adjusted);
            self.hilbert_idx = (self.hilbert_idx + 1) % 3;
            q2 = 0.2 * (q1 + ji) + 0.8 * self.prev_q2;
            i2 = 0.2 * (self.even3 - jq) + 0.8 * self.prev_i2;
            self.odd3 = self.odd2;
            self.odd2 = detrender;
        } else {
            detrender = do_hilbert_odd(&mut self.detrender, smoothed, self.hilbert_idx, adjusted);
            q1 = do_hilbert_odd(&mut self.q1, detrender, self.hilbert_idx, adjusted);
            let ji = do_hilbert_odd(&mut self.ji, self.odd3, self.hilbert_idx, adjusted);
            let jq = do_hilbert_odd(&mut self.jq, q1, self.hilbert_idx, adjusted);
            q2 = 0.2 * (q1 + ji) + 0.8 * self.prev_q2;
            i2 = 0.2 * (self.odd3 - jq) + 0.8 * self.prev_i2;
            self.even3 = self.even2;
            self.even2 = detrender;
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
        let count = ((self.smooth_period + 0.5) as i32).max(0) as usize;
        let mut real = 0.0;
        let mut imag = 0.0;
        // Walk the ring newest-to-oldest as two contiguous descending runs so
        // the loop is branch-free; the accumulation order is unchanged.
        let table = dc_sin_cos(count);
        let head = self.smooth_idx;
        let first = (head + 1).min(count);
        let (front, wrapped) = table.split_at(first);
        for (&(sin_angle, cos_angle), &price) in front
            .iter()
            .zip(self.smooth_prices[head + 1 - first..=head].iter().rev())
        {
            real += sin_angle * price;
            imag += cos_angle * price;
        }
        if !wrapped.is_empty() {
            let rest = wrapped.len();
            for (&(sin_angle, cos_angle), &price) in wrapped
                .iter()
                .zip(self.smooth_prices[50 - rest..].iter().rev())
            {
                real += sin_angle * price;
                imag += cos_angle * price;
            }
        }
        let abs_imag = imag.abs();
        if abs_imag > 0.0 {
            self.phase = (real / imag).atan() * RAD2DEG;
        } else if abs_imag <= 0.01 {
            if real < 0.0 {
                self.phase -= 90.0;
            } else if real > 0.0 {
                self.phase += 90.0;
            }
        }
        self.phase += 90.0;
        self.phase += 360.0 / self.smooth_period;
        if imag < 0.0 {
            self.phase += 180.0;
        }
        if self.phase > 315.0 {
            self.phase -= 360.0;
        }
        self.value = (today >= LOOKBACK).then_some(self.phase);
        self.smooth_idx = (self.smooth_idx + 1) % 50;
        self.value
    }
    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub(crate) fn current_phase(&self) -> f64 {
        self.phase
    }
    pub(crate) fn current_smooth_period(&self) -> f64 {
        self.smooth_period
    }
    pub(crate) fn current_smooth_price(&self) -> f64 {
        self.smooth_prices[(self.smooth_idx + 49) % 50]
    }
    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.index = 0;
        self.prices = [0.0; 3];
        self.wma_sub = 0.0;
        self.wma_sum = 0.0;
        self.trailing = 0.0;
        self.hilbert_idx = 0;
        self.detrender = HilbertVars::new();
        self.q1 = HilbertVars::new();
        self.ji = HilbertVars::new();
        self.jq = HilbertVars::new();
        self.period = 0.0;
        self.smooth_period = 0.0;
        self.prev_i2 = 0.0;
        self.prev_q2 = 0.0;
        self.re = 0.0;
        self.im = 0.0;
        self.odd2 = 0.0;
        self.odd3 = 0.0;
        self.even2 = 0.0;
        self.even3 = 0.0;
        self.smooth_prices = [0.0; 50];
        self.smooth_idx = 0;
        self.phase = 0.0;
        self.value = None;
    }
}

#[cfg(test)]
pub(crate) mod oracle {
    //! Verbatim copy of the pre-optimization HT_DCPHASE streaming state used
    //! as a bit-exact in-test oracle.

    use crate::stream::cycle::{do_hilbert_even, do_hilbert_odd, HilbertVars};
    use std::collections::VecDeque;

    const RAD2DEG: f64 = 180.0 / std::f64::consts::PI;
    const FULL_CIRCLE: f64 = 2.0 * std::f64::consts::PI;
    const LOOKBACK: usize = 63;

    pub(crate) struct OracleDcPhase {
        index: usize,
        prices: VecDeque<f64>,
        wma_sub: f64,
        wma_sum: f64,
        trailing: f64,
        hilbert_idx: usize,
        detrender: HilbertVars,
        q1: HilbertVars,
        ji: HilbertVars,
        jq: HilbertVars,
        period: f64,
        smooth_period: f64,
        prev_i2: f64,
        prev_q2: f64,
        re: f64,
        im: f64,
        odd2: f64,
        odd3: f64,
        even2: f64,
        even3: f64,
        smooth_prices: [f64; 50],
        smooth_idx: usize,
        phase: f64,
        value: Option<f64>,
    }

    impl OracleDcPhase {
        pub(crate) fn new() -> Self {
            Self {
                index: 0,
                prices: VecDeque::with_capacity(4),
                wma_sub: 0.0,
                wma_sum: 0.0,
                trailing: 0.0,
                hilbert_idx: 0,
                detrender: HilbertVars::new(),
                q1: HilbertVars::new(),
                ji: HilbertVars::new(),
                jq: HilbertVars::new(),
                period: 0.0,
                smooth_period: 0.0,
                prev_i2: 0.0,
                prev_q2: 0.0,
                re: 0.0,
                im: 0.0,
                odd2: 0.0,
                odd3: 0.0,
                even2: 0.0,
                even3: 0.0,
                smooth_prices: [0.0; 50],
                smooth_idx: 0,
                phase: 0.0,
                value: None,
            }
        }
        fn smooth(&mut self, input: f64) -> Option<f64> {
            if self.index < 3 {
                self.prices.push_back(input);
                if self.index == 2 {
                    self.wma_sub = self.prices[0];
                    self.wma_sub += self.prices[1];
                    self.wma_sub += self.prices[2];
                    self.wma_sum = self.prices[0];
                    self.wma_sum += self.prices[1] * 2.0;
                    self.wma_sum += self.prices[2] * 3.0;
                }
                return None;
            }
            self.wma_sub += input;
            self.wma_sub -= self.trailing;
            self.wma_sum += input * 4.0;
            self.trailing = self.prices.pop_front().expect("WMA is initialized");
            self.prices.push_back(input);
            let value = self.wma_sum * 0.1;
            self.wma_sum -= self.wma_sub;
            Some(value)
        }
        pub(crate) fn current_phase(&self) -> f64 {
            self.phase
        }
        pub(crate) fn current_smooth_period(&self) -> f64 {
            self.smooth_period
        }
        pub(crate) fn current_smooth_price(&self) -> f64 {
            self.smooth_prices[(self.smooth_idx + 49) % 50]
        }
        pub(crate) fn append(&mut self, input: f64) -> Option<f64> {
            let today = self.index;
            let smoothed = self.smooth(input);
            self.index += 1;
            if today < 37 {
                return None;
            }
            let smoothed = smoothed.expect("WMA is initialized");
            self.smooth_prices[self.smooth_idx] = smoothed;
            let adjusted = 0.075 * self.period + 0.54;
            let (detrender, q1, i2, q2);
            if today % 2 == 0 {
                detrender =
                    do_hilbert_even(&mut self.detrender, smoothed, self.hilbert_idx, adjusted);
                q1 = do_hilbert_even(&mut self.q1, detrender, self.hilbert_idx, adjusted);
                let ji = do_hilbert_even(&mut self.ji, self.even3, self.hilbert_idx, adjusted);
                let jq = do_hilbert_even(&mut self.jq, q1, self.hilbert_idx, adjusted);
                self.hilbert_idx = (self.hilbert_idx + 1) % 3;
                q2 = 0.2 * (q1 + ji) + 0.8 * self.prev_q2;
                i2 = 0.2 * (self.even3 - jq) + 0.8 * self.prev_i2;
                self.odd3 = self.odd2;
                self.odd2 = detrender;
            } else {
                detrender =
                    do_hilbert_odd(&mut self.detrender, smoothed, self.hilbert_idx, adjusted);
                q1 = do_hilbert_odd(&mut self.q1, detrender, self.hilbert_idx, adjusted);
                let ji = do_hilbert_odd(&mut self.ji, self.odd3, self.hilbert_idx, adjusted);
                let jq = do_hilbert_odd(&mut self.jq, q1, self.hilbert_idx, adjusted);
                q2 = 0.2 * (q1 + ji) + 0.8 * self.prev_q2;
                i2 = 0.2 * (self.odd3 - jq) + 0.8 * self.prev_i2;
                self.even3 = self.even2;
                self.even2 = detrender;
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
            let count = (self.smooth_period + 0.5) as i32;
            let mut real = 0.0;
            let mut imag = 0.0;
            let mut idx = self.smooth_idx;
            for i in 0..count {
                let angle = i as f64 * FULL_CIRCLE / count as f64;
                let price = self.smooth_prices[idx];
                real += angle.sin() * price;
                imag += angle.cos() * price;
                idx = if idx == 0 { 49 } else { idx - 1 };
            }
            let abs_imag = imag.abs();
            if abs_imag > 0.0 {
                self.phase = (real / imag).atan() * RAD2DEG;
            } else if abs_imag <= 0.01 {
                if real < 0.0 {
                    self.phase -= 90.0;
                } else if real > 0.0 {
                    self.phase += 90.0;
                }
            }
            self.phase += 90.0;
            self.phase += 360.0 / self.smooth_period;
            if imag < 0.0 {
                self.phase += 180.0;
            }
            if self.phase > 315.0 {
                self.phase -= 360.0;
            }
            self.value = (today >= LOOKBACK).then_some(self.phase);
            self.smooth_idx = (self.smooth_idx + 1) % 50;
            self.value
        }
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
    fn matches_batch() {
        let input: Vec<f64> = (0..400)
            .map(|i| 100.0 + (i as f64 * 0.11).sin() * 8.0)
            .collect();
        let expected =
            crate::stream::cycle::hilbert_transform_dominant_cycle_phase(&input).unwrap();
        let mut state = HilbertTransformDominantCyclePhase::new();
        for (&input, &expected) in input.iter().zip(&expected) {
            match state.append(input) {
                Some(value) => assert!((value - expected).abs() < 1e-12),
                None => assert!(expected.is_nan()),
            }
        }
    }

    #[test]
    fn bitwise_matches_oracle_and_reset_replay_on_lcg_bars() {
        let input = lcg_series(5_000, 0x1234_5678_9abc_def0);
        let mut oracle = oracle::OracleDcPhase::new();
        let mut state = HilbertTransformDominantCyclePhase::new();
        let mut outputs = Vec::with_capacity(input.len());
        for &bar in &input {
            let expected = oracle.append(bar);
            let actual = state.append(bar);
            assert_eq!(
                actual.map(f64::to_bits),
                expected.map(f64::to_bits),
                "streaming output diverged from oracle"
            );
            outputs.push(actual);
        }
        assert_eq!(
            state.current_phase().to_bits(),
            oracle.current_phase().to_bits()
        );
        assert_eq!(
            state.current_smooth_period().to_bits(),
            oracle.current_smooth_period().to_bits()
        );
        assert_eq!(
            state.current_smooth_price().to_bits(),
            oracle.current_smooth_price().to_bits()
        );
        // continue-after-reset replay must be bitwise identical.
        state.reset();
        for (&bar, expected) in input.iter().zip(&outputs) {
            let replay = state.append(bar);
            assert_eq!(replay.map(f64::to_bits), expected.map(f64::to_bits));
        }
    }
}
