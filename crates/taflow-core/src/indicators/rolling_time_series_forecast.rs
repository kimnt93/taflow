use crate::error::TaResult;
use crate::stream::regression::RegressionCore;
use crate::stream::StreamingIndicator;

#[derive(Debug, Clone)]
pub struct RollingTimeSeriesForecast {
    core: RegressionCore,
    value: Option<f64>,
}

impl RollingTimeSeriesForecast {
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            core: RegressionCore::new(period)?,
            value: None,
        })
    }
}

impl StreamingIndicator for RollingTimeSeriesForecast {
    type Output = f64;
    fn append(&mut self, input: f64) -> Option<f64> {
        let period = self.core.period;
        self.value = self
            .core
            .append(input)
            .map(|v| v.intercept + v.slope * period as f64);
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
        let period = self.core.period;
        self.value = self
            .core
            .extend_map_into(inputs, output, |v| v.intercept + v.slope * period as f64);
    }
}
