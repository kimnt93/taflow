use super::pattern_swing::{SwingTracker, SWING_THRESHOLD};
use crate::error::TaResult;

/// Sorted lower, midpoint, and upper prices of the golden pocket.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GoldenPocketValue {
    pub low: f64,
    pub mid: f64,
    pub high: f64,
}

/// The 61.8%-65% retracement band of the latest confirmed swing leg.
#[derive(Debug, Clone)]
pub struct GoldenPocket {
    swing: SwingTracker,
    count: usize,
    value: Option<GoldenPocketValue>,
}

impl GoldenPocket {
    /// Create an empty golden-pocket tracker.
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            swing: SwingTracker::new(SWING_THRESHOLD, 2),
            count: 0,
            value: None,
        })
    }

    /// Append one high/low bar and return the latest retracement band.
    pub fn append(&mut self, high: f64, low: f64) -> Option<GoldenPocketValue> {
        self.count += 1;
        self.swing.append(high, low);
        let pivots = self.swing.pivots();
        self.value = if pivots.len() < 2 {
            None
        } else {
            let span = pivots[0].price - pivots[1].price;
            let first = pivots[1].price + 0.618 * span;
            let second = pivots[1].price + 0.650 * span;
            let low = first.min(second);
            let high = first.max(second);
            Some(GoldenPocketValue {
                low,
                mid: f64::midpoint(low, high),
                high,
            })
        };
        self.value
    }

    /// Return the latest golden-pocket band.
    pub fn value(&self) -> Option<GoldenPocketValue> {
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
