//! Rolling midpoint and midprice streaming states.

use crate::error::{TaError, TaResult};

use super::{RollingExtrema, StreamingIndicator};

/// Stateful midpoint of the rolling highest and lowest input values.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingMidpoint`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingMidpoint {
    extrema: RollingExtrema,
    value: Option<f64>,
}

impl RollingMidpoint {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            extrema: RollingExtrema::new(period)?,
            value: None,
        })
    }
}

impl StreamingIndicator for RollingMidpoint {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self
            .extrema
            .append(input)
            .map(|(maximum, minimum)| (maximum + minimum) * 0.5);
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.extrema.reset();
        self.value = None;
    }
}

/// Stateful midpoint of rolling high maxima and low minima.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingMidprice`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingMidprice {
    highs: RollingExtrema,
    lows: RollingExtrema,
    value: Option<f64>,
}

impl RollingMidprice {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            highs: RollingExtrema::new(period)?,
            lows: RollingExtrema::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        let maximum = self.highs.append(high).map(|values| values.0);
        let minimum = self.lows.append(low).map(|values| values.1);
        self.value = maximum.zip(minimum).map(|(high, low)| (high + low) * 0.5);
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

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.highs.reset();
        self.lows.reset();
        self.value = None;
    }
}
