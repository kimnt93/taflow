//! Stateful and vectorized rate-of-change ratio.

use crate::TaResult;
use super::{StreamingIndicator, lagged_common::{LaggedValue, validate_rate_of_change}};

/// Computes the ratio of a value to its lagged value.
pub fn rate_of_change_ratio(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_rate_of_change(input, timeperiod)?;
    let mut output = vec![f64::NAN; timeperiod];
    output.extend(input[timeperiod..].iter().zip(&input[..input.len() - timeperiod]).map(|(&current, &previous)| if previous != 0.0 { current / previous } else { 0.0 }));
    Ok(output)
}

/// Computes the lagged value ratio incrementally.
#[derive(Debug, Clone)]
pub struct RateOfChangeRatio { lag: LaggedValue, value: Option<f64> }
impl RateOfChangeRatio {
    /// Creates ratio state for a positive lag period.
    pub fn new(period: usize) -> TaResult<Self> { Ok(Self { lag: LaggedValue::new(period)?, value: None }) }
}
impl StreamingIndicator for RateOfChangeRatio {
    type Output = f64;
    fn append(&mut self, input: f64) -> Option<f64> { self.value = self.lag.append(input).map(|(current, previous)| if previous != 0.0 { current / previous } else { 0.0 }); self.value }
    fn value(&self) -> Option<f64> { self.value }
    fn reset(&mut self) { self.lag.reset(); self.value = None; }
}
