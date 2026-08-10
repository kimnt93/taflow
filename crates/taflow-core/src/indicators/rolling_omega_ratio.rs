use crate::error::TaResult;
use crate::indicators::rolling_statistic_helpers::RollingValues;
use crate::stream::StreamingIndicator;

/// Rolling gains above a threshold divided by losses below that threshold.
///
/// The state retains only the configured window and emits after it is full.
#[derive(Debug, Clone)]
pub struct RollingOmegaRatio {
    values: RollingValues,
    threshold: f64,
    value: Option<f64>,
}

impl RollingOmegaRatio {
    /// Creates an Omega ratio with a positive rolling period.
    pub fn new(timeperiod: usize, threshold: f64) -> TaResult<Self> {
        Ok(Self {
            values: RollingValues::new(timeperiod)?,
            threshold,
            value: None,
        })
    }

    /// Appends one return observation and returns the latest warm ratio.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if !input.is_finite() {
            return None;
        }
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
            if losses == 0.0 {
                if gains == 0.0 {
                    0.0
                } else {
                    f64::INFINITY
                }
            } else {
                gains / losses
            }
        });
        self.value
    }
    /// Returns the latest ratio, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Clears the rolling window and latest ratio without reallocating.
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
