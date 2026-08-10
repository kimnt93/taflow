use super::pattern_swing::{SwingTracker, SWING_THRESHOLD};
use crate::error::TaResult;

/// Current Fibonacci time-zone status and distance to the next zone.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FibonacciTimeZonesValue {
    pub on_zone: f64,
    pub bars_to_next: f64,
}

/// Fibonacci bar-distance zones anchored to the latest confirmed pivot.
#[derive(Debug, Clone)]
pub struct FibonacciTimeZones {
    swing: SwingTracker,
    count: usize,
    value: Option<FibonacciTimeZonesValue>,
}

impl FibonacciTimeZones {
    /// Create an empty time-zone tracker.
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            swing: SwingTracker::new(SWING_THRESHOLD, 2),
            count: 0,
            value: None,
        })
    }

    /// Append one high/low bar and return zone information after the first pivot.
    pub fn append(&mut self, high: f64, low: f64) -> Option<FibonacciTimeZonesValue> {
        self.count += 1;
        self.swing.append(high, low);
        self.value = self.swing.pivots().last().map(|anchor| {
            let distance = self.swing.current_bar() - anchor.bar;
            let (mut previous, mut next) = (1_usize, 2_usize);
            let mut on_zone = false;
            while previous <= distance {
                on_zone |= previous == distance;
                (previous, next) = (next, previous + next);
            }
            FibonacciTimeZonesValue {
                on_zone: f64::from(u8::from(on_zone)),
                bars_to_next: (previous - distance) as f64,
            }
        });
        self.value
    }

    /// Return the latest zone information.
    pub fn value(&self) -> Option<FibonacciTimeZonesValue> {
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
