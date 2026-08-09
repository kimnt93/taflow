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

    /// Bulk kernel: runs the first SMA's bulk path into a scratch buffer and
    /// feeds the emitted suffix through the second SMA's bulk path, exactly
    /// mirroring the `sma1.append(..).and_then(|v| sma2.append(v))` chain.
    /// Bit-identical to per-bar [`Self::append`] in outputs and state.
    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        if inputs.is_empty() {
            return;
        }
        let n = inputs.len();
        output.reserve(n);
        // Bars before `sma1` warms up never reach `sma2` (matches and_then).
        // `sma1` emits its first value on the append that fills its window,
        // i.e. at index `warmup_remaining - 1` (0 when already warm).
        let first_valid = self.sma1.warmup_remaining().saturating_sub(1).min(n);
        let mut stage1 = Vec::with_capacity(n);
        self.sma1.extend_slice_into(inputs, &mut stage1);
        for _ in 0..first_valid {
            output.push(f64::NAN);
        }
        if first_valid == n {
            self.value = None;
            return;
        }
        self.sma2.extend_slice_into(&stage1[first_valid..], output);
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
