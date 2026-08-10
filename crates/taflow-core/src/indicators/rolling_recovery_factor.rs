use crate::error::TaResult;
use crate::indicators::rolling_statistic_helpers::RollingValues;
use crate::stream::StreamingIndicator;

#[derive(Debug, Clone)]
pub struct RollingRecoveryFactor {
    values: RollingValues,
    value: Option<f64>,
}
impl RollingRecoveryFactor {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            values: RollingValues::new(timeperiod)?,
            value: None,
        })
    }
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.values.push(input);
        self.value = self.values.is_full().then(|| {
            let first = *self.values.iter().next().unwrap();
            let last = *self.values.iter().last().unwrap();
            let mut peak = first;
            let mut drawdown: f64 = 0.0;
            for &v in self.values.iter() {
                peak = peak.max(v);
                if peak != 0.0 {
                    drawdown = drawdown.max((peak - v) / peak);
                }
            }
            if drawdown > 0.0 {
                (last - first) / drawdown
            } else {
                0.0
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
impl StreamingIndicator for RollingRecoveryFactor {
    type Output = f64;
    fn append(&mut self, x: f64) -> Option<f64> {
        Self::append(self, x)
    }
    fn value(&self) -> Option<f64> {
        Self::value(self)
    }
    fn reset(&mut self) {
        Self::reset(self);
    }
}
