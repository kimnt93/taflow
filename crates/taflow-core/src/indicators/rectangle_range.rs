use crate::error::TaResult;

use super::pattern_swing::{
    approximately_equal, recent_legs, SwingTracker, LEVEL_TOLERANCE, SWING_THRESHOLD,
};

/// Causal horizontal support-and-resistance range detector.
#[derive(Debug, Clone)]
pub struct RectangleRange {
    swing: SwingTracker,
    count: usize,
    value: Option<f64>,
}

impl RectangleRange {
    /// Create a detector using Wickra's fixed swing and level tolerances.
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            swing: SwingTracker::new(SWING_THRESHOLD, 4),
            count: 0,
            value: None,
        })
    }

    /// Append one OHLC bar and return the latest range-touch signal.
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

        let (old_high, new_high, old_low, new_low) = recent_legs(pivots);
        let flat_highs = approximately_equal(old_high, new_high, LEVEL_TOLERANCE);
        let flat_lows = approximately_equal(old_low, new_low, LEVEL_TOLERANCE);
        if flat_highs && flat_lows {
            let last_is_high = pivots[pivots.len() - 1].direction > 0.0;
            self.value = Some(if last_is_high { -1.0 } else { 1.0 });
        }
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
