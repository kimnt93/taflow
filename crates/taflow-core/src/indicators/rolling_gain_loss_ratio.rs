use crate::error::TaResult;
use crate::indicators::rolling_statistic_helpers::RollingValues;
use crate::stream::StreamingIndicator;

#[derive(Debug, Clone)]
pub struct RollingGainLossRatio {
    values: RollingValues,
    value: Option<f64>,
}
impl RollingGainLossRatio {
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            values: RollingValues::new(period)?,
            value: None,
        })
    }
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.values.push(input);
        self.value = self.values.is_full().then(|| {
            let (mut g, mut l) = (0.0, 0.0);
            for &v in self.values.iter() {
                if v > 0.0 {
                    g += v
                } else {
                    l -= v
                }
            }
            if l == 0.0 {
                if g == 0.0 {
                    0.0
                } else {
                    f64::INFINITY
                }
            } else {
                g / l
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
impl StreamingIndicator for RollingGainLossRatio {
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
