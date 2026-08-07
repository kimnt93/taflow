//! Causal exponentially weighted moving sum.

use crate::error::TaResult;

use super::operator_states::ewm_alpha;

/// Compute an aligned exponentially weighted sum using span=`timeperiod`.
pub fn ewm_sum(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = ExponentiallyWeightedSum::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value)).collect())
}

/// Persistent exponentially weighted sum with recurrence
/// `sum_t = x_t + (1 - alpha) * sum_(t-1)`.
#[derive(Debug, Clone)]
pub struct ExponentiallyWeightedSum {
    decay: f64,
    value: Option<f64>,
}

impl ExponentiallyWeightedSum {
    /// Create a state using `alpha = 2 / (timeperiod + 1)`.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            decay: 1.0 - ewm_alpha(timeperiod)?,
            value: None,
        })
    }

    /// Append one observation and return the updated weighted sum.
    pub fn append(&mut self, input: f64) -> f64 {
        let value = input + self.decay * self.value.unwrap_or(0.0);
        self.value = Some(value);
        value
    }

    /// Return the latest weighted sum.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clear accumulated weight while retaining the configured decay.
    pub fn reset(&mut self) {
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_polars_documented_example_and_reset() {
        let expected = vec![1.0, 2.5, 4.25];
        assert_eq!(ewm_sum(&[1.0, 2.0, 3.0], 3).unwrap(), expected);
        let mut state = ExponentiallyWeightedSum::new(3).unwrap();
        assert_eq!(state.append(1.0), 1.0);
        assert_eq!(state.append(2.0), 2.5);
        state.reset();
        assert_eq!(state.value(), None);
        assert_eq!(state.append(3.0), 3.0);
    }

    #[test]
    fn rejects_zero_period() {
        assert!(ExponentiallyWeightedSum::new(0).is_err());
    }
}
