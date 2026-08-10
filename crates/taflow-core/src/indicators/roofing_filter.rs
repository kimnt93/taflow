use crate::error::TaResult;
use crate::stream::StreamingIndicator;
#[derive(Debug, Clone)]
pub struct RoofingFilter {
    high_alpha: f64,
    low_alpha: f64,
    previous_input: Option<f64>,
    high: Option<f64>,
    low: Option<f64>,
    value: Option<f64>,
}
impl RoofingFilter {
    pub fn new(low_period: usize, high_period: usize) -> TaResult<Self> {
        Ok(Self {
            high_alpha: 2.0 / (high_period as f64 + 1.0),
            low_alpha: 2.0 / (low_period as f64 + 1.0),
            previous_input: None,
            high: None,
            low: None,
            value: None,
        })
    }
    pub fn append(&mut self, x: f64) -> Option<f64> {
        let hp = match (self.previous_input, self.high) {
            (Some(p), Some(h)) => (1.0 - self.high_alpha) * (h + x - p),
            _ => 0.0,
        };
        let lp = self.low.map_or(hp, |v| v + self.low_alpha * (hp - v));
        self.previous_input = Some(x);
        self.high = Some(hp);
        self.low = Some(lp);
        self.value = Some(lp);
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.previous_input = None;
        self.high = None;
        self.low = None;
        self.value = None;
    }
}
impl StreamingIndicator for RoofingFilter {
    type Output = f64;
    fn append(&mut self, x: f64) -> Option<f64> {
        Self::append(self, x)
    }
    fn value(&self) -> Option<f64> {
        self.value
    }
    fn reset(&mut self) {
        Self::reset(self)
    }
}
