//! Pointwise `log10` mathematical transform.

/// Apply the `log10` transform to each aligned input value.
///
/// # Parameters
///
/// * `input` - Chronological input values.
///
/// # Returns
///
/// A same-length vector with the transform applied element by element.
#[inline]
pub fn log10(input: &[f64]) -> Vec<f64> {
    input.iter().map(|&value| value.log10()).collect()
}
