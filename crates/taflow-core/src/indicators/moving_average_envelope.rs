use crate::error::{TaError, TaResult};
use crate::indicators::rolling_statistic_helpers::RollingValues;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MovingAverageEnvelopeValue {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}

/// Simple moving average wrapped by fixed percentage envelopes.
#[derive(Debug, Clone)]
pub struct MovingAverageEnvelope {
    period: usize,
    percent: f64,
    values: RollingValues,
    value: Option<MovingAverageEnvelopeValue>,
}

impl MovingAverageEnvelope {
    /// Create an envelope with positive period and percentage.
    pub fn new(period: usize, percent: f64) -> TaResult<Self> {
        if !percent.is_finite() || percent <= 0.0 {
            return Err(TaError::InvalidParameter {
                name: "percent",
                value: percent.to_string(),
                reason: "must be finite and positive",
            });
        }
        Ok(Self {
            period,
            percent,
            values: RollingValues::new(period)?,
            value: None,
        })
    }

    /// Append one price and return upper, middle, and lower bands.
    pub fn append(&mut self, input: f64) -> Option<MovingAverageEnvelopeValue> {
        self.values.push(input);
        self.value = self.values.is_full().then(|| {
            let middle = self.values.iter().sum::<f64>() / self.period as f64;
            MovingAverageEnvelopeValue {
                upper: middle * (1.0 + self.percent),
                middle,
                lower: middle * (1.0 - self.percent),
            }
        });
        self.value
    }

    /// Return the latest three bands, or `None` while the window is incomplete.
    pub fn value(&self) -> Option<MovingAverageEnvelopeValue> {
        self.value
    }
    /// Restore fresh-state behavior while retaining rolling storage.
    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}
