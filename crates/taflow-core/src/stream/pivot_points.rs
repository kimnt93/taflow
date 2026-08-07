//! Stateful classic session pivot levels.

/// Session high/low/close with anchor-triggered pivot, resistance, and support levels.
#[derive(Debug, Clone)]
pub struct PivotPoints {
    high: Option<f64>,
    low: Option<f64>,
    close: Option<f64>,
    levels: (f64, f64, f64, f64, f64),
}

impl PivotPoints {
    /// Creates an empty pivot-level state.
    pub fn new() -> Self { Self { high: None, low: None, close: None, levels: (f64::NAN, f64::NAN, f64::NAN, f64::NAN, f64::NAN) } }

    /// Appends one OHLC bar and optionally rolls to a new session.
    pub fn append(&mut self, high: f64, low: f64, close: f64, anchor: bool) -> (f64, f64, f64, f64, f64) {
        if anchor {
            if let (Some(previous_high), Some(previous_low), Some(previous_close)) = (self.high, self.low, self.close) {
                let pivot = (previous_high + previous_low + previous_close) / 3.0;
                let range = previous_high - previous_low;
                self.levels = (pivot, 2.0 * pivot - previous_low, 2.0 * pivot - previous_high, pivot - range, pivot + range);
            }
            self.high = Some(high); self.low = Some(low); self.close = Some(close);
        } else {
            self.high = Some(self.high.map_or(high, |value| value.max(high)));
            self.low = Some(self.low.map_or(low, |value| value.min(low)));
            self.close = Some(close);
        }
        self.levels
    }

    /// Returns the latest five pivot levels.
    pub fn value(&self) -> (f64, f64, f64, f64, f64) { self.levels }
    /// Clears current session and levels.
    pub fn reset(&mut self) { self.high = None; self.low = None; self.close = None; self.levels = (f64::NAN, f64::NAN, f64::NAN, f64::NAN, f64::NAN); }
}
