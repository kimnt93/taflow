//! Pointwise `tan` mathematical transform.

/// Apply the `tan` transform to each aligned input value.
///
/// # Parameters
///
/// * `input` - Chronological input values.
///
/// # Returns
///
/// A same-length vector with the transform applied element by element.
#[inline]
pub fn tan(input: &[f64]) -> Vec<f64> {
    input.iter().map(|&value| value.tan()).collect()
}
