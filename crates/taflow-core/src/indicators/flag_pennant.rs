use super::pattern_swing::{SwingTracker, SWING_THRESHOLD};
use crate::error::TaResult;

/// Causal flag/pennant continuation signal from three confirmed pivots.
#[derive(Debug, Clone)]
pub struct FlagPennant {
    swing: SwingTracker,
    count: usize,
    value: Option<f64>,
}

impl FlagPennant {
    /// Create a detector with Wickra's fixed 5% swing threshold.
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            swing: SwingTracker::new(SWING_THRESHOLD, 3),
            count: 0,
            value: None,
        })
    }
    /// Append one OHLC bar and return `1`, `-1`, or `0`.
    pub fn append(&mut self, _open: f64, high: f64, low: f64, _close: f64) -> Option<f64> {
        self.count += 1;
        self.value = Some(0.0);
        if !self.swing.append(high, low) {
            return self.value;
        }
        let pivots = self.swing.pivots();
        if pivots.len() < 3 {
            return self.value;
        }
        let n = pivots.len();
        let pole = (pivots[n - 2].price - pivots[n - 3].price).abs();
        let pullback = (pivots[n - 1].price - pivots[n - 2].price).abs();
        if pole > 0.0 && pullback < 0.5 * pole {
            self.value = Some(if pivots[n - 2].direction > 0.0 {
                1.0
            } else {
                -1.0
            });
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
