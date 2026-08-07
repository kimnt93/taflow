//! Pointwise natural logarithm of one plus the input.

/// Return `ln(1 + x)` using the numerically stable intrinsic.
pub fn log1p(input: &[f64]) -> Vec<f64> {
    input.iter().map(|value| value.ln_1p()).collect()
}
