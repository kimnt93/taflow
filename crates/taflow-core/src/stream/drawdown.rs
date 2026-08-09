//! Persistent percentage drawdown state.

use super::cumulative_maximum::CumulativeMaximum;

#[derive(Debug, Clone)]
pub struct Drawdown {
    maximum: CumulativeMaximum,
    value: Option<f64>,
}

impl Drawdown {
    pub fn new() -> Self {
        Self {
            maximum: CumulativeMaximum::default(),
            value: None,
        }
    }

    pub fn append(&mut self, input: f64) -> f64 {
        let maximum = self.maximum.append(input);
        let value = if maximum != 0.0 {
            input / maximum - 1.0
        } else {
            0.0
        };
        self.value = Some(value);
        value
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }

    pub fn reset(&mut self) {
        self.maximum.reset();
        self.value = None;
    }
}

impl Default for Drawdown {
    fn default() -> Self {
        Self::new()
    }
}
