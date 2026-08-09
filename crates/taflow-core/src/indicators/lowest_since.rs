//! Persistent lowest-since-condition state.

/// Track the minimum input since the latest true condition, inclusively.
#[derive(Debug, Clone, Default)]
pub struct LowestSince {
    extreme: Option<f64>,
    value: Option<f64>,
}

impl LowestSince {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, condition: bool, input: f64) -> Option<f64> {
        self.extreme = Some(if condition {
            input
        } else {
            self.extreme.map_or(input, |value| value.min(input))
        });
        self.value = self.extreme;
        self.value
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }

    pub fn reset(&mut self) {
        self.extreme = None;
        self.value = None;
    }
}
