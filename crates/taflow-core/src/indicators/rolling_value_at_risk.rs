use crate::error::{TaError, TaResult};
use crate::indicators::rolling_statistic_helpers::{quantile, RollingValues};
use crate::stream::StreamingIndicator;

#[derive(Debug, Clone)]
pub struct RollingValueAtRisk {
    values: RollingValues,
    confidence: f64,
    value: Option<f64>,
}

impl RollingValueAtRisk {
    pub fn new(timeperiod: usize, confidence: f64) -> TaResult<Self> {
        if !(0.0..1.0).contains(&confidence) {
            return Err(TaError::InvalidParameter {
                name: "confidence",
                value: confidence.to_string(),
                reason: "must be between 0 and 1",
            });
        }
        Ok(Self {
            values: RollingValues::new(timeperiod)?,
            confidence,
            value: None,
        })
    }
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.values.push(input);
        self.value = self
            .values
            .is_full()
            .then(|| -quantile(self.values.window(), 1.0 - self.confidence));
        self.value
    }
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}
impl StreamingIndicator for RollingValueAtRisk {
    type Output = f64;
    fn append(&mut self, input: f64) -> Option<f64> {
        Self::append(self, input)
    }
    fn value(&self) -> Option<f64> {
        Self::value(self)
    }
    fn reset(&mut self) {
        Self::reset(self);
    }
}
