//! Stateful opening range and breakout flags.

/// Opening range high/low and current breakout direction.
#[derive(Debug, Clone)]
pub struct OpeningRange {
    bars: usize,
    count: usize,
    high: f64,
    low: f64,
    value: Option<(f64, f64, i32)>,
}

impl OpeningRange {
    /// Creates an opening range lasting `bars` observations.
    pub fn new(bars: usize) -> Self {
        Self { bars, count: 0, high: f64::NEG_INFINITY, low: f64::INFINITY, value: None }
    }

    /// Appends a bar, optionally beginning a new session.
    pub fn append(&mut self, high: f64, low: f64, close: f64, anchor: bool) -> (f64, f64, i32) {
        if anchor { self.count = 0; self.high = f64::NEG_INFINITY; self.low = f64::INFINITY; }
        if self.count < self.bars { self.high = self.high.max(high); self.low = self.low.min(low); self.count += 1; }
        let breakout = if close > self.high { 1 } else if close < self.low { -1 } else { 0 };
        self.value = Some((self.high, self.low, breakout));
        (self.high, self.low, breakout)
    }

    /// Returns the latest opening range values.
    pub fn value(&self) -> Option<(f64, f64, i32)> { self.value }
    /// Clears the current session and output.
    pub fn reset(&mut self) { self.count = 0; self.high = f64::NEG_INFINITY; self.low = f64::INFINITY; self.value = None; }
}
