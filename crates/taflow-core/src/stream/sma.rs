//! Incremental Simple Moving Average (SMA).

use crate::error::TaResult;

use super::{StreamingIndicator, Window};

/// Computes an aligned Simple Moving Average vector.
pub fn simple_moving_average(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = SimpleMovingAverage::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value).unwrap_or(f64::NAN)).collect())
}

/// Stateful simple moving average with O(1) updates.
#[derive(Debug, Clone)]
pub struct SimpleMovingAverage {
    period: usize,
    window: Window,
    sum: f64,
    value: Option<f64>,
}

impl SimpleMovingAverage {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            period,
            window: Window::new(period)?,
            sum: 0.0,
            value: None,
        })
    }
}

impl StreamingIndicator for SimpleMovingAverage {
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
