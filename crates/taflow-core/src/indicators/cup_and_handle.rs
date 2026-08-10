use crate::error::TaResult;

use super::pattern_swing::{approximately_equal, SwingTracker, LEVEL_TOLERANCE, SWING_THRESHOLD};

/// Causal cup-and-handle and inverse-cup-and-handle detector.
#[derive(Debug, Clone)]
pub struct CupAndHandle {
    swing: SwingTracker,
    count: usize,
    value: Option<f64>,
}

impl CupAndHandle {
    /// Create a detector using Wickra's fixed 5% swing threshold.
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            swing: SwingTracker::new(SWING_THRESHOLD, 4),
            count: 0,
            value: None,
        })
    }

    /// Append one OHLC bar and return `1`, `-1`, or `0` from the first bar.
    pub fn append(&mut self, _open: f64, high: f64, low: f64, _close: f64) -> Option<f64> {
        self.count += 1;
        self.value = Some(0.0);
        if !self.swing.append(high, low) {
            return self.value;
        }
        let pivots = self.swing.pivots();
        if pivots.len() < 4 {
            return self.value;
        }

        let length = pivots.len();
        let left_rim = pivots[length - 4];
        let extreme = pivots[length - 3];
        let right_rim = pivots[length - 2];
        let handle = pivots[length - 1];
        let rims_match = approximately_equal(left_rim.price, right_rim.price, LEVEL_TOLERANCE);
        self.value = if handle.direction < 0.0
            && rims_match
            && handle.price > extreme.price
            && handle.price < right_rim.price
        {
            Some(1.0)
        } else if handle.direction > 0.0
            && rims_match
            && handle.price < extreme.price
            && handle.price > right_rim.price
        {
            Some(-1.0)
        } else {
            Some(0.0)
        };
        self.value
    }

    /// Return the signal produced for the latest bar.
    pub fn value(&self) -> Option<f64> {
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

    /// Clear pivots and restore fresh-state behaviour.
    pub fn reset(&mut self) {
        self.swing.reset();
        self.count = 0;
        self.value = None;
    }
}
