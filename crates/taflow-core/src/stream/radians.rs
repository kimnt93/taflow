//! Pointwise degrees-to-radians conversion.

/// Convert every aligned input observation from degrees to radians.
pub fn radians(input: &[f64]) -> Vec<f64> {
    input.iter().map(|value| value.to_radians()).collect()
}
