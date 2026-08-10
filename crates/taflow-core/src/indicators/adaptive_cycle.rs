use crate::error::TaResult;
use crate::stream::StreamingIndicator;
#[derive(Debug, Clone)]
pub struct AdaptiveCycle {
    previous: Option<f64>,
    value: Option<f64>,
}
impl AdaptiveCycle {
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            previous: None,
            value: None,
        })
    }
    pub fn append(&mut self, x: f64) -> Option<f64> {
        let out = self.previous.map(|p| x - p);
        self.previous = Some(x);
        self.value = out;
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
impl StreamingIndicator for AdaptiveCycle {
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
