//! Stateful pandas-ta-classic Even Better Sinewave oscillator.

use super::invalid_period;
use crate::error::TaResult;
use crate::stream::StreamingIndicator;

/// Computes a causal detrended cycle value from close prices.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `EvenBetterSinewave`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct EvenBetterSinewave {
    period: usize,
    index: usize,
    alpha1: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    previous_close: f64,
    previous_high_pass: f64,
    filter_two_back: f64,
    filter_one_back: f64,
    value: Option<f64>,
}

impl EvenBetterSinewave {
    /// Creates the oscillator with a positive nominal cycle period.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 1 {
            return Err(invalid_period("length", period, 1));
        }
        let bars = 10.0_f64;
        // pandas-ta-classic applies numpy's radian trig functions directly
        // to these degree-looking constants; preserve that public recurrence.
        let alpha1 = (1.0 - (360.0 / period as f64).sin()) / (360.0 / period as f64).cos();
        let a1 = (-2.0_f64.sqrt() * std::f64::consts::PI / bars).exp();
        let c2 = 2.0 * a1 * (2.0_f64.sqrt() * 180.0 / bars).cos();
        let c3 = -a1 * a1;
        let c1 = 1.0 - c2 - c3;
        Ok(Self {
            period,
            index: 0,
            alpha1,
            c1,
            c2,
            c3,
            previous_close: 0.0,
            previous_high_pass: 0.0,
            filter_two_back: 0.0,
            filter_one_back: 0.0,
            value: None,
        })
    }

    /// Extends a close slice through the scalar recurrence.
    pub fn extend_slice_into(&mut self, input: &[f64], output: &mut Vec<f64>) {
        output.extend(
            input
                .iter()
                .map(|&value| self.append(value).unwrap_or(f64::NAN)),
        );
    }
}

impl StreamingIndicator for EvenBetterSinewave {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        let index = self.index;
        self.index += 1;
        if index < self.period - 1 {
            self.value = None;
            return None;
        }
        if index == self.period - 1 {
            self.value = Some(0.0);
            return self.value;
        }
        let high_pass = 0.5 * (1.0 + self.alpha1) * (input - self.previous_close)
            + self.alpha1 * self.previous_high_pass;
        let filter = self.c1 * (high_pass + self.previous_high_pass) * 0.5
            + self.c2 * self.filter_one_back
            + self.c3 * self.filter_two_back;
        let wave = (filter + self.filter_one_back + self.filter_two_back) / 3.0;
        let power = (filter * filter
            + self.filter_one_back * self.filter_one_back
            + self.filter_two_back * self.filter_two_back)
            / 3.0;
        let output = if power > 0.0 {
            wave / power.sqrt()
        } else {
            0.0
        };
        self.filter_two_back = self.filter_one_back;
        self.filter_one_back = filter;
        self.previous_high_pass = high_pass;
        self.previous_close = input;
        self.value = Some(output);
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }
    fn reset(&mut self) {
        self.index = 0;
        self.previous_close = 0.0;
        self.previous_high_pass = 0.0;
        self.filter_two_back = 0.0;
        self.filter_one_back = 0.0;
        self.value = None;
    }
}
