//! Persistent bars-since counter.

/// Count bars since the most recent true condition, resetting to zero on true.
#[derive(Debug, Clone, Default)]
pub struct BarsSince {
    count: Option<usize>,
    value: Option<f64>,
}

impl BarsSince {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, condition: bool) -> Option<f64> {
        self.count = Some(if condition {
            0
        } else {
            self.count.map_or(0, |value| value + 1)
        });
        self.value = self.count.map(|value| value as f64);
        self.value
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }

    pub fn reset(&mut self) {
        self.count = None;
        self.value = None;
    }
}
