use super::pattern_swing::{recent_legs, SwingTracker, SWING_THRESHOLD};
use crate::error::TaResult;

/// Rising/falling wedge reversal signal from four confirmed pivots.
#[derive(Debug, Clone)]
pub struct WedgePattern {
    swing: SwingTracker,
    count: usize,
    value: Option<f64>,
}

impl WedgePattern {
    /// Create a detector with Wickra's fixed 5% swing threshold.
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            swing: SwingTracker::new(SWING_THRESHOLD, 4),
            count: 0,
            value: None,
        })
    }
    /// Append one OHLC bar and return `1`, `-1`, or `0`.
    pub fn append(&mut self, _open: f64, high: f64, low: f64, _close: f64) -> Option<f64> {
        self.count += 1;
        self.value = Some(0.0);
        if !self.swing.append(high, low) || self.swing.pivots().len() < 4 {
            return self.value;
        }
        let (old_high, new_high, old_low, new_low) = recent_legs(self.swing.pivots());
        let high_slope = new_high - old_high;
        let low_slope = new_low - old_low;
        self.value = if high_slope > 0.0 && low_slope > high_slope {
            Some(-1.0)
        } else if high_slope < low_slope && low_slope < 0.0 {
            Some(1.0)
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
