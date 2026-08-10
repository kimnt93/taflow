use super::pattern_swing::{approximately_equal, SwingTracker, LEVEL_TOLERANCE, SWING_THRESHOLD};
use crate::error::TaResult;

/// Head-and-shoulders and inverse reversal signal from five pivots.
#[derive(Debug, Clone)]
pub struct HeadAndShoulders {
    swing: SwingTracker,
    count: usize,
    value: Option<f64>,
}

impl HeadAndShoulders {
    /// Create a detector with Wickra's fixed swing geometry.
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            swing: SwingTracker::new(SWING_THRESHOLD, 5),
            count: 0,
            value: None,
        })
    }
    /// Append one OHLC bar and return the latest directional signal.
    pub fn append(&mut self, _open: f64, high: f64, low: f64, _close: f64) -> Option<f64> {
        self.count += 1;
        self.value = Some(0.0);
        if !self.swing.append(high, low) || self.swing.pivots().len() < 5 {
            return self.value;
        }
        let p = self.swing.pivots();
        let n = p.len();
        let left = p[n - 5];
        let neck1 = p[n - 4];
        let head = p[n - 3];
        let neck2 = p[n - 2];
        let right = p[n - 1];
        let frame = approximately_equal(left.price, right.price, LEVEL_TOLERANCE)
            && approximately_equal(neck1.price, neck2.price, LEVEL_TOLERANCE);
        self.value = if right.direction > 0.0
            && frame
            && head.price > left.price
            && head.price > right.price
        {
            Some(-1.0)
        } else if right.direction < 0.0
            && frame
            && head.price < left.price
            && head.price < right.price
        {
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
