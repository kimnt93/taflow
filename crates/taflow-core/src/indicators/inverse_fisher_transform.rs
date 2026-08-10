use crate::error::TaResult;
use crate::stream::StreamingIndicator;
#[derive(Debug, Clone)]
pub struct InverseFisherTransform {
    scale: f64,
    value: Option<f64>,
}
impl InverseFisherTransform {
    pub fn new(scale: f64) -> TaResult<Self> {
        Ok(Self { scale, value: None })
    }
    pub fn append(&mut self, x: f64) -> Option<f64> {
        self.value = Some((self.scale * x).tanh());
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.value = None;
    }
}
impl StreamingIndicator for InverseFisherTransform {
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
