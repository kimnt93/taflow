use super::pattern_swing::{ratios_in, xabcd, SwingTracker, SWING_THRESHOLD};
use crate::error::TaResult;

/// Gartley X-A-B-C-D harmonic completion signal.
#[derive(Debug, Clone)]
pub struct GartleyPattern {
    swing: SwingTracker,
    count: usize,
    value: Option<f64>,
}

impl GartleyPattern {
    /// Create a detector retaining five confirmed pivots.
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            swing: SwingTracker::new(SWING_THRESHOLD, 5),
            count: 0,
            value: None,
        })
    }

    /// Append one OHLC bar and return the latest harmonic signal.
    pub fn append(&mut self, _open: f64, high: f64, low: f64, _close: f64) -> Option<f64> {
        self.count += 1;
        self.value = Some(0.0);
        if !self.swing.append(high, low) || self.swing.pivots().len() < 5 {
            return self.value;
        }

        let points = xabcd(self.swing.pivots());
        let xa = (points.a - points.x).abs();
        let ab = (points.b - points.a).abs();
        let bc = (points.c - points.b).abs();
        let cd = (points.d - points.c).abs();
        let ad = (points.d - points.a).abs();
        if ratios_in(&[
            (ab / xa, 0.550, 0.700),
            (bc / ab, 0.382, 0.886),
            (cd / bc, 1.130, 1.618),
            (ad / xa, 0.740, 0.840),
        ]) {
            self.value = Some(if points.bullish { 1.0 } else { -1.0 });
        }
        self.value
    }

    /// Return the latest signal.
    pub fn value(&self) -> Option<f64> {
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
