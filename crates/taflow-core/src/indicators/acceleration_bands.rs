//! Stateful Acceleration Bands.
//!
//! ACCBANDS applies TA-Lib's high/low acceleration transform and advances
//! three aligned simple moving averages for upper, middle, and lower bands.

use crate::error::TaResult;

use crate::stream::{invalid_period, SimpleMovingAverage, StreamingIndicator};

/// One aligned upper, middle, and lower Acceleration Bands observation.
#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `AccelerationBandsValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct AccelerationBandsValue {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}

/// Incremental Acceleration Bands with constant per-bar work.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `AccelerationBands`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct AccelerationBands {
    upper: SimpleMovingAverage,
    middle: SimpleMovingAverage,
    lower: SimpleMovingAverage,
    value: Option<AccelerationBandsValue>,
}

impl AccelerationBands {
    /// Creates an ACCBANDS state for a period of at least two bars.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            upper: SimpleMovingAverage::new(period)?,
            middle: SimpleMovingAverage::new(period)?,
            lower: SimpleMovingAverage::new(period)?,
            value: None,
        })
    }

    /// Appends one high, low, and close bar.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<AccelerationBandsValue> {
        let denominator = high + low;
        let (upper_input, lower_input) = if denominator == 0.0 {
            (high, low)
        } else {
            let adjustment = 4.0 * (high - low) / denominator;
            (high * (1.0 + adjustment), low * (1.0 - adjustment))
        };
        let upper = self.upper.append(upper_input);
        let middle = self.middle.append(close);
        let lower = self.lower.append(lower_input);
        self.value = upper
            .zip(middle)
            .zip(lower)
            .map(|((upper, middle), lower)| AccelerationBandsValue {
                upper,
                middle,
                lower,
            });
        self.value
    }

    /// Bulk kernel: materializes the deterministic high/low acceleration
    /// transforms once, then advances each band through the SMA bulk path
    /// (O(1) add/evict sliding sums over the transformed slices).
    /// Bit-identical to per-bar [`Self::append`] in outputs and state.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        upper_out: &mut Vec<f64>,
        middle_out: &mut Vec<f64>,
        lower_out: &mut Vec<f64>,
    ) -> TaResult<()> {
        if high.len() != low.len() || high.len() != close.len() {
            return Err(crate::TaError::LengthMismatch {
                expected: high.len(),
                got: low.len().min(close.len()),
            });
        }
        let n = high.len();
        if n == 0 {
            return Ok(());
        }
        let mut upper_inputs = Vec::with_capacity(n);
        let mut lower_inputs = Vec::with_capacity(n);
        for i in 0..n {
            let high = high[i];
            let low = low[i];
            let denominator = high + low;
            let (upper_input, lower_input) = if denominator == 0.0 {
                (high, low)
            } else {
                let adjustment = 4.0 * (high - low) / denominator;
                (high * (1.0 + adjustment), low * (1.0 - adjustment))
            };
            upper_inputs.push(upper_input);
            lower_inputs.push(lower_input);
        }
        self.upper.extend_slice_into(&upper_inputs, upper_out);
        self.middle.extend_slice_into(close, middle_out);
        self.lower.extend_slice_into(&lower_inputs, lower_out);
        self.value = self
            .upper
            .value()
            .zip(self.middle.value())
            .zip(self.lower.value())
            .map(|((upper, middle), lower)| AccelerationBandsValue {
                upper,
                middle,
                lower,
            });
        Ok(())
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<AccelerationBandsValue> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.upper.reset();
        self.middle.reset();
        self.lower.reset();
        self.value = None;
    }
}
