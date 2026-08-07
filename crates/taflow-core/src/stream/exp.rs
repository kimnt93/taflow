//! Pointwise `exp` mathematical transform.

/// Apply the `exp` transform to each aligned input value.
///
/// # Parameters
///
/// * `input` - Chronological input values.
///
/// # Returns
///
/// A same-length vector with the transform applied element by element.
#[inline]
pub fn exp(input: &[f64]) -> Vec<f64> {
    input.iter().map(|&value| value.exp()).collect()
}
