use crate::error::TaResult;
use crate::indicators::rolling_statistic_helpers::RollingValues;
use crate::stream::StreamingIndicator;

#[derive(Debug, Clone)]
pub struct RollingOmegaRatio {
    values: RollingValues,
    threshold: f64,
    value: Option<f64>,
}

impl RollingOmegaRatio {
    pub fn new(timeperiod: usize, threshold: f64) -> TaResult<Self> {
        Ok(Self {
            values: RollingValues::new(timeperiod)?,
            threshold,
            value: None,
        })
    }

    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.values.push(input);
        self.value = self.values.is_full().then(|| {
            let (mut gains, mut losses) = (0.0, 0.0);
            for &sample in self.values.iter() {
                if sample >= self.threshold {
                    gains += sample - self.threshold;
                } else {
                    losses += self.threshold - sample;
                }
            }
            if losses > 0.0 {
                gains / losses
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

impl StreamingIndicator for RollingOmegaRatio {
    type Output = f64;
    fn append(&mut self, input: f64) -> Option<f64> {
        Self::append(self, input)
    }
    fn value(&self) -> Option<f64> {
        Self::value(self)
    }
    fn reset(&mut self) {
        Self::reset(self);
    }
}
