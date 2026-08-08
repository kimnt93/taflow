//! Fixed-capacity storage shared by bounded-window indicators.

use crate::error::TaResult;

use super::invalid_period;

/// A fixed-capacity FIFO backed by a ring buffer.
///
/// Allocates exactly once at construction. `push` on a full window overwrites
/// the oldest slot and returns the evicted value — no reallocation, no
/// per-push capacity checks beyond a predictable wrap branch.
#[derive(Debug, Clone)]
pub struct Window {
    buf: Box<[f64]>,
    /// Index of the oldest element.
    head: usize,
    len: usize,
}

impl Window {
    pub fn new(capacity: usize) -> TaResult<Self> {
        if capacity == 0 {
            return Err(invalid_period("capacity", capacity, 1));
        }
        Ok(Self {
            buf: vec![0.0; capacity].into_boxed_slice(),
            head: 0,
            len: 0,
        })
    }

    /// Appends `value`, returning the value evicted from a full window.
    #[inline]
    pub fn push(&mut self, value: f64) -> Option<f64> {
        let cap = self.buf.len();
        if self.len == cap {
            let evicted = self.buf[self.head];
            self.buf[self.head] = value;
            self.head += 1;
            if self.head == cap {
                self.head = 0;
            }
            Some(evicted)
        } else {
            let mut tail = self.head + self.len;
            if tail >= cap {
                tail -= cap;
            }
            self.buf[tail] = value;
            self.len += 1;
            None
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.len == self.buf.len()
    }

    /// The window contents as (older, newer) contiguous slices.
    #[inline]
    pub fn as_slices(&self) -> (&[f64], &[f64]) {
        let cap = self.buf.len();
        let first_len = (cap - self.head).min(self.len);
        let front = &self.buf[self.head..self.head + first_len];
        let back = &self.buf[..self.len - first_len];
        (front, back)
    }

    /// Iterates oldest → newest.
    #[inline]
    pub fn iter(&self) -> WindowIter<'_> {
        let (front, back) = self.as_slices();
        WindowIter {
            front: front.iter(),
            back: back.iter(),
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.head = 0;
        self.len = 0;
    }
}

/// Oldest-to-newest iterator over a [`Window`].
#[derive(Debug, Clone)]
pub struct WindowIter<'a> {
    front: std::slice::Iter<'a, f64>,
    back: std::slice::Iter<'a, f64>,
}

impl<'a> Iterator for WindowIter<'a> {
    type Item = &'a f64;

    #[inline]
    fn next(&mut self) -> Option<&'a f64> {
        self.front.next().or_else(|| self.back.next())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.front.len() + self.back.len();
        (len, Some(len))
    }
}

impl DoubleEndedIterator for WindowIter<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.back.next_back().or_else(|| self.front.next_back())
    }
}

impl ExactSizeIterator for WindowIter<'_> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_matches_fifo_semantics() {
        let mut w = Window::new(3).unwrap();
        assert_eq!(w.push(1.0), None);
        assert_eq!(w.push(2.0), None);
        assert!(!w.is_full());
        assert_eq!(w.push(3.0), None);
        assert!(w.is_full());
        assert_eq!(w.push(4.0), Some(1.0));
        assert_eq!(w.push(5.0), Some(2.0));
        assert_eq!(w.len(), 3);
        let values: Vec<f64> = w.iter().copied().collect();
        assert_eq!(values, vec![3.0, 4.0, 5.0]);
        let rev: Vec<f64> = w.iter().rev().copied().collect();
        assert_eq!(rev, vec![5.0, 4.0, 3.0]);
        assert_eq!(w.iter().len(), 3);
        let (a, b) = w.as_slices();
        let joined: Vec<f64> = a.iter().chain(b).copied().collect();
        assert_eq!(joined, vec![3.0, 4.0, 5.0]);
        w.clear();
        assert_eq!(w.len(), 0);
        assert_eq!(w.push(9.0), None);
        assert_eq!(w.iter().copied().collect::<Vec<_>>(), vec![9.0]);
    }

    #[test]
    fn wraparound_slices() {
        let mut w = Window::new(4).unwrap();
        for v in 1..=6 {
            w.push(v as f64);
        }
        // Contents: 3,4,5,6 with head in the middle of the buffer.
        let values: Vec<f64> = w.iter().copied().collect();
        assert_eq!(values, vec![3.0, 4.0, 5.0, 6.0]);
    }
}
