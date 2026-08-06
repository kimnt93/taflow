//! Incremental Simple Moving Average (SMA).

use crate::error::TaResult;

use super::{StreamingIndicator, Window};

/// Stateful simple moving average with O(1) updates.
#[derive(Debug, Clone)]
pub struct Sma {
    period: usize,
    window: Window,
    sum: f64,
    value: Option<f64>,
}

impl Sma {
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            period,
            window: Window::new(period)?,
            sum: 0.0,
            value: None,
        })
    }
}

impl StreamingIndicator for Sma {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        if let Some(old) = self.window.push(input) {
            self.sum -= old;
        }
        self.sum += input;
        self.value = self.window.is_full().then(|| self.sum / self.period as f64);
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.window.clear();
        self.sum = 0.0;
        self.value = None;
    }
}
