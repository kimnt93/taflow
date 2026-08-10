use crate::error::TaResult;
use crate::stream::StreamingIndicator;
#[derive(Debug, Clone)]
pub struct SuperSmoother {
    alpha: f64,
    previous_input: Option<f64>,
    previous_output: Option<f64>,
    value: Option<f64>,
}
impl SuperSmoother {
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            alpha: 2.0 / (period as f64 + 1.0),
            previous_input: None,
            previous_output: None,
            value: None,
        })
    }
    pub fn append(&mut self, x: f64) -> Option<f64> {
        let out = match (self.previous_input, self.previous_output) {
            (Some(p), Some(y)) => y + self.alpha * ((x + p) * 0.5 - y),
            _ => x,
        };
        self.previous_input = Some(x);
        self.previous_output = Some(out);
        self.value = Some(out);
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.previous_input = None;
        self.previous_output = None;
        self.value = None;
    }
}
impl StreamingIndicator for SuperSmoother {
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
