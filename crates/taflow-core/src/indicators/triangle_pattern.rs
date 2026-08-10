use super::pattern_swing::{
    approximately_equal, recent_legs, SwingTracker, LEVEL_TOLERANCE, SWING_THRESHOLD,
};
use crate::error::TaResult;

/// Ascending, descending, and symmetrical triangle signal.
#[derive(Debug, Clone)]
pub struct TrianglePattern {
    swing: SwingTracker,
    count: usize,
    value: Option<f64>,
}

impl TrianglePattern {
    /// Create a detector with Wickra's fixed swing geometry.
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            swing: SwingTracker::new(SWING_THRESHOLD, 4),
            count: 0,
            value: None,
        })
    }
    /// Append one OHLC bar and return the latest directional signal.
    pub fn append(&mut self, _open: f64, high: f64, low: f64, _close: f64) -> Option<f64> {
        self.count += 1;
        self.value = Some(0.0);
        if !self.swing.append(high, low) || self.swing.pivots().len() < 4 {
            return self.value;
        }
        let pivots = self.swing.pivots();
        let (ho, hn, lo, ln) = recent_legs(pivots);
        let flat_highs = approximately_equal(ho, hn, LEVEL_TOLERANCE);
        let flat_lows = approximately_equal(lo, ln, LEVEL_TOLERANCE);
        let rising_lows = ln > lo * (1.0 + LEVEL_TOLERANCE);
        let falling_highs = hn < ho * (1.0 - LEVEL_TOLERANCE);
        self.value = if flat_highs && rising_lows {
            Some(1.0)
        } else if falling_highs && flat_lows {
            Some(-1.0)
        } else if falling_highs && rising_lows {
            Some(if pivots[pivots.len() - 1].direction > 0.0 {
                -1.0
            } else {
                1.0
            })
        } else {
            Some(0.0)
        };
        self.value
    }
    /// Return the signal for the latest bar.
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Return the number of processed bars.
    pub fn len(&self) -> usize {
        self.count
    }
    /// Return whether no bars were processed.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    /// Clear pivots and latest output.
    pub fn reset(&mut self) {
        self.swing.reset();
        self.count = 0;
        self.value = None;
    }
}
