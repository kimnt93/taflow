use super::pattern_swing::{SwingTracker, SWING_THRESHOLD};
use crate::error::TaResult;

/// Continuation targets for the latest confirmed swing leg.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FibonacciExtensionValue {
    pub extension_1272: f64,
    pub extension_1414: f64,
    pub extension_1618: f64,
    pub extension_2000: f64,
    pub extension_2618: f64,
}

/// Extend the latest confirmed leg at canonical Fibonacci multiples.
#[derive(Debug, Clone)]
pub struct FibonacciExtension {
    swing: SwingTracker,
    count: usize,
    value: Option<FibonacciExtensionValue>,
}

impl FibonacciExtension {
    /// Create an empty tracker retaining two confirmed pivots.
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            swing: SwingTracker::new(SWING_THRESHOLD, 2),
            count: 0,
            value: None,
        })
    }

    /// Append one high/low bar and return continuation targets for the latest leg.
    pub fn append(&mut self, high: f64, low: f64) -> Option<FibonacciExtensionValue> {
        self.count += 1;
        self.swing.append(high, low);
        let pivots = self.swing.pivots();
        self.value = if pivots.len() < 2 {
            None
        } else {
            let (start, end) = (pivots[0].price, pivots[1].price);
            let extend = |ratio: f64| start + ratio * (end - start);
            Some(FibonacciExtensionValue {
                extension_1272: extend(1.272),
                extension_1414: extend(1.414),
                extension_1618: extend(1.618),
                extension_2000: extend(2.0),
                extension_2618: extend(2.618),
            })
        };
        self.value
    }

    /// Return the latest targets, or `None` before two pivots confirm.
    pub fn value(&self) -> Option<FibonacciExtensionValue> {
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
