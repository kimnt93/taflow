use crate::error::{TaError, TaResult};
use crate::indicators::rolling_statistic_helpers::RollingValues;
use crate::stream::StreamingIndicator;

/// Rolling mean loss at or below the configured lower-tail quantile.
#[derive(Debug, Clone)]
pub struct RollingConditionalValueAtRisk {
    values: RollingValues,
    sorted: Vec<f64>,
    confidence: f64,
    value: Option<f64>,
}

impl RollingConditionalValueAtRisk {
    /// Creates the estimator with a positive period and confidence in `(0, 1)`.
    pub fn new(timeperiod: usize, confidence: f64) -> TaResult<Self> {
        if timeperiod < 2 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: timeperiod.to_string(),
                reason: "must be >= 2",
            });
        }
        if !(0.0..1.0).contains(&confidence) {
            return Err(TaError::InvalidParameter {
                name: "confidence",
                value: confidence.to_string(),
                reason: "must be between 0 and 1",
            });
        }
        Ok(Self {
            values: RollingValues::new(timeperiod)?,
            sorted: Vec::with_capacity(timeperiod),
            confidence,
            value: None,
        })
    }
    /// Appends one return and emits expected tail loss when the window is full.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if !input.is_finite() {
            return None;
        }
        self.values.push(input);
        self.value = if self.values.is_full() {
            self.sorted.clear();
            self.sorted.extend(self.values.iter().copied());
            self.sorted.sort_by(f64::total_cmp);
            let count =
                (((1.0 - self.confidence) * self.sorted.len() as f64).floor() as usize).max(1);
            let mean = self.sorted[..count].iter().sum::<f64>() / count as f64;
            Some((-mean).max(0.0))
        } else {
            None
        };
        self.value
    }
    /// Returns the latest tail-loss estimate, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Clears retained returns and the latest estimate.
    pub fn reset(&mut self) {
        self.values.clear();
        self.sorted.clear();
        self.value = None;
    }
}
impl StreamingIndicator for RollingConditionalValueAtRisk {
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
