use crate::error::TaResult;
use crate::stream::StreamingIndicator;

#[derive(Debug, Clone)]
pub struct RollingDrawdownDuration {
    peak: f64,
    duration: usize,
    value: Option<f64>,
}
impl RollingDrawdownDuration {
    pub fn new() -> TaResult<Self> {
        Ok(Self {
            peak: f64::NEG_INFINITY,
            duration: 0,
            value: None,
        })
    }
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if input >= self.peak {
            self.peak = input;
            self.duration = 0;
        } else {
            self.duration += 1;
        }
        self.value = Some(self.duration as f64);
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.peak = f64::NEG_INFINITY;
        self.duration = 0;
        self.value = None;
    }
}
impl StreamingIndicator for RollingDrawdownDuration {
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
