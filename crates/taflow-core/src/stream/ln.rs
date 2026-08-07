//! Pointwise `ln` mathematical transform.

/// Apply the `ln` transform to each aligned input value.
///
/// # Parameters
///
/// * `input` - Chronological input values.
///
/// # Returns
///
/// A same-length vector with the transform applied element by element.
#[inline]
pub fn ln(input: &[f64]) -> Vec<f64> {
    input.iter().map(|&value| value.ln()).collect()
}
