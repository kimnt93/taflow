//! Pointwise radians-to-degrees conversion.

/// Convert every aligned input observation from radians to degrees.
pub fn degrees(input: &[f64]) -> Vec<f64> {
    input.iter().map(|value| value.to_degrees()).collect()
}
