//! Stateful TA-Lib-order rolling population standard deviation.

use crate::error::TaResult;
use crate::stream::rolling_statistics::stddev_from_variance;
use crate::stream::{StreamingIndicator, Window};

/// Standard Deviation (STDDEV) — fused single-pass var+sqrt
///
/// Eliminates intermediate var Vec (8MB at 1M bars) by computing
/// Compute the rolling std result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
/// * `nbdev` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
/// Stateful population standard deviation multiplied by `nbdev`.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingStandardDeviation`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingStandardDeviation {
    period: usize,
    period_f: f64,
    window: Window,
    sum: f64,
    sum_squares: f64,
    nbdev: f64,
    value: Option<f64>,
}

impl RollingStandardDeviation {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize, nbdev: f64) -> TaResult<Self> {
        if period < 2 {
            return Err(crate::stream::invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            period,
            period_f: period as f64,
            window: Window::new(period - 1)?,
            sum: 0.0,
            sum_squares: 0.0,
            nbdev,
            value: None,
        })
    }
}

impl StreamingIndicator for RollingStandardDeviation {
    type Output = f64;

    /// Bulk kernel preserving TA-Lib's add/emit/subtract statement order.
    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        let n = inputs.len();
        if n == 0 {
            return;
        }
        let trailing = self.period - 1;
        let start = output.len();
        output.resize(start + n, f64::NAN);

        // Consume enough scalar inputs that the remaining direct loop can use
        // this chunk itself as its trailing ring, regardless of prior state.
        let prologue = n.min(trailing);
        for index in 0..prologue {
            output[start + index] = self.append(inputs[index]).unwrap_or(f64::NAN);
        }
        if n <= trailing {
            return;
        }

        let mut sum = self.sum;
        let mut sum_squares = self.sum_squares;
        let period_f = self.period_f;
        let nbdev = self.nbdev;
        let mut last = None;
        for index in trailing..n {
            let input = inputs[index];
            sum += input;
            sum_squares += input * input;
            let mean1 = sum / period_f;
            let mean2 = sum_squares / period_f;
            let variance = mean2 - mean1 * mean1;
            let standard_deviation = stddev_from_variance(variance, nbdev);
            output[start + index] = standard_deviation;
            last = Some(standard_deviation);
            let old = inputs[index - trailing];
            sum -= old;
            sum_squares -= old * old;
        }

        self.sum = sum;
        self.sum_squares = sum_squares;
        self.window.clear();
        for &input in &inputs[n - trailing..] {
            self.window.push(input);
        }
        self.value = last;
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        self.sum += input;
        self.sum_squares += input * input;
        self.value = self.window.is_full().then(|| {
            let mean1 = self.sum / self.period_f;
            let mean2 = self.sum_squares / self.period_f;
            let variance = mean2 - mean1 * mean1;
            stddev_from_variance(variance, self.nbdev)
        });
        if let Some(old) = self.window.push(input) {
            self.sum -= old;
            self.sum_squares -= old * old;
        }
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.window.clear();
        self.sum = 0.0;
        self.sum_squares = 0.0;
        self.value = None;
    }
}
