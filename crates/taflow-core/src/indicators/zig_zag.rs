use crate::error::{TaError, TaResult};

/// A confirmed ZigZag swing and its direction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZigZagValue {
    /// Price of the confirmed running extreme.
    pub swing: f64,
    /// `1.0` for a swing high and `-1.0` for a swing low.
    pub direction: f64,
}

#[derive(Debug, Clone, Copy)]
struct ZigZagState {
    direction: f64,
    extreme: f64,
}

/// Non-repainting percentage-threshold swing detector.
#[derive(Debug, Clone)]
pub struct ZigZag {
    threshold: f64,
    state: Option<ZigZagState>,
    value: Option<ZigZagValue>,
}

impl ZigZag {
    /// Create a detector for reversals strictly between zero and 100 percent.
    pub fn new(threshold: f64) -> TaResult<Self> {
        if !threshold.is_finite() || threshold <= 0.0 || threshold >= 1.0 {
            return Err(TaError::InvalidParameter {
                name: "threshold",
                value: threshold.to_string(),
                reason: "must be a finite fraction in (0, 1)",
            });
        }
        Ok(Self {
            threshold,
            state: None,
            value: None,
        })
    }

    /// Append one high/low bar and emit only when a swing is confirmed.
    pub fn append(&mut self, high: f64, low: f64) -> Option<ZigZagValue> {
        let Some(state) = self.state else {
            self.state = Some(ZigZagState {
                direction: 1.0,
                extreme: high,
            });
            self.value = None;
            return None;
        };

        self.value = if state.direction > 0.0 {
            if high > state.extreme {
                self.state = Some(ZigZagState {
                    direction: 1.0,
                    extreme: high,
                });
                None
            } else if low <= state.extreme * (1.0 - self.threshold) {
                self.state = Some(ZigZagState {
                    direction: -1.0,
                    extreme: low,
                });
                Some(ZigZagValue {
                    swing: state.extreme,
                    direction: 1.0,
                })
            } else {
                None
            }
        } else if low < state.extreme {
            self.state = Some(ZigZagState {
                direction: -1.0,
                extreme: low,
            });
            None
        } else if high >= state.extreme * (1.0 + self.threshold) {
            self.state = Some(ZigZagState {
                direction: 1.0,
                extreme: high,
            });
            Some(ZigZagValue {
                swing: state.extreme,
                direction: -1.0,
            })
        } else {
            None
        };
        self.value
    }

    /// Return the swing emitted by the latest bar, if any.
    pub fn value(&self) -> Option<ZigZagValue> {
        self.value
    }

    /// Reset the running extreme and latest confirmation.
    pub fn reset(&mut self) {
        self.state = None;
        self.value = None;
    }
}
