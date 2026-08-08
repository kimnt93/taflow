//! Monotonic-queue implementations for sliding-window extrema.
//!
//! These helpers compute fixed-width maxima/minima and their indices in O(n),
//! replacing the O(n * period) rescan used by naive rolling implementations.

use std::collections::VecDeque;

/// Sliding-window maximum and its index (monotonically decreasing queue).
///
/// Returns `(max_value, max_index)` for each complete window.
pub fn sliding_max(data: &[f64], window: usize) -> Vec<(f64, usize)> {
    if window == 0 || data.is_empty() {
        return Vec::new();
    }
    let len = data.len();
    let mut result = Vec::with_capacity(len.saturating_sub(window - 1));
    let mut deque: VecDeque<usize> = VecDeque::with_capacity(window);

    for i in 0..len {
        // Remove indices that have fallen out of the window.
        while let Some(&front) = deque.front() {
            if front + window <= i {
                deque.pop_front();
            } else {
                break;
            }
        }
        // Keep the queue decreasing; newest equal values win ties.
        while let Some(&back) = deque.back() {
            if data[back] <= data[i] {
                deque.pop_back();
            } else {
                break;
            }
        }
        deque.push_back(i);

        if i >= window - 1 {
            let max_idx = *deque.front().unwrap();
            result.push((data[max_idx], max_idx));
        }
    }

    result
}

/// Sliding-window minimum and its index (monotonically increasing queue).
pub fn sliding_min(data: &[f64], window: usize) -> Vec<(f64, usize)> {
    if window == 0 || data.is_empty() {
        return Vec::new();
    }
    let len = data.len();
    let mut result = Vec::with_capacity(len.saturating_sub(window - 1));
    let mut deque: VecDeque<usize> = VecDeque::with_capacity(window);

    for i in 0..len {
        while let Some(&front) = deque.front() {
            if front + window <= i {
                deque.pop_front();
            } else {
                break;
            }
        }
        while let Some(&back) = deque.back() {
            if data[back] >= data[i] {
                deque.pop_back();
            } else {
                break;
            }
        }
        deque.push_back(i);

        if i >= window - 1 {
            let min_idx = *deque.front().unwrap();
            result.push((data[min_idx], min_idx));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sliding_max() {
        let data = vec![1.0, 3.0, 2.0, 5.0, 4.0, 1.0];
        let result = sliding_max(&data, 3);
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], (3.0, 1)); // [1,3,2] -> max=3 at 1
        assert_eq!(result[1], (5.0, 3)); // [3,2,5] -> max=5 at 3
        assert_eq!(result[2], (5.0, 3)); // [2,5,4] -> max=5 at 3
        assert_eq!(result[3], (5.0, 3)); // [5,4,1] -> max=5 at 3
    }

    #[test]
    fn test_sliding_min() {
        let data = vec![5.0, 3.0, 4.0, 1.0, 2.0, 6.0];
        let result = sliding_min(&data, 3);
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], (3.0, 1)); // [5,3,4] -> min=3 at 1
        assert_eq!(result[1], (1.0, 3)); // [3,4,1] -> min=1 at 3
        assert_eq!(result[2], (1.0, 3)); // [4,1,2] -> min=1 at 3
        assert_eq!(result[3], (1.0, 3)); // [1,2,6] -> min=1 at 3
    }

    #[test]
    fn empty_or_zero_width_windows_are_empty() {
        assert!(sliding_max(&[1.0, 2.0], 0).is_empty());
        assert!(sliding_min(&[1.0, 2.0], 0).is_empty());
        assert!(sliding_max(&[], 3).is_empty());
        assert!(sliding_min(&[], 3).is_empty());
    }
}
