use crate::error::TaResult;
use crate::indicators::rolling_statistic_helpers::RollingValues;
use crate::stream::StreamingIndicator;
#[derive(Debug, Clone)]
pub struct LinearRegressionChannel {
    period: usize,
    values: RollingValues,
    value: Option<f64>,
}
impl LinearRegressionChannel {
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
            let mx = (n - 1.0) / 2.0;
            let my = self.values.window().iter().sum::<f64>() / n;
            let (mut a, mut b) = (0.0, 0.0);
            for (i, y) in self.values.window().iter().enumerate() {
                let dx = i as f64 - mx;
                a += dx * (y - my);
                b += dx * dx;
            }
            my + (if b == 0.0 { 0.0 } else { a / b }) * (n - 1.0 - mx)
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
impl StreamingIndicator for LinearRegressionChannel {
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
