//! Stateful and vectorized fractional rate of change.

use crate::TaResult;
use super::{StreamingIndicator, lagged_common::{LaggedValue, validate_rate_of_change}};

/// Computes fractional rate of change over a fixed lag.
pub fn rate_of_change_percent(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_rate_of_change(input, timeperiod)?;
    let mut output = vec![f64::NAN; timeperiod];
    output.extend(input[timeperiod..].iter().zip(&input[..input.len() - timeperiod]).map(|(&current, &previous)| if previous != 0.0 { (current - previous) / previous } else { 0.0 }));
    Ok(output)
}

/// Computes fractional rate of change incrementally.
#[derive(Debug, Clone)]
pub struct RateOfChangePercent { lag: LaggedValue, value: Option<f64> }
impl RateOfChangePercent {
    /// Creates fractional rate-of-change state for a positive lag period.
    pub fn new(period: usize) -> TaResult<Self> { Ok(Self { lag: LaggedValue::new(period)?, value: None }) }
}
impl StreamingIndicator for RateOfChangePercent {
    type Output = f64;
    fn append(&mut self, input: f64) -> Option<f64> { self.value = self.lag.append(input).map(|(current, previous)| if previous != 0.0 { (current - previous) / previous } else { 0.0 }); self.value }
    fn value(&self) -> Option<f64> { self.value }
    fn reset(&mut self) { self.lag.reset(); self.value = None; }
}
