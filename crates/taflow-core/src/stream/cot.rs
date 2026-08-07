//! Pointwise cotangent.

/// Return `1 / tan(x)` for every aligned input observation in radians.
pub fn cot(input: &[f64]) -> Vec<f64> {
    input.iter().map(|value| value.tan().recip()).collect()
}
