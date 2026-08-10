use crate::error::TaResult;
use crate::stream::StreamingIndicator;
#[derive(Debug, Clone)]
pub struct Decycler {
    alpha: f64,
    trend: Option<f64>,
    value: Option<f64>,
}
impl Decycler {
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            alpha: 2.0 / (period as f64 + 1.0),
            trend: None,
            value: None,
        })
    }
    pub fn append(&mut self, x: f64) -> Option<f64> {
        let t = self.trend.map_or(x, |v| v + self.alpha * (x - v));
        self.trend = Some(t);
        self.value = Some(t);
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.trend = None;
        self.value = None;
    }
}
impl StreamingIndicator for Decycler {
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
