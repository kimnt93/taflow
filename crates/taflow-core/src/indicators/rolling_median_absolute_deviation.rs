use crate::error::TaResult;
use crate::indicators::rolling_statistic_helpers::{quantile, RollingValues};
use crate::stream::StreamingIndicator;
#[derive(Debug, Clone)]
pub struct RollingMedianAbsoluteDeviation {
    values: RollingValues,
    value: Option<f64>,
}
impl RollingMedianAbsoluteDeviation {
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            values: RollingValues::new(period)?,
            value: None,
        })
    }
    pub fn append(&mut self, x: f64) -> Option<f64> {
        self.values.push(x);
        self.value = self.values.is_full().then(|| {
            let m = quantile(self.values.window(), 0.5);
            let mut d = self
                .values
                .window()
                .iter()
                .map(|v| (v - m).abs())
                .collect::<Vec<_>>();
            d.sort_by(f64::total_cmp);
            d[d.len() / 2]
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
impl StreamingIndicator for RollingMedianAbsoluteDeviation {
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
