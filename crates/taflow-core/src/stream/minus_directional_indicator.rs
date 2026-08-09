//! Incremental Minus Directional Indicator (-DI).
use super::directional::DirectionalMovement;
use crate::error::TaResult;

/// Persistent Rust state or aligned output type for `MinusDirectionalIndicator`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct MinusDirectionalIndicator {
    directional: DirectionalMovement,
    value: Option<f64>,
}
impl MinusDirectionalIndicator {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            directional: DirectionalMovement::new(period)?,
            value: None,
        })
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        self.value = self
            .directional
            .append(high, low, close)
            .map(|v| v.minus_di);
        self.value
    }

    /// Append aligned HLC slices while preserving scalar state and warm-up.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        if high.len() != low.len() || high.len() != close.len() {
            return Err(crate::TaError::LengthMismatch {
                expected: high.len(),
                got: low.len().min(close.len()),
            });
        }
        output.reserve(high.len());
        for ((high, low), close) in high.iter().zip(low).zip(close) {
            output.push(self.append(*high, *low, *close).unwrap_or(f64::NAN));
        }
        Ok(())
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
        self.directional.reset();
        self.value = None;
    }
}
