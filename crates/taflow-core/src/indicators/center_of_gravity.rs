use crate::error::TaResult;
use crate::indicators::rolling_statistic_helpers::RollingValues;
use crate::stream::StreamingIndicator;
#[derive(Debug, Clone)]
pub struct CenterOfGravity {
    period: usize,
    values: RollingValues,
    value: Option<f64>,
}
impl CenterOfGravity {
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
            let denominator = self.values.window().iter().sum::<f64>();
            if denominator == 0.0 {
                0.0
            } else {
                -self
                    .values
                    .window()
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(i, v)| (i + 1) as f64 * v)
                    .sum::<f64>()
                    / denominator
                    + (self.period as f64 + 1.0) / 2.0
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
impl StreamingIndicator for CenterOfGravity {
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
