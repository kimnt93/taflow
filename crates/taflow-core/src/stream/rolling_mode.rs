//! Rolling mode state.

use std::collections::HashMap;

use super::operator_states::validate_period;
use crate::TaResult;

/// Map key with the semantics of `f64` equality: `-0.0` and `+0.0` share a
/// bin. NaNs are never inserted (NaN equals nothing, so its count is 0).
#[inline]
fn count_key(value: f64) -> u64 {
    if value == 0.0 {
        0.0f64.to_bits()
    } else {
        value.to_bits()
    }
}

/// Computes the causal most-frequent value over a fixed trailing window.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingMode`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
///
/// The exact-value counts are maintained incrementally (integer work on the
/// two touched bins per bar); the winning bin is re-selected each bar by a
/// single window-order scan that reproduces the original tie semantics:
/// earliest value in window order wins among maximal counts, and NaN (whose
/// `==` count is zero) is never selected.
pub struct RollingMode {
    ring: Box<[f64]>,
    head: usize,
    len: usize,
    counts: HashMap<u64, u32>,
    timeperiod: usize,
    value: Option<f64>,
}

impl RollingMode {
    /// Creates an empty rolling-mode state.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self {
            ring: vec![0.0; timeperiod].into_boxed_slice(),
            head: 0,
            len: 0,
            counts: HashMap::with_capacity(timeperiod),
            timeperiod,
            value: None,
        })
    }

    /// Appends one observation and returns the mode after warm-up.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.len == self.timeperiod {
            let evicted = self.ring[self.head];
            if !evicted.is_nan() {
                let key = count_key(evicted);
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
        if self.head == self.timeperiod {
            self.head = 0;
        }
        if !input.is_nan() {
            *self.counts.entry(count_key(input)).or_insert(0) += 1;
        }
        self.value = if self.len == self.timeperiod {
            // `head` now points at the oldest value in window order.
            let start = self.head;
            let mut best = self.ring[start % self.timeperiod];
            let mut best_count = 0u32;
            for i in 0..self.timeperiod {
                let mut idx = start + i;
                if idx >= self.timeperiod {
                    idx -= self.timeperiod;
                }
                let candidate = self.ring[idx];
                let count = if candidate.is_nan() {
                    0
                } else {
                    *self.counts.get(&count_key(candidate)).expect("counted")
                };
                if count > best_count {
                    best = candidate;
                    best_count = count;
                }
            }
            Some(best)
        } else {
            None
        };
        self.value
    }

    /// Returns the latest mode, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clears the trailing window and latest output.
    pub fn reset(&mut self) {
        self.head = 0;
        self.len = 0;
        self.counts.clear();
        self.value = None;
    }
}

/// Rolling mode. Warm-up values are `NaN`; exact-value ties keep the earliest
/// Compute the rolling mode result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rolling_mode(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_period(timeperiod)?;
    let mut state = RollingMode::new(timeperiod)?;
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
    struct OldRollingMode {
        values: VecDeque<f64>,
        timeperiod: usize,
    }

    impl OldRollingMode {
        fn new(timeperiod: usize) -> Self {
            Self {
                values: VecDeque::with_capacity(timeperiod),
                timeperiod,
            }
        }

        fn append(&mut self, input: f64) -> Option<f64> {
            if self.values.len() == self.timeperiod {
                self.values.pop_front();
            }
            self.values.push_back(input);
            if self.values.len() == self.timeperiod {
                let mut best = self.values[0];
                let mut best_count = 0;
                for &candidate in &self.values {
                    let count = self
                        .values
                        .iter()
                        .filter(|&&value| value == candidate)
                        .count();
                    if count > best_count {
                        best = candidate;
                        best_count = count;
                    }
                }
                Some(best)
            } else {
                None
            }
        }
    }

    fn lcg_bars(n: usize) -> Vec<f64> {
        let mut state = 0xDA3E39CB94B95BDBu64;
        (0..n)
            .map(|i| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                // Small discrete set so real mode ties occur; sprinkle in
                // signed zeros to exercise the `-0.0 == 0.0` bin merge.
                if i % 61 == 0 {
                    -0.0
                } else if i % 53 == 0 {
                    0.0
                } else {
                    ((state >> 33) % 17) as f64 * 0.5 - 4.0
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
            let mut old = OldRollingMode::new(period);
            let expected: Vec<Option<f64>> = bars.iter().map(|&v| old.append(v)).collect();
            let mut state = RollingMode::new(period).unwrap();
            for (i, &v) in bars.iter().enumerate() {
                assert_bits(state.append(v), expected[i], i, period);
            }
            let batch = rolling_mode(&bars, period).unwrap();
            for (i, &v) in batch.iter().enumerate() {
                assert_bits(Some(v), expected[i], i, period);
            }
            for chunk in [1usize, 7, 97] {
                let mut state = RollingMode::new(period).unwrap();
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
            let mut state = RollingMode::new(period).unwrap();
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
