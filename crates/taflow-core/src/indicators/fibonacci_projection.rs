use super::pattern_swing::{SwingTracker, SWING_THRESHOLD};
use crate::error::TaResult;

/// Measured-move targets projected from the last three confirmed pivots.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FibonacciProjectionValue {
    pub projection_618: f64,
    pub projection_1000: f64,
    pub projection_1618: f64,
    pub projection_2618: f64,
}

/// Project the A-to-B leg from pivot C at canonical Fibonacci ratios.
#[derive(Debug, Clone)]
pub struct FibonacciProjection {
    swing: SwingTracker,
    count: usize,
    value: Option<FibonacciProjectionValue>,
}

impl FibonacciProjection {
    /// Create an empty tracker retaining three confirmed pivots.
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            swing: SwingTracker::new(SWING_THRESHOLD, 3),
            count: 0,
            value: None,
        })
    }

    /// Append one high/low bar and return measured-move targets after pivot C.
    pub fn append(&mut self, high: f64, low: f64) -> Option<FibonacciProjectionValue> {
        self.count += 1;
        self.swing.append(high, low);
        let pivots = self.swing.pivots();
        self.value = if pivots.len() < 3 {
            None
        } else {
            let (a, b, c) = (pivots[0].price, pivots[1].price, pivots[2].price);
            let project = |ratio: f64| c + ratio * (b - a);
            Some(FibonacciProjectionValue {
                projection_618: project(0.618),
                projection_1000: project(1.0),
                projection_1618: project(1.618),
                projection_2618: project(2.618),
            })
        };
        self.value
    }

    /// Return the latest targets, or `None` before three pivots confirm.
    pub fn value(&self) -> Option<FibonacciProjectionValue> {
        self.value
    }

    /// Return the number of processed bars.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Return whether no bars have been processed.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Clear all pivots, history count, and the latest value.
    pub fn reset(&mut self) {
        self.swing.reset();
        self.count = 0;
        self.value = None;
    }
}
