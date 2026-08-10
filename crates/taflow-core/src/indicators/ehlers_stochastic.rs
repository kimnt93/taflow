use crate::error::TaResult;
use crate::indicators::rolling_statistic_helpers::RollingValues;
use crate::stream::StreamingIndicator;
#[derive(Debug, Clone)]
pub struct EhlersStochastic {
    values: RollingValues,
    value: Option<f64>,
}
impl EhlersStochastic {
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            values: RollingValues::new(period)?,
            value: None,
        })
    }
    pub fn append(&mut self, x: f64) -> Option<f64> {
        self.values.push(x);
        self.value = self.values.is_full().then(|| {
            let high = self
                .values
                .window()
                .iter()
                .copied()
                .fold(f64::NEG_INFINITY, f64::max);
            let low = self
                .values
                .window()
                .iter()
                .copied()
                .fold(f64::INFINITY, f64::min);
            if high == low {
                0.0
            } else {
                (x - low) / (high - low)
            }
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
impl StreamingIndicator for EhlersStochastic {
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
