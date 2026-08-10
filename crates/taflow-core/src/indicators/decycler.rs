use std::f64::consts::PI;

use crate::error::TaResult;
use crate::stream::{invalid_period, StreamingIndicator};

/// Ehlers trend component formed by subtracting a two-pole high-pass.
#[derive(Debug, Clone)]
pub struct Decycler {
    alpha: f64,
    previous_input_1: Option<f64>,
    previous_input_2: Option<f64>,
    previous_high_pass_1: f64,
    previous_high_pass_2: f64,
    value: Option<f64>,
}

impl Decycler {
    /// Create a decycler with a non-zero high-pass critical period.
    pub fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("period", period, 1));
        }
        let argument = 0.707 * 2.0 * PI / period as f64;
        let cosine = argument.cos();
        Ok(Self {
            alpha: (cosine + argument.sin() - 1.0) / cosine,
            previous_input_1: None,
            previous_input_2: None,
            previous_high_pass_1: 0.0,
            previous_high_pass_2: 0.0,
            value: None,
        })
    }

    /// Append one input and return price minus the current high-pass output.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        let high_pass = if let (Some(x1), Some(x2)) = (self.previous_input_1, self.previous_input_2)
        {
            let half = 1.0 - self.alpha * 0.5;
            let remaining = 1.0 - self.alpha;
            half * half * (input - 2.0 * x1 + x2) + 2.0 * remaining * self.previous_high_pass_1
                - remaining * remaining * self.previous_high_pass_2
        } else {
            0.0
        };
        self.previous_high_pass_2 = self.previous_high_pass_1;
        self.previous_high_pass_1 = high_pass;
        self.previous_input_2 = self.previous_input_1;
        self.previous_input_1 = Some(input);
        self.value = Some(input - high_pass);
        self.value
    }

    /// Return the latest decycled trend value.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clear input and high-pass history.
    pub fn reset(&mut self) {
        self.previous_input_1 = None;
        self.previous_input_2 = None;
        self.previous_high_pass_1 = 0.0;
        self.previous_high_pass_2 = 0.0;
        self.value = None;
    }
}

impl StreamingIndicator for Decycler {
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
