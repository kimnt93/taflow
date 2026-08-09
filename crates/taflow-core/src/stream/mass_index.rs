use super::operator_states::*;
use super::*;
use crate::error::TaResult;

/// Stateful Mass Index (Dorsey): rolling sum of the ratio between a short EMA
/// of the high-low range and an EMA of that EMA.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `MassIndex`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct MassIndex {
    ema_range: MassEma,
    ema_signal: MassEma,
    ratio_sum: crate::stream::RollingSum,
    value: Option<f64>,
}

impl MassIndex {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(ema_period: usize, sum_period: usize) -> TaResult<Self> {
        validate_period(ema_period)?;
        validate_period(sum_period)?;
        Ok(Self {
            ema_range: MassEma::new(ema_period),
            ema_signal: MassEma::new(ema_period),
            ratio_sum: crate::stream::RollingSum::new(sum_period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        let range_ema = self.ema_range.append(high - low);
        let signal_ema = range_ema.and_then(|value| self.ema_signal.append(value));
        self.value = signal_ema.and_then(|signal| {
            let range = range_ema?;
            let ratio = if signal == 0.0 { 0.0 } else { range / signal };
            self.ratio_sum.append(ratio)
        });
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

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.ema_range.reset();
        self.ema_signal.reset();
        self.ratio_sum.reset();
        self.value = None;
    }
}
