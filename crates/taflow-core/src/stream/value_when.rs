//! Persistent value-at-condition state.

/// Retain the most recent input observed when its aligned condition was true.
#[derive(Debug, Clone, Default)]
pub struct ValueWhen {
    latest: Option<f64>,
    value: Option<f64>,
}

impl ValueWhen {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, condition: bool, input: f64) -> Option<f64> {
        if condition {
            self.latest = Some(input);
        }
        self.value = self.latest;
        self.value
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }

    pub fn reset(&mut self) {
        self.latest = None;
        self.value = None;
    }
}
