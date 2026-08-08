//! Shared sorted-ring primitive for rolling order statistics.
//!
//! Couples a fixed-capacity FIFO ring (insertion order) with a parallel
//! sorted array maintained by binary-search insert/evict and `copy_within`.
//! The sorted array is bit-identical to collecting the window and running
//! `sort_by(f64::total_cmp)` on it every bar: `total_cmp` is a total order in
//! which two values compare `Equal` only when their bit patterns are
//! identical, so the sorted permutation of a given multiset is unique.
//! Median/quantile/winsorize consumers read order statistics directly off
//! `sorted()` and keep their existing interpolation arithmetic untouched.

use std::cmp::Ordering;

/// Fixed-capacity FIFO window with a parallel `total_cmp`-sorted view.
#[derive(Debug, Clone)]
pub struct SortedRing {
    ring: Box<[f64]>,
    sorted: Box<[f64]>,
    /// Next write slot; when full this is also the index of the oldest value.
    head: usize,
    len: usize,
}

impl SortedRing {
    /// Creates an empty ring holding at most `capacity` values.
    ///
    /// `capacity` must be at least 1 (callers validate their period first).
    pub fn new(capacity: usize) -> Self {
        debug_assert!(capacity >= 1);
        Self {
            ring: vec![0.0; capacity].into_boxed_slice(),
            sorted: vec![0.0; capacity].into_boxed_slice(),
            head: 0,
            len: 0,
        }
    }

    /// Maximum number of values held.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.ring.len()
    }

    /// Current number of values held.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether the window is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Whether the window has reached capacity.
    #[inline]
    pub fn is_full(&self) -> bool {
        self.len == self.ring.len()
    }

    /// The current window sorted under `f64::total_cmp`.
    #[inline]
    pub fn sorted(&self) -> &[f64] {
        &self.sorted[..self.len]
    }

    /// Pushes one value, evicting the oldest when full.
    pub fn push(&mut self, value: f64) {
        let cap = self.ring.len();
        if self.len == cap {
            let evicted = self.ring[self.head];
            // First element total_cmp-equal to `evicted` (identical bits).
            let idx = self.sorted[..self.len]
                .partition_point(|x| x.total_cmp(&evicted) == Ordering::Less);
            debug_assert!(idx < self.len);
            self.sorted.copy_within(idx + 1..self.len, idx);
            self.len -= 1;
        }
        self.ring[self.head] = value;
        self.head += 1;
        if self.head == cap {
            self.head = 0;
        }
        let idx =
            self.sorted[..self.len].partition_point(|x| x.total_cmp(&value) == Ordering::Less);
        self.sorted.copy_within(idx..self.len, idx + 1);
        self.sorted[idx] = value;
        self.len += 1;
    }

    /// Clears the window in place without reallocating.
    pub fn clear(&mut self) {
        self.head = 0;
        self.len = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::SortedRing;

    #[test]
    fn matches_full_resort() {
        let mut state = 0x2545F4914F6CDD1Du64;
        let mut lcg = move || {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            // Quantized so ties occur.
            ((state >> 33) % 41) as f64 * 0.25 - 5.0
        };
        for cap in [1usize, 2, 5, 14, 30, 200] {
            let mut ring = SortedRing::new(cap);
            let mut window: Vec<f64> = Vec::new();
            for _ in 0..2_000 {
                let v = lcg();
                if window.len() == cap {
                    window.remove(0);
                }
                window.push(v);
                ring.push(v);
                let mut expected = window.clone();
                expected.sort_by(f64::total_cmp);
                let got = ring.sorted();
                assert_eq!(got.len(), expected.len());
                for (a, b) in got.iter().zip(expected.iter()) {
                    assert_eq!(a.to_bits(), b.to_bits());
                }
            }
        }
    }
}
