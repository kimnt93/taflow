use super::pattern_swing::{SwingTracker, SWING_THRESHOLD};
use crate::error::TaResult;

/// Seven retracement prices for the dominant recent confirmed swing leg.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutomaticFibonacciValue {
    pub level_000: f64,
    pub level_236: f64,
    pub level_382: f64,
    pub level_500: f64,
    pub level_618: f64,
    pub level_786: f64,
    pub level_100: f64,
}

/// Automatically anchor Fibonacci retracements to the largest of five recent legs.
#[derive(Debug, Clone)]
pub struct AutomaticFibonacci {
    swing: SwingTracker,
    count: usize,
    value: Option<AutomaticFibonacciValue>,
}

impl AutomaticFibonacci {
    /// Create an empty tracker retaining at most six confirmed pivots.
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            swing: SwingTracker::new(SWING_THRESHOLD, 6),
            count: 0,
            value: None,
        })
    }

    /// Append one high/low bar and return levels once a complete leg exists.
    pub fn append(&mut self, high: f64, low: f64) -> Option<AutomaticFibonacciValue> {
        self.count += 1;
        self.swing.append(high, low);

        let mut dominant = None;
        let mut dominant_size = f64::NEG_INFINITY;
        for leg in self.swing.pivots().windows(2) {
            let size = (leg[0].price - leg[1].price).abs();
            // Match Wickra's max-by behavior: the later leg wins an exact tie.
            if size >= dominant_size {
                dominant = Some((leg[0].price, leg[1].price));
                dominant_size = size;
            }
        }
        self.value = dominant.map(|(start, end)| {
            let level = |ratio: f64| end + ratio * (start - end);
            AutomaticFibonacciValue {
                level_000: level(0.0),
                level_236: level(0.236),
                level_382: level(0.382),
                level_500: level(0.500),
                level_618: level(0.618),
                level_786: level(0.786),
                level_100: level(1.0),
            }
        });
        self.value
    }

    /// Return the latest levels, or `None` before two pivots confirm.
    pub fn value(&self) -> Option<AutomaticFibonacciValue> {
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
