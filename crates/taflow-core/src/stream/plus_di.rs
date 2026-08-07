//! Incremental Plus Directional Indicator (+DI).
use super::directional::DirectionalMovement;
use crate::error::TaResult;

/// Computes an aligned Plus Directional Indicator vector from HLC slices.
pub fn plus_directional_indicator(high: &[f64], low: &[f64], close: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if high.len() != low.len() || high.len() != close.len() {
        return Err(crate::TaError::LengthMismatch { expected: high.len(), got: low.len().min(close.len()) });
    }
    let mut state = PlusDirectionalIndicator::new(timeperiod)?;
    Ok(high.iter().zip(low).zip(close).map(|((high, low), close)| state.append(*high, *low, *close).unwrap_or(f64::NAN)).collect())
}

pub struct PlusDirectionalIndicator {
    directional: DirectionalMovement,
    value: Option<f64>,
}
impl PlusDirectionalIndicator {
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
        self.value = self.directional.append(high, low, close).map(|v| v.plus_di);
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
        self.directional.reset();
        self.value = None;
    }
}
