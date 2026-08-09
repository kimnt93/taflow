#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SessionsValue {
    pub active: f64,
    pub session_high: f64,
    pub session_low: f64,
}

#[derive(Debug, Clone)]
/// Causal session-scoped running extrema.
pub struct Sessions {
    session_high: Option<f64>,
    session_low: Option<f64>,
    started: bool,
    value: Option<SessionsValue>,
}

impl Sessions {
    pub fn new() -> Self {
        Self {
            session_high: None,
            session_low: None,
            started: false,
            value: None,
        }
    }

    pub fn append(&mut self, new_session: bool, high: f64, low: f64) -> SessionsValue {
        if new_session || !self.started {
            self.session_high = Some(high);
            self.session_low = Some(low);
            self.started = true;
        } else {
            self.session_high = Some(self.session_high.map_or(high, |running| running.max(high)));
            self.session_low = Some(self.session_low.map_or(low, |running| running.min(low)));
        }
        let value = SessionsValue {
            active: 1.0,
            session_high: self.session_high.unwrap_or(f64::NAN),
            session_low: self.session_low.unwrap_or(f64::NAN),
        };
        self.value = Some(value);
        value
    }

    pub fn value(&self) -> Option<SessionsValue> {
        self.value
    }

    pub fn reset(&mut self) {
        self.session_high = None;
        self.session_low = None;
        self.started = false;
        self.value = None;
    }
}

impl Default for Sessions {
    fn default() -> Self {
        Self::new()
    }
}
