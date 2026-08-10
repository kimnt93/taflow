use super::pattern_swing::{approximately_equal, ratios_in, SwingTracker, SWING_THRESHOLD};
use crate::error::TaResult;

/// Four-point AB=CD harmonic completion signal.
#[derive(Debug, Clone)]
pub struct FourPointHarmonicPattern {
    swing: SwingTracker,
    count: usize,
    value: Option<f64>,
}

impl FourPointHarmonicPattern {
    /// Create a detector retaining four confirmed pivots.
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            swing: SwingTracker::new(SWING_THRESHOLD, 4),
            count: 0,
            value: None,
        })
    }

    /// Append one OHLC bar and return the latest harmonic signal.
    pub fn append(&mut self, _open: f64, high: f64, low: f64, _close: f64) -> Option<f64> {
        self.count += 1;
        self.value = Some(0.0);
        if !self.swing.append(high, low) || self.swing.pivots().len() < 4 {
            return self.value;
        }

        let pivots = self.swing.pivots();
        let length = pivots.len();
        let ab = (pivots[length - 3].price - pivots[length - 4].price).abs();
        let bc = (pivots[length - 2].price - pivots[length - 3].price).abs();
        let cd = (pivots[length - 1].price - pivots[length - 2].price).abs();
        if ratios_in(&[(bc / ab, 0.382, 0.886), (cd / bc, 1.130, 2.618)])
            && approximately_equal(ab, cd, 0.10)
        {
            self.value = Some(if pivots[length - 1].direction < 0.0 {
                1.0
            } else {
                -1.0
            });
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
