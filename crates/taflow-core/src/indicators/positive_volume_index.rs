use super::*;
use crate::stream::operator_states::*;

/// Persistent Rust state or aligned output type for `PositiveVolumeIndex`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct PositiveVolumeIndex(VolumeIndex);

impl PositiveVolumeIndex {
    /// Create a new empty state.
    ///
    pub fn new() -> Self {
        Self(VolumeIndex::new(VolumeIndexMode::Positive))
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, close: f64, volume: f64) -> f64 {
        self.0.append(close, volume)
    }
    /// Return the latest computed result, if warm-up is complete.
    ///
    pub fn value(&self) -> f64 {
        self.0.value
    }
    /// Reset the state and clear its accumulated history.
    ///
    pub fn reset(&mut self) {
        self.0.reset();
    }
}
