//! Pointwise `atan` mathematical transform.

/// Apply the `atan` transform to each aligned input value.
///
/// # Parameters
///
/// * `input` - Chronological input values.
///
/// # Returns
///
/// A same-length vector with the transform applied element by element.
#[inline]
pub fn atan(input: &[f64]) -> Vec<f64> {
    input.iter().map(|&value| value.atan()).collect()
}
