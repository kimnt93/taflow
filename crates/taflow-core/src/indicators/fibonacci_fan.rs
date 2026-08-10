use super::pattern_swing::{SwingTracker, SWING_THRESHOLD};
use crate::error::TaResult;

/// Fibonacci fan-line prices at the current bar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FibonacciFanValue {
    pub fan_382: f64,
    pub fan_500: f64,
    pub fan_618: f64,
}

/// Fan lines from the latest confirmed swing leg.
#[derive(Debug, Clone)]
pub struct FibonacciFan {
    swing: SwingTracker,
    count: usize,
    value: Option<FibonacciFanValue>,
}

impl FibonacciFan {
    /// Create an empty fan tracker.
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            swing: SwingTracker::new(SWING_THRESHOLD, 2),
            count: 0,
            value: None,
        })
    }

    /// Append one high/low bar and evaluate three fan lines.
    pub fn append(&mut self, high: f64, low: f64) -> Option<FibonacciFanValue> {
        self.count += 1;
        self.swing.append(high, low);
        let pivots = self.swing.pivots();
        self.value = if pivots.len() < 2 {
            None
        } else {
            let start = pivots[0];
            let end = pivots[1];
            let progress =
                (self.swing.current_bar() - start.bar) as f64 / (end.bar - start.bar) as f64;
            let line = |ratio: f64| start.price + ratio * (end.price - start.price) * progress;
            Some(FibonacciFanValue {
                fan_382: line(0.382),
                fan_500: line(0.500),
                fan_618: line(0.618),
            })
        };
        self.value
    }

    /// Return the latest fan-line prices.
    pub fn value(&self) -> Option<FibonacciFanValue> {
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
