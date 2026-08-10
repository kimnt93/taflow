use super::pattern_swing::{SwingTracker, SWING_THRESHOLD};
use crate::error::TaResult;

/// Sloped Fibonacci channel lines at the current bar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FibonacciChannelValue {
    pub base: f64,
    pub level_618: f64,
    pub level_1000: f64,
    pub level_1618: f64,
}

/// Parallel Fibonacci channel from the three newest confirmed pivots.
#[derive(Debug, Clone)]
pub struct FibonacciChannel {
    swing: SwingTracker,
    count: usize,
    value: Option<FibonacciChannelValue>,
}

impl FibonacciChannel {
    /// Create an empty channel tracker.
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            swing: SwingTracker::new(SWING_THRESHOLD, 3),
            count: 0,
            value: None,
        })
    }

    /// Append one high/low bar and evaluate channel lines at the current bar.
    pub fn append(&mut self, high: f64, low: f64) -> Option<FibonacciChannelValue> {
        self.count += 1;
        self.swing.append(high, low);
        let pivots = self.swing.pivots();
        self.value = if pivots.len() < 3 {
            None
        } else {
            let first = pivots[0];
            let middle = pivots[1];
            let last = pivots[2];
            let slope = (last.price - first.price) / (last.bar - first.bar) as f64;
            let base_at = |bar: usize| first.price + slope * (bar - first.bar) as f64;
            let width = middle.price - base_at(middle.bar);
            let base = base_at(self.swing.current_bar());
            Some(FibonacciChannelValue {
                base,
                level_618: base + 0.618 * width,
                level_1000: base + width,
                level_1618: base + 1.618 * width,
            })
        };
        self.value
    }

    /// Return the latest channel lines.
    pub fn value(&self) -> Option<FibonacciChannelValue> {
        self.value
    }
    /// Return the processed-bar count.
    pub fn len(&self) -> usize {
        self.count
    }
    /// Return whether no bars were processed.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    /// Clear pivots and output.
    pub fn reset(&mut self) {
        self.swing.reset();
        self.count = 0;
        self.value = None;
    }
}
