//! Rolling mode state.

use std::collections::HashMap;

use crate::stream::validate_period;
use crate::TaResult;

const NONE: usize = usize::MAX;

#[inline]
fn count_key(value: f64) -> u64 {
    if value == 0.0 {
        0.0f64.to_bits()
    } else {
        value.to_bits()
    }
}

#[derive(Debug, Clone)]
struct ModeBin {
    count: u32,
    first: usize,
    last: usize,
    heap_index: usize,
}

impl Default for ModeBin {
    fn default() -> Self {
        Self {
            count: 0,
            first: NONE,
            last: NONE,
            heap_index: NONE,
        }
    }
}

/// Computes the causal most-frequent value over a fixed trailing window.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingMode`.
///
/// Exact-value counts and per-value occurrence queues are updated in constant
/// time. An indexed max heap selects the largest count in `O(log period)` and
/// uses the oldest occurrence as its tie breaker, preserving window-order tie
/// semantics without scanning the entire ring. NaNs do not form a value bin.
pub struct RollingMode {
    ring: Box<[f64]>,
    positions: Box<[u64]>,
    next_same: Box<[usize]>,
    head: usize,
    len: usize,
    bins_by_key: HashMap<u64, usize>,
    bins: Box<[ModeBin]>,
    free_bins: Vec<usize>,
    heap: Box<[usize]>,
    heap_len: usize,
    sequence: u64,
    timeperiod: usize,
    value: Option<f64>,
}

impl RollingMode {
    /// Creates an empty rolling-mode state.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self {
            ring: vec![0.0; timeperiod].into_boxed_slice(),
            positions: vec![0; timeperiod].into_boxed_slice(),
            next_same: vec![NONE; timeperiod].into_boxed_slice(),
            head: 0,
            len: 0,
            bins_by_key: HashMap::with_capacity(timeperiod),
            bins: vec![ModeBin::default(); timeperiod].into_boxed_slice(),
            free_bins: (0..timeperiod).rev().collect(),
            heap: vec![0; timeperiod].into_boxed_slice(),
            heap_len: 0,
            sequence: 0,
            timeperiod,
            value: None,
        })
    }

    #[inline]
    fn higher_priority(&self, left: usize, right: usize) -> bool {
        let left_bin = &self.bins[left];
        let right_bin = &self.bins[right];
        left_bin.count > right_bin.count
            || (left_bin.count == right_bin.count
                && self.sequence.wrapping_sub(self.positions[left_bin.first])
                    > self.sequence.wrapping_sub(self.positions[right_bin.first]))
    }

    #[inline]
    fn swap_heap(&mut self, left: usize, right: usize) {
        self.heap.swap(left, right);
        self.bins[self.heap[left]].heap_index = left;
        self.bins[self.heap[right]].heap_index = right;
    }

    fn sift_up(&mut self, mut index: usize) {
        while index != 0 {
            let parent = (index - 1) / 2;
            if !self.higher_priority(self.heap[index], self.heap[parent]) {
                break;
            }
            self.swap_heap(index, parent);
            index = parent;
        }
    }

    fn sift_down(&mut self, mut index: usize) {
        loop {
            let left = index * 2 + 1;
            if left >= self.heap_len {
                break;
            }
            let right = left + 1;
            let child = if right < self.heap_len
                && self.higher_priority(self.heap[right], self.heap[left])
            {
                right
            } else {
                left
            };
            if !self.higher_priority(self.heap[child], self.heap[index]) {
                break;
            }
            self.swap_heap(index, child);
            index = child;
        }
    }

    #[inline]
    fn insert_heap(&mut self, bin_index: usize) {
        let index = self.heap_len;
        self.heap[index] = bin_index;
        self.heap_len += 1;
        self.bins[bin_index].heap_index = index;
        self.sift_up(index);
    }

    #[inline]
    fn remove_heap(&mut self, bin_index: usize) {
        let index = self.bins[bin_index].heap_index;
        self.heap_len -= 1;
        self.bins[bin_index].heap_index = NONE;
        if index == self.heap_len {
            return;
        }

        let replacement = self.heap[self.heap_len];
        self.heap[index] = replacement;
        self.bins[replacement].heap_index = index;
        if index != 0 && self.higher_priority(replacement, self.heap[(index - 1) / 2]) {
            self.sift_up(index);
        } else {
            self.sift_down(index);
        }
    }

    #[inline]
    fn remove_occurrence(&mut self, slot: usize, value: f64) {
        if value.is_nan() {
            return;
        }

        let key = count_key(value);
        let bin_index = *self.bins_by_key.get(&key).expect("evicted value counted");
        self.remove_heap(bin_index);
        let next = self.next_same[slot];
        let bin = &mut self.bins[bin_index];
        debug_assert_eq!(bin.first, slot);
        bin.count -= 1;
        if bin.count == 0 {
            self.bins_by_key.remove(&key);
            *bin = ModeBin::default();
            self.free_bins.push(bin_index);
        } else {
            bin.first = next;
            self.insert_heap(bin_index);
        }
    }

    #[inline]
    fn add_occurrence(&mut self, slot: usize, value: f64) {
        if value.is_nan() {
            return;
        }

        let key = count_key(value);
        if let Some(&bin_index) = self.bins_by_key.get(&key) {
            self.remove_heap(bin_index);
            let last = self.bins[bin_index].last;
            self.next_same[last] = slot;
            self.bins[bin_index].last = slot;
            self.bins[bin_index].count += 1;
            self.insert_heap(bin_index);
        } else {
            let bin_index = self.free_bins.pop().expect("one bin per window slot");
            self.bins[bin_index] = ModeBin {
                count: 1,
                first: slot,
                last: slot,
                heap_index: NONE,
            };
            self.bins_by_key.insert(key, bin_index);
            self.insert_heap(bin_index);
        }
    }

    /// Appends one observation and returns the mode after warm-up.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        let slot = self.head;
        if self.len == self.timeperiod {
            self.remove_occurrence(slot, self.ring[slot]);
        } else {
            self.len += 1;
        }

        self.ring[slot] = input;
        self.positions[slot] = self.sequence;
        self.sequence = self.sequence.wrapping_add(1);
        self.next_same[slot] = NONE;
        self.add_occurrence(slot, input);

        self.head += 1;
        if self.head == self.timeperiod {
            self.head = 0;
        }

        self.value = if self.len == self.timeperiod {
            if self.heap_len == 0 {
                Some(self.ring[self.head])
            } else {
                let bin = &self.bins[self.heap[0]];
                Some(self.ring[bin.first])
            }
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

    /// Clears the trailing window and latest output without reallocating.
    pub fn reset(&mut self) {
        self.head = 0;
        self.len = 0;
        self.bins_by_key.clear();
        self.free_bins.clear();
        self.free_bins.extend((0..self.timeperiod).rev());
        self.heap_len = 0;
        self.sequence = 0;
        self.value = None;
    }
}
