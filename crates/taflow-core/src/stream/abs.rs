//! Pointwise absolute value.

/// Return the absolute value of every aligned input observation.
pub fn abs(input: &[f64]) -> Vec<f64> {
    input.iter().map(|value| value.abs()).collect()
}
