#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SessionExtremaValue {
    pub high: f64,
    pub low: f64,
}

#[derive(Debug, Clone, Default)]
/// Running high and low values within explicit session boundaries.
pub struct SessionExtrema {
    high: Option<f64>,
    low: Option<f64>,
    value: Option<SessionExtremaValue>,
}

impl SessionExtrema {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, new_session: bool, high: f64, low: f64) -> SessionExtremaValue {
        if new_session || self.high.is_none() {
            self.high = Some(high);
            self.low = Some(low);
        } else {
            self.high = Some(self.high.expect("session high is initialized").max(high));
            self.low = Some(self.low.expect("session low is initialized").min(low));
        }
        let value = SessionExtremaValue {
            high: self.high.expect("session high is initialized"),
            low: self.low.expect("session low is initialized"),
        };
        self.value = Some(value);
        value
    }

    pub fn value(&self) -> Option<SessionExtremaValue> {
        self.value
    }

    pub fn reset(&mut self) {
        self.high = None;
        self.low = None;
        self.value = None;
    }
}
