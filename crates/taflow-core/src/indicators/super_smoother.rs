use std::f64::consts::{PI, SQRT_2};

use crate::error::TaResult;
use crate::stream::{invalid_period, StreamingIndicator};

/// Ehlers two-pole Butterworth-style low-pass filter.
#[derive(Debug, Clone)]
pub struct SuperSmoother {
    c1: f64,
    c2: f64,
    c3: f64,
    previous_input: Option<f64>,
    previous_output_1: Option<f64>,
    previous_output_2: Option<f64>,
}

impl SuperSmoother {
    /// Create a filter with a non-zero critical period.
    pub fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("period", period, 1));
        }
        let argument = SQRT_2 * PI / period as f64;
        let a1 = (-argument).exp();
        let c2 = 2.0 * a1 * argument.cos();
        let c3 = -a1 * a1;
        Ok(Self {
            c1: 1.0 - c2 - c3,
            c2,
            c3,
            previous_input: None,
            previous_output_1: None,
            previous_output_2: None,
        })
    }

    /// Append one sample and return the filtered value from the first bar.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        let output = match (
            self.previous_input,
            self.previous_output_1,
            self.previous_output_2,
        ) {
            (Some(previous_input), Some(y1), Some(y2)) => {
                self.c1 * 0.5 * (input + previous_input) + self.c2 * y1 + self.c3 * y2
            }
            _ => input,
        };
        self.previous_output_2 = self.previous_output_1;
        self.previous_output_1 = Some(output);
        self.previous_input = Some(input);
        Some(output)
    }

    /// Return the latest filtered value.
    pub fn value(&self) -> Option<f64> {
        self.previous_output_1
    }

    /// Clear filter history while retaining coefficients.
    pub fn reset(&mut self) {
        self.previous_input = None;
        self.previous_output_1 = None;
        self.previous_output_2 = None;
    }
}

impl StreamingIndicator for SuperSmoother {
    type Output = f64;
    fn append(&mut self, input: f64) -> Option<f64> {
        Self::append(self, input)
    }
    fn value(&self) -> Option<f64> {
        self.value()
    }
    fn reset(&mut self) {
        Self::reset(self)
    }
}
