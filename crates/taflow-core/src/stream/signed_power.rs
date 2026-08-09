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
