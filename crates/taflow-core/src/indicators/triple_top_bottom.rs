use super::pattern_swing::{approximately_equal, SwingTracker, LEVEL_TOLERANCE, SWING_THRESHOLD};
use crate::error::TaResult;

/// Triple-top and triple-bottom reversal signal from five pivots.
#[derive(Debug, Clone)]
pub struct TripleTopBottom {
    swing: SwingTracker,
    count: usize,
    value: Option<f64>,
}

impl TripleTopBottom {
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
        if approximately_equal(p[n - 5].price, p[n - 3].price, LEVEL_TOLERANCE)
            && approximately_equal(p[n - 3].price, p[n - 1].price, LEVEL_TOLERANCE)
        {
            self.value = Some(if p[n - 1].direction > 0.0 { -1.0 } else { 1.0 });
        }
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
