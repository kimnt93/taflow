//! Batch implementation for `frac_diff`.

use super::operator_states::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `frac_diff` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn frac_diff(input: &[f64], d: f64, threshold: f64) -> TaResult<Vec<f64>> {
    let mut state = FracDiff::new(d, threshold)?;
    Ok(input
        .iter()
        .map(|&value| state.append(value).unwrap_or(f64::NAN))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// Verbatim pre-ring implementation, kept as the bitwise oracle. The
    /// accumulation order (weights[0] * newest, then older bars) is what the
    /// contiguous-ring dot product must reproduce exactly.
    struct OldFracDiff {
        weights: Vec<f64>,
        window: VecDeque<f64>,
    }

    impl OldFracDiff {
        fn new(d: f64, threshold: f64) -> Self {
            let mut weights = vec![1.0];
            let mut k = 1usize;
            loop {
                let wk = -weights[k - 1] * (d - k as f64 + 1.0) / k as f64;
                if wk.abs() < threshold {
                    break;
                }
                weights.push(wk);
                k += 1;
            }
            let capacity = weights.len();
            Self {
                weights,
                window: VecDeque::with_capacity(capacity),
            }
        }

        fn append(&mut self, input: f64) -> Option<f64> {
            if self.window.len() == self.weights.len() {
                self.window.pop_front();
            }
            self.window.push_back(input);
            if self.window.len() == self.weights.len() {
                let mut acc = 0.0;
                for (i, &w) in self.weights.iter().enumerate() {
                    acc += w * self.window[self.window.len() - 1 - i];
                }
                Some(acc)
            } else {
                None
            }
        }
    }

    fn lcg_bars(n: usize) -> Vec<f64> {
        let mut state = 0xC6A4A7935BD1E995u64;
        (0..n)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                100.0 + (state >> 11) as f64 / (1u64 << 53) as f64 * 10.0
            })
            .collect()
    }

    fn assert_bits(a: Option<f64>, b: Option<f64>, i: usize, d: f64, t: f64) {
        let a = a.unwrap_or(f64::NAN);
        let b = b.unwrap_or(f64::NAN);
        assert_eq!(
            a.to_bits(),
            b.to_bits(),
            "bar {i} d {d} threshold {t}: {a} vs {b}"
        );
    }

    #[test]
    fn bitwise_matches_old_implementation() {
        let bars = lcg_bars(5_000);
        // Parameter grid spanning short and long weight vectors (the
        // window length plays the role of the period here).
        for (d, threshold) in [
            (0.35, 1e-2),
            (0.5, 1e-3),
            (0.5, 1e-4),
            (0.75, 1e-3),
            (1.0, 1e-3),
            (2.5, 1e-5),
        ] {
            let mut old = OldFracDiff::new(d, threshold);
            assert!(
                old.weights.len() <= 4_000,
                "weight vector must warm up within the test data"
            );
            let expected: Vec<Option<f64>> = bars.iter().map(|&v| old.append(v)).collect();
            let mut state = FracDiff::new(d, threshold).unwrap();
            for (i, &v) in bars.iter().enumerate() {
                assert_bits(state.append(v), expected[i], i, d, threshold);
            }
            let batch = frac_diff(&bars, d, threshold).unwrap();
            for (i, &v) in batch.iter().enumerate() {
                assert_bits(Some(v), expected[i], i, d, threshold);
            }
            for chunk in [1usize, 7, 97] {
                let mut state = FracDiff::new(d, threshold).unwrap();
                let mut i = 0;
                for block in bars.chunks(chunk) {
                    for &v in block {
                        assert_bits(state.append(v), expected[i], i, d, threshold);
                        i += 1;
                    }
                }
            }
            // Continue after bulk.
            let (head, tail) = bars.split_at(4_000);
            let mut state = FracDiff::new(d, threshold).unwrap();
            for &v in head {
                state.append(v);
            }
            for (j, &v) in tail.iter().enumerate() {
                assert_bits(
                    state.append(v),
                    expected[4_000 + j],
                    4_000 + j,
                    d,
                    threshold,
                );
            }
            state.reset();
            for (i, &v) in bars.iter().take(1_000).enumerate() {
                assert_bits(state.append(v), expected[i], i, d, threshold);
            }
        }
    }
}
