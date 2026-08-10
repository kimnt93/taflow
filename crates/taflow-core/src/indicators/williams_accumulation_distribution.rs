use crate::error::TaResult;
#[derive(Debug, Clone)]
pub struct WilliamsAccumulationDistribution {
    previous: Option<f64>,
    value: f64,
}
impl WilliamsAccumulationDistribution {
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            previous: None,
            value: 0.0,
        })
    }
    pub fn append(&mut self, h: f64, l: f64, c: f64) -> Option<f64> {
        if let Some(p) = self.previous {
            self.value += if c > p {
                c - l
            } else if c < p {
                c - h
            } else {
                0.0
            };
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
