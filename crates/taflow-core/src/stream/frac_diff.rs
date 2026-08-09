//! Batch implementation for `frac_diff`.

use super::operator_states::*;
use super::*;
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
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// Fractionally-differentiated series (AFML §5.4, fixed-width window).
///
/// Weights `w_0 = 1`, `w_k = −w_{k−1}·(d−k+1)/k` truncated once
/// `|w_k| < threshold`; each output is the dot product of the weights with the
/// last `len(weights)` inputs — O(w) per bar over a ring buffer.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `FracDiff`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct FracDiff {
    weights: Box<[f64]>,
    /// Double-written ring of `2 * weights.len()` slots: each input is
    /// written at `pos` and `pos + width`, so the current window is always
    /// the contiguous slice `buffer[pos..pos + width]` (oldest to newest).
    buffer: Box<[f64]>,
    pos: usize,
    len: usize,
    value: Option<f64>,
}

impl FracDiff {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(d: f64, threshold: f64) -> TaResult<Self> {
        if !(d > 0.0) {
            return Err(TaError::InvalidParameter {
                name: "d",
                value: d.to_string(),
                reason: "must be > 0",
            });
        }
        if !(threshold > 0.0) {
            return Err(TaError::InvalidParameter {
                name: "threshold",
                value: threshold.to_string(),
                reason: "must be > 0",
            });
        }
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
        Ok(Self {
            weights: weights.into_boxed_slice(),
            buffer: vec![0.0; 2 * capacity].into_boxed_slice(),
            pos: 0,
            len: 0,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    ///
    /// The dot product accumulates newest-first (`weights[0] * latest`, then
    /// older bars), in the same order and with the same `acc += w * x`
    /// operation as the previous `VecDeque` implementation, so results are
    /// bit-identical; only the storage changed to a contiguous slice.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        let width = self.weights.len();
        self.buffer[self.pos] = input;
        self.buffer[self.pos + width] = input;
        self.pos += 1;
        if self.pos == width {
            self.pos = 0;
        }
        if self.len < width {
            self.len += 1;
        }
        self.value = if self.len == width {
            let window = &self.buffer[self.pos..self.pos + width];
            let mut acc = 0.0;
            for (&w, &x) in self.weights.iter().zip(window.iter().rev()) {
                acc += w * x;
            }
            Some(acc)
        } else {
            None
        };
        self.value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.pos = 0;
        self.len = 0;
        self.value = None;
    }
}
