use crate::error::TaResult;
use crate::indicators::rolling_statistic_helpers::RollingValues;
use crate::stream::StreamingIndicator;
#[derive(Debug, Clone)]
pub struct MovingAverageEnvelope {
    period: usize,
    percent: f64,
    values: RollingValues,
    value: Option<f64>,
}
impl MovingAverageEnvelope {
    pub fn new(period: usize, percent: f64) -> TaResult<Self> {
        Ok(Self {
            period,
            percent,
            values: RollingValues::new(period)?,
            value: None,
        })
    }
    pub fn append(&mut self, x: f64) -> Option<f64> {
        self.values.push(x);
        self.value = (self.values.is_full()).then(|| {
            self.values.window().iter().sum::<f64>() / self.period as f64 * (1.0 + self.percent)
        });
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
impl StreamingIndicator for MovingAverageEnvelope {
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
