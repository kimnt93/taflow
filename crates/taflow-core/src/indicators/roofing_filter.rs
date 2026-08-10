use std::f64::consts::PI;

use crate::error::TaResult;
use crate::indicators::SuperSmoother;
use crate::stream::{invalid_period, StreamingIndicator};

/// Ehlers high-pass filter followed by a SuperSmoother low-pass.
#[derive(Debug, Clone)]
pub struct RoofingFilter {
    alpha: f64,
    previous_input: Option<f64>,
    previous_high_pass: f64,
    smoother: SuperSmoother,
    value: Option<f64>,
}

impl RoofingFilter {
    /// Create a roofing filter requiring `low_period < high_period`.
    pub fn new(low_period: usize, high_period: usize) -> TaResult<Self> {
        if low_period == 0 {
            return Err(invalid_period("low_period", low_period, 1));
        }
        if high_period <= low_period {
            return Err(invalid_period("high_period", high_period, low_period + 1));
        }
        let argument = 2.0 * PI / high_period as f64;
        Ok(Self {
            alpha: (argument.cos() + argument.sin() - 1.0) / argument.cos(),
            previous_input: None,
            previous_high_pass: 0.0,
            smoother: SuperSmoother::new(low_period)?,
            value: None,
        })
    }

    /// Append one sample and return the band-limited output.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        let high_pass = self.previous_input.map_or(0.0, |previous| {
            (1.0 - self.alpha * 0.5) * (input - previous)
                + (1.0 - self.alpha) * self.previous_high_pass
        });
        self.previous_input = Some(input);
        self.previous_high_pass = high_pass;
        self.value = self.smoother.append(high_pass);
        self.value
    }

    /// Return the latest band-limited value.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clear high-pass and smoothing history.
    pub fn reset(&mut self) {
        self.previous_input = None;
        self.previous_high_pass = 0.0;
        self.smoother.reset();
        self.value = None;
    }
}

impl StreamingIndicator for RoofingFilter {
    type Output = f64;
    fn append(&mut self, input: f64) -> Option<f64> {
        Self::append(self, input)
    }
    fn value(&self) -> Option<f64> {
        self.value
    }
    fn reset(&mut self) {
        Self::reset(self)
    }
}
