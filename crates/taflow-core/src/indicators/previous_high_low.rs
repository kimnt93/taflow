#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PreviousHighLowValue {
    pub prev_high: f64,
    pub prev_low: f64,
    pub broken_high: f64,
    pub broken_low: f64,
}

#[derive(Debug, Clone)]
/// Previous session extrema and causal break flags.
pub struct PreviousHighLow {
    running_high: Option<f64>,
    running_low: Option<f64>,
    previous_high: Option<f64>,
    previous_low: Option<f64>,
    value: Option<PreviousHighLowValue>,
}

impl PreviousHighLow {
    pub fn new() -> Self {
        Self {
            running_high: None,
            running_low: None,
            previous_high: None,
            previous_low: None,
            value: None,
        }
    }

    pub fn append(&mut self, new_session: bool, high: f64, low: f64) -> PreviousHighLowValue {
        if new_session {
            if self.running_high.is_some() {
                self.previous_high = self.running_high;
                self.previous_low = self.running_low;
            }
            self.running_high = Some(high);
            self.running_low = Some(low);
        } else {
            self.running_high = Some(self.running_high.map_or(high, |running| running.max(high)));
            self.running_low = Some(self.running_low.map_or(low, |running| running.min(low)));
        }
        let value = PreviousHighLowValue {
            prev_high: self.previous_high.unwrap_or(f64::NAN),
            prev_low: self.previous_low.unwrap_or(f64::NAN),
            broken_high: self.previous_high.map_or(f64::NAN, |previous| {
                if high > previous {
                    1.0
                } else {
                    f64::NAN
                }
            }),
            broken_low: self.previous_low.map_or(f64::NAN, |previous| {
                if low < previous {
                    1.0
                } else {
                    f64::NAN
                }
            }),
        };
        self.value = Some(value);
        value
    }

    pub fn value(&self) -> Option<PreviousHighLowValue> {
        self.value
    }

    pub fn reset(&mut self) {
        self.running_high = None;
        self.running_low = None;
        self.previous_high = None;
        self.previous_low = None;
        self.value = None;
    }
}

impl Default for PreviousHighLow {
    fn default() -> Self {
        Self::new()
    }
}
