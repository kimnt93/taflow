use super::*;
use crate::stream::operator_states::*;

/// Stateful Volume-price Trend, aligned to `ta.volume.VolumePriceTrendIndicator`.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `VolumePriceTrend`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct VolumePriceTrend {
    previous_close: Option<f64>,
    total: f64,
    value: Option<f64>,
}

impl VolumePriceTrend {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            previous_close: None,
            total: 0.0,
            value: None,
        }
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, close: f64, volume: f64) -> Option<f64> {
        let previous = self.previous_close.replace(close);
        self.value = previous.map(|previous| {
            if previous != 0.0 {
                self.total += volume * (close - previous) / previous;
            }
            self.total
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
        self.previous_close = None;
        self.total = 0.0;
        self.value = None;
    }
}
