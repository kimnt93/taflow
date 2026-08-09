//! Incremental Hilbert Transform sine wave (HT_SINE).

use crate::stream::HilbertTransformDominantCyclePhase;

const DEG2RAD: f64 = std::f64::consts::PI / 180.0;

/// Sine and lead-sine values returned by [`HilbertTransformSineWave`].
#[derive(Clone, Copy, Debug, PartialEq)]
/// Persistent Rust state or aligned output type for `HilbertTransformSineWaveValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct HilbertTransformSineWaveValue {
    pub sine: f64,
    pub leadsine: f64,
}

/// Incremental HT_SINE state.
/// Persistent Rust state or aligned output type for `HilbertTransformSineWave`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct HilbertTransformSineWave {
    phase: HilbertTransformDominantCyclePhase,
    value: Option<HilbertTransformSineWaveValue>,
}

impl Default for HilbertTransformSineWave {
    fn default() -> Self {
        Self::new()
    }
}
impl HilbertTransformSineWave {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            phase: HilbertTransformDominantCyclePhase::new(),
            value: None,
        }
    }
    /// Appends one price and returns values after TA-Lib's 63-bar warmup.
    pub fn append(&mut self, input: f64) -> Option<HilbertTransformSineWaveValue> {
        self.value = self
            .phase
            .append(input)
            .map(|phase| HilbertTransformSineWaveValue {
                sine: (phase * DEG2RAD).sin(),
                leadsine: ((phase + 45.0) * DEG2RAD).sin(),
            });
        self.value
    }
    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<HilbertTransformSineWaveValue> {
        self.value
    }
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.phase.reset();
        self.value = None;
    }
}
