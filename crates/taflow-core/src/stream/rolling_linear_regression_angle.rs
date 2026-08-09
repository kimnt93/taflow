use super::regression::RegressionCore;
use super::StreamingIndicator;
use crate::error::TaResult;

#[derive(Debug, Clone)]
pub struct RollingLinearRegressionAngle {
    core: RegressionCore,
    value: Option<f64>,
}

impl RollingLinearRegressionAngle {
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            core: RegressionCore::new(period)?,
            value: None,
        })
    }
}

impl StreamingIndicator for RollingLinearRegressionAngle {
    type Output = f64;
    fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self.core.append(input).map(|v| v.slope.atan().to_degrees());
        self.value
    }
    fn value(&self) -> Option<f64> {
        self.value
    }
    fn reset(&mut self) {
        self.core.reset();
        self.value = None;
    }
    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        self.value = self
            .core
            .extend_map_into(inputs, output, |v| v.slope.atan().to_degrees());
    }
}
