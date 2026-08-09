//! Batch implementation for `rolling_avgdev`.

use super::statistic::*;
use crate::error::{TaError, TaResult};

/// Average Deviation (AVGDEV), measured from each window's arithmetic mean.
///
/// TA-Lib intentionally recomputes the absolute deviations for every window;
/// Compute the rolling avgdev result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
use super::rolling_statistics::*;
use super::*;

/// Stateful average absolute deviation with TA-Lib's newest-to-oldest summation order.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingAverageDeviation`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingAverageDeviation {
    period: usize,
    window: Window,
    value: Option<f64>,
}

impl RollingAverageDeviation {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            period,
            window: Window::new(period)?,
            value: None,
        })
    }
}

impl StreamingIndicator for RollingAverageDeviation {
    type Output = f64;

    /// Bulk kernel: the O(period) mean + deviation rescans are inherent to
    /// TA-Lib's AVGDEV semantics (newest-to-oldest summation order), but here
    /// they run over the contiguous input slice instead of the ring iterator.
    /// Bit-identical to per-bar [`Self::append`] in outputs and state.
    ///
    /// Note: maintaining an incremental running sum for the mean would change
    /// the summation order (and therefore low bits) versus the per-window
    /// newest-to-oldest rescan, so the rescan is kept.
    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        let period = self.period;
        let n = inputs.len();
        output.reserve(n);
        // Warm-up prologue: from index period-1 onward the ring contents are
        // exactly the trailing input-slice window, regardless of prior state.
        let prologue = n.min(period - 1);
        for &input in &inputs[..prologue] {
            output.push(self.append(input).unwrap_or(f64::NAN));
        }
        if n < period {
            return;
        }
        let period_f = period as f64;
        let mut last = f64::NAN;
        for i in (period - 1)..n {
            let window = &inputs[i + 1 - period..=i];
            // Newest-to-oldest, exactly like `window.iter().rev()` in append.
            let mut sum = 0.0;
            for &value in window.iter().rev() {
                sum += value;
            }
            let mean = sum / period_f;
            let mut deviation = 0.0;
            for &value in window.iter().rev() {
                deviation += (value - mean).abs();
            }
            last = deviation / period_f;
            output.push(last);
        }
        self.value = Some(last);
        // Rebuild the ring so subsequent appends continue bit-identically.
        self.window.clear();
        for &input in &inputs[n - period..] {
            self.window.push(input);
        }
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        self.window.push(input);
        self.value = self.window.is_full().then(|| {
            let period = self.period as f64;
            let mean = self.window.iter().rev().sum::<f64>() / period;
            self.window
                .iter()
                .rev()
                .map(|value| (*value - mean).abs())
                .sum::<f64>()
                / period
        });
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.window.clear();
        self.value = None;
    }
}
