//! Stateful Directional Movement Index.

use crate::error::TaResult;

use crate::stream::directional::DirectionalMovement;

/// Incremental DX with TA-Lib-compatible Wilder smoothing and lookback.
/// Persistent Rust state or aligned output type for `DirectionalMovementIndex`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct DirectionalMovementIndex {
    directional: DirectionalMovement,
    value: Option<f64>,
}

impl DirectionalMovementIndex {
    /// Creates a DX state with a period of at least two bars.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            directional: DirectionalMovement::new(period)?,
            value: None,
        })
    }

    /// Appends one high, low, and close observation.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        self.value = self
            .directional
            .append(high, low, close)
            .map(|value| value.dx);
        self.value
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restores the post-construction state.
    pub fn reset(&mut self) {
        self.directional.reset();
        self.value = None;
    }
}
