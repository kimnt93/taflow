use crate::error::{TaError, TaResult};
use crate::indicators::rolling_statistic_helpers::RollingValues;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DoubleBollingerBandsValue {
    pub upper_outer: f64,
    pub upper_inner: f64,
    pub middle: f64,
    pub lower_inner: f64,
    pub lower_outer: f64,
}

/// Two population-standard-deviation envelopes around one simple average.
#[derive(Debug, Clone)]
pub struct DoubleBollingerBands {
    period: usize,
    inner_multiplier: f64,
    outer_multiplier: f64,
    values: RollingValues,
    value: Option<DoubleBollingerBandsValue>,
}

impl DoubleBollingerBands {
    /// Create ordered inner and outer Bollinger envelopes.
    pub fn new(period: usize, inner_multiplier: f64, outer_multiplier: f64) -> TaResult<Self> {
        if !inner_multiplier.is_finite() || inner_multiplier <= 0.0 {
            return Err(TaError::InvalidParameter {
                name: "inner_multiplier",
                value: inner_multiplier.to_string(),
                reason: "must be finite and positive",
            });
        }
        if !outer_multiplier.is_finite() || outer_multiplier <= inner_multiplier {
            return Err(TaError::InvalidParameter {
                name: "outer_multiplier",
                value: outer_multiplier.to_string(),
                reason: "must be finite and greater than inner_multiplier",
            });
        }
        Ok(Self {
            period,
            inner_multiplier,
            outer_multiplier,
            values: RollingValues::new(period)?,
            value: None,
        })
    }

    /// Append one price and return outer/inner/middle/inner/outer bands.
    pub fn append(&mut self, input: f64) -> Option<DoubleBollingerBandsValue> {
        self.values.push(input);
        self.value = self.values.is_full().then(|| {
            let n = self.period as f64;
            let middle = self.values.iter().sum::<f64>() / n;
            let deviation = (self
                .values
                .iter()
                .map(|value| (value - middle).powi(2))
                .sum::<f64>()
                / n)
                .sqrt();
            DoubleBollingerBandsValue {
                upper_outer: middle + self.outer_multiplier * deviation,
                upper_inner: middle + self.inner_multiplier * deviation,
                middle,
                lower_inner: middle - self.inner_multiplier * deviation,
                lower_outer: middle - self.outer_multiplier * deviation,
            }
        });
        self.value
    }

    /// Return all five latest bands, or `None` during warm-up.
    pub fn value(&self) -> Option<DoubleBollingerBandsValue> {
        self.value
    }
    /// Clear the rolling observations and latest band value.
    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}
