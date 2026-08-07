//! Pointwise inverse hyperbolic cosine.

/// Return `acosh(x)` for every aligned input observation.
pub fn acosh(input: &[f64]) -> Vec<f64> {
    input.iter().map(|value| value.acosh()).collect()
}
