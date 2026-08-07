//! Batch implementation for `signed_power`.

/// Persistent pointwise signed-power state.
#[derive(Debug, Clone)]
pub struct SignedPower {
    exponent: f64,
    value: Option<f64>,
}

impl SignedPower {
    /// Create a state that computes `sign(x) * abs(x)^exponent`.
    pub fn new(exponent: f64) -> Self {
        Self {
            exponent,
            value: None,
        }
    }

    /// Append one observation and return its signed power.
    pub fn append(&mut self, input: f64) -> f64 {
        let value = if self.exponent == 2.0 {
            input * input.abs()
        } else {
            input.signum() * input.abs().powf(self.exponent)
        };
        self.value = Some(value);
        value
    }

    /// Return the most recently computed value.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clear the latest value while retaining the configured exponent.
    pub fn reset(&mut self) {
        self.value = None;
    }
}

/// Computes pointwise signed power `sign(x)·|x|^a`.
/// Compute the signed power result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `exponent` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn signed_power(input: &[f64], exponent: f64) -> Vec<f64> {
    let mut state = SignedPower::new(exponent);
    input.iter().map(|&value| state.append(value)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_sign_and_resets() {
        assert_eq!(signed_power(&[-2.0, 0.0, 3.0], 2.0), vec![-4.0, 0.0, 9.0]);
        let mut state = SignedPower::new(0.5);
        assert_eq!(state.append(-4.0), -2.0);
        state.reset();
        assert_eq!(state.value(), None);
    }
}
