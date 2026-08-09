#[derive(Debug, Clone)]
/// Stateful Ease of Movement oscillator using high, low, and volume.
pub struct EaseOfMovement {
    previous_midpoint: Option<f64>,
    value: Option<f64>,
}

impl EaseOfMovement {
    pub fn new() -> Self {
        Self {
            previous_midpoint: None,
            value: None,
        }
    }

    pub fn append(&mut self, high: f64, low: f64, volume: f64) -> Option<f64> {
        let midpoint = (high + low) * 0.5;
        let previous = self.previous_midpoint.replace(midpoint)?;
        self.value = Some(if volume != 0.0 {
            (midpoint - previous) * (high - low) / volume
        } else {
            0.0
        });
        self.value
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }

    pub fn reset(&mut self) {
        self.previous_midpoint = None;
        self.value = None;
    }
}

impl Default for EaseOfMovement {
    fn default() -> Self {
        Self::new()
    }
}
