//! Incremental Triangular Moving Average (TRIMA).

use crate::error::TaResult;

use crate::stream::{invalid_period, SimpleMovingAverage, StreamingIndicator};

/// Compute the triangular moving average result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
/// Stateful triangular moving average as two cascaded SMA windows.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `TriangularMovingAverage`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct TriangularMovingAverage {
    sma1: SimpleMovingAverage,
    sma2: SimpleMovingAverage,
    value: Option<f64>,
}

impl TriangularMovingAverage {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("timeperiod", period, 1));
        }
        let (p1, p2) = if period % 2 == 1 {
            let half = (period + 1) / 2;
            (half, half)
        } else {
            (period / 2 + 1, period / 2)
        };
        Ok(Self {
            sma1: SimpleMovingAverage::new(p1)?,
            sma2: SimpleMovingAverage::new(p2)?,
            value: None,
        })
    }
}

impl StreamingIndicator for TriangularMovingAverage {
    type Output = f64;

    /// Bulk kernel that streams first-stage averages directly into stage two.
    ///
    /// A bounded first-stage prologue displaces any prior ring contents in
    /// scalar order. Its steady recurrence then evicts directly from `inputs`,
    /// avoiding a full-size intermediate allocation while leaving both SMA
    /// states bit-identical to scalar replay.
    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        let n = inputs.len();
        if n == 0 {
            return;
        }
        let period = self.sma1.period();
        output.reserve(n);

        let prologue = n.min(period);
        for &input in &inputs[..prologue] {
            output.push(self.append(input).unwrap_or(f64::NAN));
        }
        if n <= period {
            return;
        }

        let period_f = period as f64;
        let mut sum = self.sma1.raw_sum();
        for index in period..n {
            sum -= inputs[index - period];
            sum += inputs[index];
            let first = sum / period_f;
            output.push(self.sma2.append(first).unwrap_or(f64::NAN));
        }

        self.sma1.window_mut().clear();
        for &input in &inputs[n - period..] {
            self.sma1.window_mut().push(input);
        }
        self.sma1.store_bulk_state(sum, Some(sum / period_f));
        self.value = self.sma2.value();
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self
            .sma1
            .append(input)
            .and_then(|first| self.sma2.append(first));
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.sma1.reset();
        self.sma2.reset();
        self.value = None;
    }
}
