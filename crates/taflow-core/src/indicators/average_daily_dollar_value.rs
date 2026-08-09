pub struct AverageDailyDollarValue {
    sum: f64,
    window: VecDeque<f64>,
    timeperiod: usize,
    value: Option<f64>,
}

impl AverageDailyDollarValue {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self {
            sum: 0.0,
            window: VecDeque::with_capacity(timeperiod),
            timeperiod,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, close: f64, volume: f64) -> Option<f64> {
        let term = close * volume;
        if self.window.len() == self.timeperiod {
            self.sum -= self.window.pop_front().expect("ring is full");
        }
        self.window.push_back(term);
        self.sum += term;
        self.value = if self.window.len() == self.timeperiod {
            Some(self.sum / self.timeperiod as f64)
        } else {
            None
        };
        self.value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.sum = 0.0;
        self.window.clear();
        self.value = None;
    }
}
use crate::error::TaResult;
use crate::stream::operator_states::validate_period;
use std::collections::VecDeque;
