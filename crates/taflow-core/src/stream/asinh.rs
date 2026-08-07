//! Pointwise inverse hyperbolic sine.

/// Return `asinh(x)` for every aligned input observation.
pub fn asinh(input: &[f64]) -> Vec<f64> {
    input.iter().map(|value| value.asinh()).collect()
}
