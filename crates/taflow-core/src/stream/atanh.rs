//! Pointwise inverse hyperbolic tangent.

/// Return `atanh(x)` for every aligned input observation.
pub fn atanh(input: &[f64]) -> Vec<f64> {
    input.iter().map(|value| value.atanh()).collect()
}
