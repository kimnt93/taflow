use super::pattern_swing::{approximately_equal, ratios_in, xabcd, SwingTracker, SWING_THRESHOLD};
use crate::error::TaResult;

/// Three-Drives harmonic completion signal.
#[derive(Debug, Clone)]
pub struct ThreeDrives {
    swing: SwingTracker,
    count: usize,
    value: Option<f64>,
}

impl ThreeDrives {
    /// Create a detector retaining the five newest confirmed pivots.
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
        let p = xabcd(self.swing.pivots());
        let xa = (p.a - p.x).abs();
        let ab = (p.b - p.a).abs();
        let bc = (p.c - p.b).abs();
        let cd = (p.d - p.c).abs();
        if ratios_in(&[(ab / xa, 1.13, 1.75), (cd / bc, 1.13, 1.75)])
            && approximately_equal(ab, cd, 0.20)
            && approximately_equal(xa, bc, 0.30)
        {
            self.value = Some(if p.bullish { 1.0 } else { -1.0 });
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
