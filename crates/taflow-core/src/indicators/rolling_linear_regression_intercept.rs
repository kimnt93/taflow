use crate::error::TaResult;
use crate::stream::regression::RegressionCore;
use crate::stream::StreamingIndicator;

#[derive(Debug, Clone)]
pub struct RollingLinearRegressionIntercept {
    core: RegressionCore,
    value: Option<f64>,
}

impl RollingLinearRegressionIntercept {
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            core: RegressionCore::new(period)?,
            value: None,
        })
    }
}

impl StreamingIndicator for RollingLinearRegressionIntercept {
    type Output = f64;
    fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self.core.append(input).map(|v| v.intercept);
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
        self.value = self.core.extend_map_into(inputs, output, |v| v.intercept);
    }
}
