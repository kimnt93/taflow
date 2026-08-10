use crate::error::TaResult;
#[derive(Debug, Clone)]
pub struct TimeSegmentedVolume {
    previous: Option<f64>,
    value: f64,
}
impl TimeSegmentedVolume {
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            previous: None,
            value: 0.0,
        })
    }
    pub fn append(&mut self, c: f64, v: f64) -> Option<f64> {
        if let Some(p) = self.previous {
            if c > p {
                self.value += v
            } else if c < p {
                self.value -= v
            }
        }
        self.previous = Some(c);
        Some(self.value)
    }
    pub fn value(&self) -> Option<f64> {
        self.previous.map(|_| self.value)
    }
    pub fn reset(&mut self) {
        self.previous = None;
        self.value = 0.0;
    }
}
