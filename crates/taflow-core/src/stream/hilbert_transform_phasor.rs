//! Incremental Hilbert Transform phasor components (HT_PHASOR).

use std::collections::VecDeque;

use crate::stream::cycle::{do_hilbert_even, do_hilbert_odd, HilbertVars};

const RAD2DEG: f64 = 180.0 / std::f64::consts::PI;
const LOOKBACK: usize = 32;

/// In-phase and quadrature components returned by [`HilbertTransformPhasor`].
#[derive(Clone, Copy, Debug, PartialEq)]
/// Persistent Rust state or aligned output type for `HilbertTransformPhasorValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct HilbertTransformPhasorValue {
    pub inphase: f64,
    pub quadrature: f64,
}

/// Incremental HT_PHASOR state.
/// Persistent Rust state or aligned output type for `HilbertTransformPhasor`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct HilbertTransformPhasor {
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
    prev_i2: f64,
    prev_q2: f64,
    re: f64,
    im: f64,
    odd2: f64,
    odd3: f64,
    even2: f64,
    even3: f64,
    value: Option<HilbertTransformPhasorValue>,
}

impl Default for HilbertTransformPhasor {
    fn default() -> Self {
        Self::new()
    }
}

impl HilbertTransformPhasor {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
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
            prev_i2: 0.0,
            prev_q2: 0.0,
            re: 0.0,
            im: 0.0,
            odd2: 0.0,
            odd3: 0.0,
            even2: 0.0,
            even3: 0.0,
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

    /// Appends one price and returns components after TA-Lib's 32-bar warmup.
    pub fn append(&mut self, input: f64) -> Option<HilbertTransformPhasorValue> {
        let today = self.index;
        let smoothed = self.smooth(input);
        self.index += 1;
        if today < 12 {
            return None;
        }
        let adjusted = 0.075 * self.period + 0.54;
        let smoothed = smoothed.expect("WMA is initialized");
        let (detrender, q1, inphase, quadrature, i2, q2);
        if today % 2 == 0 {
            detrender = do_hilbert_even(&mut self.detrender, smoothed, self.hilbert_idx, adjusted);
            q1 = do_hilbert_even(&mut self.q1, detrender, self.hilbert_idx, adjusted);
            inphase = self.even3;
            quadrature = q1;
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
            inphase = self.odd3;
            quadrature = q1;
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
        self.value = (today >= LOOKBACK).then_some(HilbertTransformPhasorValue {
            inphase,
            quadrature,
        });
        self.value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<HilbertTransformPhasorValue> {
        self.value
    }
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}
