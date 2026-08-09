//! Batch implementation for `rolling_entropy`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Compute rolling Shannon entropy over an aligned input series.
///
/// Parameters are the input values and window length; the result is aligned
/// Compute the rolling entropy result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_entropy(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = RollingEntropy::new(timeperiod)?;
    Ok(input
        .iter()
        .map(|&value| state.append(value).unwrap_or(f64::NAN))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// Verbatim pre-count-map implementation, kept as the bitwise oracle.
    /// The summation order (first occurrence in window order) is what the
    /// incremental version must reproduce exactly.
    struct OldRollingEntropy {
        values: VecDeque<f64>,
        period: usize,
    }

    impl OldRollingEntropy {
        fn new(period: usize) -> Self {
            Self {
                values: VecDeque::with_capacity(period),
                period,
            }
        }

        fn append(&mut self, input: f64) -> Option<f64> {
            if self.values.len() == self.period {
                self.values.pop_front();
            }
            self.values.push_back(input);
            (self.values.len() == self.period).then(|| {
                let n = self.period as f64;
                let mut entropy = 0.0;
                let mut seen = Vec::new();
                for &candidate in &self.values {
                    if seen.contains(&candidate) {
                        continue;
                    }
                    seen.push(candidate);
                    let count = self
                        .values
                        .iter()
                        .filter(|&&value| value == candidate)
                        .count();
                    let probability = count as f64 / n;
                    entropy -= probability * probability.ln();
                }
                entropy
            })
        }
    }

    fn lcg_bars(n: usize) -> Vec<f64> {
        let mut state = 0x9E3779B97F4A7C15u64;
        (0..n)
            .map(|i| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                // Small discrete set so bins repeat; signed zeros exercise
                // the `-0.0 == 0.0` bin merge.
                if i % 71 == 0 {
                    -0.0
                } else if i % 43 == 0 {
                    0.0
                } else {
                    ((state >> 33) % 23) as f64 * 0.25 - 2.0
                }
            })
            .collect()
    }

    fn assert_bits(a: Option<f64>, b: Option<f64>, i: usize, p: usize) {
        let a = a.unwrap_or(f64::NAN);
        let b = b.unwrap_or(f64::NAN);
        assert_eq!(a.to_bits(), b.to_bits(), "bar {i} period {p}: {a} vs {b}");
    }

    #[test]
    fn bitwise_matches_old_implementation() {
        let bars = lcg_bars(5_000);
        for period in [2usize, 5, 14, 30, 200] {
            let mut old = OldRollingEntropy::new(period);
            let expected: Vec<Option<f64>> = bars.iter().map(|&v| old.append(v)).collect();
            let mut state = RollingEntropy::new(period).unwrap();
            for (i, &v) in bars.iter().enumerate() {
                assert_bits(state.append(v), expected[i], i, period);
            }
            let batch = rolling_entropy(&bars, period).unwrap();
            for (i, &v) in batch.iter().enumerate() {
                assert_bits(Some(v), expected[i], i, period);
            }
            for chunk in [1usize, 7, 97] {
                let mut state = RollingEntropy::new(period).unwrap();
                let mut i = 0;
                for block in bars.chunks(chunk) {
                    for &v in block {
                        assert_bits(state.append(v), expected[i], i, period);
                        i += 1;
                    }
                }
            }
            // Continue after bulk.
            let (head, tail) = bars.split_at(4_000);
            let mut state = RollingEntropy::new(period).unwrap();
            for &v in head {
                state.append(v);
            }
            for (j, &v) in tail.iter().enumerate() {
                assert_bits(state.append(v), expected[4_000 + j], 4_000 + j, period);
            }
            state.reset();
            for (i, &v) in bars.iter().take(500).enumerate() {
                assert_bits(state.append(v), expected[i], i, period);
            }
        }
    }
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingEntropy`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingEntropy {
    ring: Box<[f64]>,
    head: usize,
    len: usize,
    counts: std::collections::HashMap<u64, u32>,
    seen: std::collections::HashSet<u64>,
    period: usize,
    value: Option<f64>,
}

impl RollingEntropy {
    /// Map key with `f64` equality semantics: `-0.0` and `+0.0` share a bin.
    /// NaNs are never inserted (NaN equals nothing, so its count is 0).
    #[inline]
    fn count_key(value: f64) -> u64 {
        if value == 0.0 {
            0.0f64.to_bits()
        } else {
            value.to_bits()
        }
    }

    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            ring: vec![0.0; period].into_boxed_slice(),
            head: 0,
            len: 0,
            counts: std::collections::HashMap::with_capacity(period),
            seen: std::collections::HashSet::with_capacity(period),
            period,
            value: None,
        })
    }

    /// Shannon entropy of exact-value frequencies in the rolling window.
    ///
    /// The exact-value counts are maintained incrementally (integer work on
    /// the two touched bins per bar); the entropy sum itself is recomputed
    /// per bar in the original iteration order (first occurrence in window
    /// order) so the floating-point result stays bit-identical to the
    /// previous full-rescan implementation.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.len == self.period {
            let evicted = self.ring[self.head];
            if !evicted.is_nan() {
                let key = Self::count_key(evicted);
                let count = self.counts.get_mut(&key).expect("evicted value counted");
                *count -= 1;
                if *count == 0 {
                    self.counts.remove(&key);
                }
            }
        } else {
            self.len += 1;
        }
        self.ring[self.head] = input;
        self.head += 1;
        if self.head == self.period {
            self.head = 0;
        }
        if !input.is_nan() {
            *self.counts.entry(Self::count_key(input)).or_insert(0) += 1;
        }
        let value = if self.len == self.period {
            let n = self.period as f64;
            let mut entropy = 0.0;
            self.seen.clear();
            // `head` now points at the oldest value in window order.
            let start = self.head;
            for i in 0..self.period {
                let mut idx = start + i;
                if idx >= self.period {
                    idx -= self.period;
                }
                let candidate = self.ring[idx];
                let probability = if candidate.is_nan() {
                    // NaN never equals anything: count 0, exactly as the
                    // full rescan produced (0.0 * ln(0.0) => NaN result).
                    0.0
                } else {
                    let key = Self::count_key(candidate);
                    if !self.seen.insert(key) {
                        continue;
                    }
                    *self.counts.get(&key).expect("window value counted") as f64 / n
                };
                entropy -= probability * probability.ln();
            }
            Some(entropy)
        } else {
            None
        };
        self.value = value;
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
        self.head = 0;
        self.len = 0;
        self.counts.clear();
        self.seen.clear();
        self.value = None;
    }
}
