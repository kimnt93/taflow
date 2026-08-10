use super::pattern_swing::{SwingTracker, SWING_THRESHOLD};
use crate::error::TaResult;

/// Fibonacci arc prices at the current bar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FibonacciArcsValue {
    pub arc_382: f64,
    pub arc_500: f64,
    pub arc_618: f64,
}

/// Semicircular retracement arcs from the latest confirmed swing leg.
#[derive(Debug, Clone)]
pub struct FibonacciArcs {
    swing: SwingTracker,
    count: usize,
    value: Option<FibonacciArcsValue>,
}

impl FibonacciArcs {
    /// Create an empty arc tracker.
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            swing: SwingTracker::new(SWING_THRESHOLD, 2),
            count: 0,
            value: None,
        })
    }

    /// Append one high/low bar and evaluate the three arcs.
    pub fn append(&mut self, high: f64, low: f64) -> Option<FibonacciArcsValue> {
        self.count += 1;
        self.swing.append(high, low);
        let pivots = self.swing.pivots();
        self.value = if pivots.len() < 2 {
            None
        } else {
            let start = pivots[0];
            let end = pivots[1];
            let elapsed = (self.swing.current_bar() - end.bar) as f64;
            let duration = (end.bar - start.bar) as f64;
            let curve = (1.0 - (elapsed / duration).powi(2)).max(0.0).sqrt();
            let arc = |ratio: f64| end.price + (start.price - end.price) * ratio * curve;
            Some(FibonacciArcsValue {
                arc_382: arc(0.382),
                arc_500: arc(0.500),
                arc_618: arc(0.618),
            })
        };
        self.value
    }

    /// Return the latest arc prices.
    pub fn value(&self) -> Option<FibonacciArcsValue> {
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
