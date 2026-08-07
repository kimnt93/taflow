//! Stateful and vectorized hundred-scaled rate-of-change ratio.

use crate::TaResult;
use super::{StreamingIndicator, lagged_common::{LaggedValue, validate_rate_of_change}};

/// Compute the rate of change ratio percent result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn rate_of_change_ratio_percent(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_rate_of_change(input, timeperiod)?;
    let mut output = vec![f64::NAN; timeperiod];
    output.extend(input[timeperiod..].iter().zip(&input[..input.len() - timeperiod]).map(|(&current, &previous)| if previous != 0.0 { current / previous * 100.0 } else { 0.0 }));
    Ok(output)
}

/// Computes the scaled lagged ratio incrementally.
#[derive(Debug, Clone)]
pub struct RateOfChangeRatioPercent { lag: LaggedValue, value: Option<f64> }
impl RateOfChangeRatioPercent {
    /// Creates scaled ratio state for a positive lag period.
    pub fn new(period: usize) -> TaResult<Self> { Ok(Self { lag: LaggedValue::new(period)?, value: None }) }
}
impl StreamingIndicator for RateOfChangeRatioPercent {
    type Output = f64;
    fn append(&mut self, input: f64) -> Option<f64> { self.value = self.lag.append(input).map(|(current, previous)| if previous != 0.0 { current / previous * 100.0 } else { 0.0 }); self.value }
    fn value(&self) -> Option<f64> { self.value }
    fn reset(&mut self) { self.lag.reset(); self.value = None; }
}
