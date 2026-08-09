//! Stateful classic session pivot levels.

/// Session high/low/close with anchor-triggered pivot, resistance, and support levels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PivotPointsValue {
    pub pivot: f64,
    pub resistance_one: f64,
    pub support_one: f64,
    pub support_two: f64,
    pub resistance_two: f64,
}

#[derive(Debug, Clone)]
pub struct PivotPoints {
    high: Option<f64>,
    low: Option<f64>,
    close: Option<f64>,
    levels: PivotPointsValue,
    value: Option<PivotPointsValue>,
}

impl PivotPoints {
    /// Creates an empty pivot-level state.
    pub fn new() -> Self {
        Self {
            high: None,
            low: None,
            close: None,
            levels: PivotPointsValue {
                pivot: f64::NAN,
                resistance_one: f64::NAN,
                support_one: f64::NAN,
                support_two: f64::NAN,
                resistance_two: f64::NAN,
            },
            value: None,
        }
    }

    /// Appends one OHLC bar and optionally rolls to a new session.
    pub fn append(&mut self, high: f64, low: f64, close: f64, anchor: bool) -> PivotPointsValue {
        if anchor {
            if let (Some(previous_high), Some(previous_low), Some(previous_close)) =
                (self.high, self.low, self.close)
            {
                let pivot = (previous_high + previous_low + previous_close) / 3.0;
                let range = previous_high - previous_low;
                self.levels = PivotPointsValue {
                    pivot,
                    resistance_one: 2.0 * pivot - previous_low,
                    support_one: 2.0 * pivot - previous_high,
                    support_two: pivot - range,
                    resistance_two: pivot + range,
                };
                self.value = Some(self.levels);
            }
            self.high = Some(high);
            self.low = Some(low);
            self.close = Some(close);
        } else {
            self.high = Some(self.high.map_or(high, |value| value.max(high)));
            self.low = Some(self.low.map_or(low, |value| value.min(low)));
            self.close = Some(close);
        }
        self.levels
    }

    pub fn extend_slice_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        anchor: &[bool],
        output: &mut [Vec<f64>; 5],
    ) -> Result<(), crate::error::TaError> {
        if high.len() != low.len() || high.len() != close.len() || high.len() != anchor.len() {
            return Err(crate::error::TaError::LengthMismatch {
                expected: high.len(),
                got: low.len().max(close.len()).max(anchor.len()),
            });
        }
        for (((&high, &low), &close), &anchor) in high.iter().zip(low).zip(close).zip(anchor) {
            let value = self.append(high, low, close, anchor);
            output[0].push(value.pivot);
            output[1].push(value.resistance_one);
            output[2].push(value.support_one);
            output[3].push(value.support_two);
            output[4].push(value.resistance_two);
        }
        Ok(())
    }

    /// Returns the latest five pivot levels.
    pub fn value(&self) -> Option<PivotPointsValue> {
        self.value
    }
    /// Clears current session and levels.
    pub fn reset(&mut self) {
        self.high = None;
        self.low = None;
        self.close = None;
        self.levels = PivotPointsValue {
            pivot: f64::NAN,
            resistance_one: f64::NAN,
            support_one: f64::NAN,
            support_two: f64::NAN,
            resistance_two: f64::NAN,
        };
        self.value = None;
    }
}
