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

    /// Extend the state with a chronological slice and aligned NaN warm-up.
    pub fn extend_slice_into(&mut self, input: &[f64], output: &mut Vec<f64>) {
        output.extend(
            input
                .iter()
                .copied()
                .map(|value| self.append(value).unwrap_or(f64::NAN)),
        );
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
