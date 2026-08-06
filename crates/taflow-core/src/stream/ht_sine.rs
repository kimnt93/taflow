//! Incremental Hilbert Transform sine wave (HT_SINE).

use crate::stream::HtDcphase;

const DEG2RAD: f64 = std::f64::consts::PI / 180.0;

/// Sine and lead-sine values returned by [`HtSine`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HtSineValue {
    pub sine: f64,
    pub leadsine: f64,
}

/// Incremental HT_SINE state.
pub struct HtSine {
    phase: HtDcphase,
    value: Option<HtSineValue>,
}

impl Default for HtSine {
    fn default() -> Self {
        Self::new()
    }
}
impl HtSine {
    pub fn new() -> Self {
        Self {
            phase: HtDcphase::new(),
            value: None,
        }
    }
    /// Appends one price and returns values after TA-Lib's 63-bar warmup.
    pub fn append(&mut self, input: f64) -> Option<HtSineValue> {
        self.value = self.phase.append(input).map(|phase| HtSineValue {
            sine: (phase * DEG2RAD).sin(),
            leadsine: ((phase + 45.0) * DEG2RAD).sin(),
        });
        self.value
    }
    pub fn value(&self) -> Option<HtSineValue> {
        self.value
    }
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
        let (sine, leadsine) = crate::cycle::ht_sine(&input).unwrap();
        let mut state = HtSine::new();
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
