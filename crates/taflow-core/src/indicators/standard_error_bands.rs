use crate::error::{TaError, TaResult};
use crate::indicators::rolling_statistic_helpers::RollingValues;
use crate::stream::invalid_period;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StandardErrorBandsValue {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}

/// Rolling OLS endpoint wrapped by residual standard error.
#[derive(Debug, Clone)]
pub struct StandardErrorBands {
    period: usize,
    multiplier: f64,
    values: RollingValues,
    sum_x: f64,
    sum_x_squared: f64,
    value: Option<StandardErrorBandsValue>,
}

impl StandardErrorBands {
    /// Create bands with at least three observations and positive multiplier.
    pub fn new(period: usize, multiplier: f64) -> TaResult<Self> {
        if period < 3 {
            return Err(invalid_period("period", period, 3));
        }
        if !multiplier.is_finite() || multiplier <= 0.0 {
            return Err(TaError::InvalidParameter {
                name: "multiplier",
                value: multiplier.to_string(),
                reason: "must be finite and positive",
            });
        }
        let n = period as f64;
        Ok(Self {
            period,
            multiplier,
            values: RollingValues::new(period)?,
            sum_x: n * (n - 1.0) * 0.5,
            sum_x_squared: (n - 1.0) * n * (2.0 * n - 1.0) / 6.0,
            value: None,
        })
    }

    /// Append one price and return the OLS standard-error bands.
    pub fn append(&mut self, input: f64) -> Option<StandardErrorBandsValue> {
        self.values.push(input);
        if !self.values.is_full() {
            self.value = None;
            return None;
        }
        let n = self.period as f64;
        let sum_y = self.values.iter().sum::<f64>();
        let sum_xy = self
            .values
            .iter()
            .enumerate()
            .map(|(index, value)| index as f64 * value)
            .sum::<f64>();
        let slope =
            (n * sum_xy - self.sum_x * sum_y) / (n * self.sum_x_squared - self.sum_x * self.sum_x);
        let intercept = (sum_y - slope * self.sum_x) / n;
        let residual_sum = self
            .values
            .iter()
            .enumerate()
            .map(|(index, value)| {
                let residual = value - (intercept + slope * index as f64);
                residual * residual
            })
            .sum::<f64>();
        let middle = intercept + slope * (n - 1.0);
        let width = self.multiplier * (residual_sum / (n - 2.0)).sqrt();
        self.value = Some(StandardErrorBandsValue {
            upper: middle + width,
            middle,
            lower: middle - width,
        });
        self.value
    }

    /// Return the latest three bands, or `None` during warm-up.
    pub fn value(&self) -> Option<StandardErrorBandsValue> {
        self.value
    }
    /// Clear the rolling observations and latest band value.
    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}
