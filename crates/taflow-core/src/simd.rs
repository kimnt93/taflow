//! Bulk array kernels.
//!
//! Plain, auto-vectorizable loops. Element-wise IEEE operations (+, -, *, /,
//! sqrt, abs, max, min) round identically whether executed scalar or in SIMD
//! lanes, so these kernels are compiled for multiple x86-64 feature levels via
//! `multiversion` and dispatched at runtime without affecting results.
//!
//! Reductions (`sum_f64`, `sum_sq_diff`) are intentionally SERIAL: their
//! summation order feeds indicator seeds (EMA/CORREL/TRIX) and must match the
//! order the streaming `append` path accumulates in, or chunk invariance
//! breaks. Do not reassociate them.

// The multiversion attribute expands cfg(target_feature) checks that rustc's
// check-cfg lint does not know about; harmless.
#![allow(unexpected_cfgs)]

use multiversion::multiversion;

/// Serial array sum, in slice order.
///
/// Must stay serial: seeds computed here have to be bit-identical to the
/// running `sum += value` accumulation the streaming paths perform.
pub fn sum_f64(data: &[f64]) -> f64 {
    let mut total = 0.0;
    for &value in data {
        total += value;
    }
    total
}

/// Serial squared-difference sum: Σ(x - mean)², in slice order.
pub fn sum_sq_diff(data: &[f64], mean: f64) -> f64 {
    let mut total = 0.0;
    for &value in data {
        let d = value - mean;
        total += d * d;
    }
    total
}

macro_rules! binary_arrays {
    ($name:ident, $op:tt) => {
        #[multiversion(targets("x86_64+avx2+fma", "x86_64+avx", "x86_64+sse4.2"))]
        pub fn $name(a: &[f64], b: &[f64]) -> Vec<f64> {
            a.iter().zip(b).map(|(&x, &y)| x $op y).collect()
        }
    };
}

binary_arrays!(add_arrays, +);
binary_arrays!(sub_arrays, -);
binary_arrays!(mult_arrays, *);
binary_arrays!(div_arrays, /);

#[multiversion(targets("x86_64+avx2+fma", "x86_64+avx", "x86_64+sse4.2"))]
pub fn sqrt_array(input: &[f64]) -> Vec<f64> {
    input.iter().map(|&x| x.sqrt()).collect()
}

#[multiversion(targets("x86_64+avx2+fma", "x86_64+avx", "x86_64+sse4.2"))]
pub fn abs_array(input: &[f64]) -> Vec<f64> {
    input.iter().map(|&x| x.abs()).collect()
}

/// Element-wise transcendental kernels. These lower to scalar libm calls, so
/// there is nothing to multiversion; a plain loop keeps codegen simple.
macro_rules! unary_array {
    ($name:ident, $op:expr) => {
        pub fn $name(input: &[f64]) -> Vec<f64> {
            input.iter().map(|&x| $op(x)).collect()
        }
    };
}

unary_array!(sin_array, f64::sin);
unary_array!(cos_array, f64::cos);
unary_array!(tan_array, f64::tan);
unary_array!(asin_array, f64::asin);
unary_array!(acos_array, f64::acos);
unary_array!(atan_array, f64::atan);
unary_array!(sinh_array, f64::sinh);
unary_array!(cosh_array, f64::cosh);
unary_array!(tanh_array, f64::tanh);
unary_array!(exp_array, f64::exp);
unary_array!(ln_array, f64::ln);
unary_array!(log10_array, f64::log10);
unary_array!(ceil_array, f64::ceil);
unary_array!(floor_array, f64::floor);

/// Max of a slice with its index; on ties the LAST occurrence wins
/// (TA-Lib latest-wins semantics). `data` must not be empty.
pub fn slice_max_with_index(data: &[f64]) -> (f64, usize) {
    let mut best = data[0];
    let mut best_idx = 0;
    for (i, &value) in data.iter().enumerate().skip(1) {
        if value >= best {
            best = value;
            best_idx = i;
        }
    }
    (best, best_idx)
}

/// Min of a slice with its index; on ties the LAST occurrence wins.
/// `data` must not be empty.
pub fn slice_min_with_index(data: &[f64]) -> (f64, usize) {
    let mut best = data[0];
    let mut best_idx = 0;
    for (i, &value) in data.iter().enumerate().skip(1) {
        if value <= best {
            best = value;
            best_idx = i;
        }
    }
    (best, best_idx)
}

/// Offset subtraction: output[i] = input[i] - input[i-offset] for i in
/// [offset..len). output[0..offset] is left untouched (caller sets NaN).
#[multiversion(targets("x86_64+avx2+fma", "x86_64+avx", "x86_64+sse4.2"))]
pub fn sub_offset_simd(input: &[f64], output: &mut [f64], offset: usize) {
    for i in offset..input.len() {
        output[i] = input[i] - input[i - offset];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_f64() {
        let data: Vec<f64> = (1..=100).map(|x| x as f64).collect();
        let result = sum_f64(&data);
        assert!((result - 5050.0).abs() < 1e-10);
    }

    #[test]
    fn test_sum_f64_non_aligned() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        let result = sum_f64(&data);
        assert!((result - 28.0).abs() < 1e-10);
    }

    #[test]
    fn test_sum_sq_diff() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let mean = 3.0;
        let result = sum_sq_diff(&data, mean);
        assert!((result - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_add_arrays() {
        let a: Vec<f64> = (1..=10).map(|x| x as f64).collect();
        let b: Vec<f64> = (11..=20).map(|x| x as f64).collect();
        let result = add_arrays(&a, &b);
        for i in 0..10 {
            assert!((result[i] - (a[i] + b[i])).abs() < 1e-10);
        }
    }

    #[test]
    fn test_sqrt_array() {
        let input = vec![1.0, 4.0, 9.0, 16.0, 25.0];
        let result = sqrt_array(&input);
        assert!((result[0] - 1.0).abs() < 1e-10);
        assert!((result[1] - 2.0).abs() < 1e-10);
        assert!((result[2] - 3.0).abs() < 1e-10);
        assert!((result[3] - 4.0).abs() < 1e-10);
        assert!((result[4] - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_sin_array() {
        let input = vec![0.0, std::f64::consts::FRAC_PI_2, std::f64::consts::PI];
        let result = sin_array(&input);
        assert!((result[0] - 0.0).abs() < 1e-10);
        assert!((result[1] - 1.0).abs() < 1e-10);
        assert!(result[2].abs() < 1e-10);
    }

    #[test]
    fn test_slice_max_ties_latest_wins() {
        let data = vec![3.0, 5.0, 5.0, 2.0];
        assert_eq!(slice_max_with_index(&data), (5.0, 2));
        let data = vec![1.0, 1.0, 1.0];
        assert_eq!(slice_max_with_index(&data), (1.0, 2));
        assert_eq!(slice_min_with_index(&data), (1.0, 2));
    }
}
