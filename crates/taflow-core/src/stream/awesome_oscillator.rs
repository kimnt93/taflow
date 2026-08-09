//! Persistent Awesome Oscillator state.

use super::operator_states::{validate_period, ContiguousWindow};
use crate::error::{TaError, TaResult};

#[derive(Debug, Clone)]
pub struct AwesomeOscillator {
    fast: usize,
    slow: usize,
    values: ContiguousWindow,
    value: Option<f64>,
}

impl AwesomeOscillator {
    pub fn new(fast: usize, slow: usize) -> TaResult<Self> {
        validate_period(fast)?;
        validate_period(slow)?;
        if fast > slow {
            return Err(TaError::InvalidParameter {
                name: "fast/slow",
                value: format!("{fast}/{slow}"),
                reason: "fast must be <= slow",
            });
        }
        Ok(Self {
            fast,
            slow,
            values: ContiguousWindow::new(slow),
            value: None,
        })
    }

    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        self.values.push((high + low) * 0.5);
        self.value = self.values.is_full().then(|| {
            let window = self.values.window();
            let fast = window[self.slow - self.fast..].iter().rev().sum::<f64>() / self.fast as f64;
            let slow = window.iter().sum::<f64>() / self.slow as f64;
            fast - slow
        });
        self.value
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }

    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}
