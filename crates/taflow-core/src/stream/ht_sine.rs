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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let input: Vec<f64> = (0..400)
            .map(|i| 100.0 + (i as f64 * 0.11).sin() * 8.0)
            .collect();
        let (sine, leadsine) = crate::stream::cycle::hilbert_transform_sine_wave(&input).unwrap();
        let mut state = HilbertTransformSineWave::new();
        for ((&input, &sine), &leadsine) in input.iter().zip(&sine).zip(&leadsine) {
            match state.append(input) {
                Some(value) => {
                    assert!((value.sine - sine).abs() < 1e-12);
                    assert!((value.leadsine - leadsine).abs() < 1e-12);
                }
                None => {
                    assert!(sine.is_nan());
                    assert!(leadsine.is_nan());
                }
            }
        }
    }
}
