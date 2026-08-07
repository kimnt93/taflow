//! Pointwise cube root.

/// Return the cube root of every aligned input observation.
pub fn cbrt(input: &[f64]) -> Vec<f64> {
    input.iter().map(|value| value.cbrt()).collect()
}
