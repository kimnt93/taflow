#[derive(Debug, Clone)]
/// Stateful Force Index derived from close-to-close change and volume.
pub struct ForceIndex {
    previous: Option<f64>,
    value: Option<f64>,
}

impl ForceIndex {
    pub fn new() -> Self {
        Self {
            previous: None,
            value: None,
        }
    }

    pub fn append(&mut self, close: f64, volume: f64) -> Option<f64> {
        let previous = self.previous.replace(close)?;
        self.value = Some((close - previous) * volume);
        self.value
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }

    pub fn reset(&mut self) {
        self.previous = None;
        self.value = None;
    }
}

impl Default for ForceIndex {
    fn default() -> Self {
        Self::new()
    }
}
