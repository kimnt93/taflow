use crate::error::TaResult;
use crate::indicators::rolling_statistic_helpers::RollingValues;
use crate::stream::StreamingIndicator;
#[derive(Debug, Clone)]
pub struct DoubleBollingerBands {
    period: usize,
    values: RollingValues,
    value: Option<f64>,
}
impl DoubleBollingerBands {
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            period,
            values: RollingValues::new(period)?,
            value: None,
        })
    }
    pub fn append(&mut self, x: f64) -> Option<f64> {
        self.values.push(x);
        self.value = self.values.is_full().then(|| {
            let n = self.period as f64;
            self.values.window().iter().sum::<f64>() / n
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
impl StreamingIndicator for DoubleBollingerBands {
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
