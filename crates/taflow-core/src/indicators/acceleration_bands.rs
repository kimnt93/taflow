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

    /// Bulk kernel: materializes the paired high/low acceleration transforms
    /// once, then advances all three SMA recurrences in one fused pass.
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
        let mut transformed = Vec::with_capacity(n);
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
            transformed.push((upper_input, lower_input));
        }

        upper_out.reserve(n);
        middle_out.reserve(n);
        lower_out.reserve(n);
        let period = self.upper.period();
        let prologue = n.min(period);
        for i in 0..prologue {
            let (upper_input, lower_input) = transformed[i];
            upper_out.push(self.upper.append(upper_input).unwrap_or(f64::NAN));
            middle_out.push(self.middle.append(close[i]).unwrap_or(f64::NAN));
            lower_out.push(self.lower.append(lower_input).unwrap_or(f64::NAN));
        }
        if n <= period {
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
            return Ok(());
        }

        let period_f = period as f64;
        let mut upper_sum = self.upper.raw_sum();
        let mut middle_sum = self.middle.raw_sum();
        let mut lower_sum = self.lower.raw_sum();
        let mut latest = AccelerationBandsValue {
            upper: f64::NAN,
            middle: f64::NAN,
            lower: f64::NAN,
        };
        for i in period..n {
            upper_sum -= transformed[i - period].0;
            upper_sum += transformed[i].0;
            middle_sum -= close[i - period];
            middle_sum += close[i];
            lower_sum -= transformed[i - period].1;
            lower_sum += transformed[i].1;
            latest = AccelerationBandsValue {
                upper: upper_sum / period_f,
                middle: middle_sum / period_f,
                lower: lower_sum / period_f,
            };
            upper_out.push(latest.upper);
            middle_out.push(latest.middle);
            lower_out.push(latest.lower);
        }

        self.upper.window_mut().clear();
        self.middle.window_mut().clear();
        self.lower.window_mut().clear();
        for i in n - period..n {
            self.upper.window_mut().push(transformed[i].0);
            self.middle.window_mut().push(close[i]);
            self.lower.window_mut().push(transformed[i].1);
        }
        self.upper.store_bulk_state(upper_sum, Some(latest.upper));
        self.middle
            .store_bulk_state(middle_sum, Some(latest.middle));
        self.lower.store_bulk_state(lower_sum, Some(latest.lower));
        self.value = Some(latest);
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
