//! Stateful opening range and breakout flags.

/// Opening range high/low and current breakout direction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OpeningRangeValue {
    pub high: f64,
    pub low: f64,
    pub breakout: i32,
}

/// Persistent opening-range state.
#[derive(Debug, Clone)]
pub struct OpeningRange {
    bars: usize,
    count: usize,
    high: f64,
    low: f64,
    value: Option<OpeningRangeValue>,
}

impl OpeningRange {
    /// Creates an opening range lasting `bars` observations.
    pub fn new(bars: usize) -> Self {
        Self {
            bars,
            count: 0,
            high: f64::NEG_INFINITY,
            low: f64::INFINITY,
            value: None,
        }
    }

    /// Appends a bar, optionally beginning a new session.
    pub fn append(&mut self, high: f64, low: f64, close: f64, anchor: bool) -> OpeningRangeValue {
        if anchor {
            self.count = 0;
            self.high = f64::NEG_INFINITY;
            self.low = f64::INFINITY;
        }
        if self.count < self.bars {
            self.high = self.high.max(high);
            self.low = self.low.min(low);
            self.count += 1;
        }
        let breakout = if close > self.high {
            1
        } else if close < self.low {
            -1
        } else {
            0
        };
        let value = OpeningRangeValue {
            high: self.high,
            low: self.low,
            breakout,
        };
        self.value = Some(value);
        value
    }

    pub fn extend_slice_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        anchor: &[bool],
        output_high: &mut Vec<f64>,
        output_low: &mut Vec<f64>,
        breakout: &mut Vec<i32>,
    ) -> Result<(), crate::error::TaError> {
        if high.len() != low.len() || high.len() != close.len() || high.len() != anchor.len() {
            return Err(crate::error::TaError::LengthMismatch {
                expected: high.len(),
                got: low.len().max(close.len()).max(anchor.len()),
            });
        }
        for (((&high, &low), &close), &anchor) in high.iter().zip(low).zip(close).zip(anchor) {
            let value = self.append(high, low, close, anchor);
            output_high.push(value.high);
            output_low.push(value.low);
            breakout.push(value.breakout);
        }
        Ok(())
    }

    /// Returns the latest opening range values.
    pub fn value(&self) -> Option<OpeningRangeValue> {
        self.value
    }
    /// Clears the current session and output.
    pub fn reset(&mut self) {
        self.count = 0;
        self.high = f64::NEG_INFINITY;
        self.low = f64::INFINITY;
        self.value = None;
    }
}
