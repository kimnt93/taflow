//! Incremental Weighted Moving Average (WMA).

use crate::error::TaResult;

use super::{StreamingIndicator, Window};

/// Computes an aligned Weighted Moving Average vector.
pub fn weighted_moving_average(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let mut state = WeightedMovingAverage::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value).unwrap_or(f64::NAN)).collect())
}

/// Stateful weighted moving average with O(1) updates.
#[derive(Debug, Clone)]
pub struct WeightedMovingAverage {
    period: usize,
    divider: f64,
    window: Window,
    sum: f64,
    weighted_sum: f64,
    value: Option<f64>,
}

impl WeightedMovingAverage {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        let window = Window::new(period)?;
        let period_f = period as f64;
        Ok(Self {
            period,
            divider: period_f * (period_f + 1.0) / 2.0,
            window,
            sum: 0.0,
            weighted_sum: 0.0,
            value: None,
        })
    }
}

impl StreamingIndicator for WeightedMovingAverage {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        if self.window.is_full() {
            let previous_sum = self.sum;
            let old = self
                .window
                .push(input)
                .expect("a full window evicts one value");
            self.weighted_sum += self.period as f64 * input - previous_sum;
            self.sum += input - old;
        } else {
            let weight = self.window.len() + 1;
            self.window.push(input);
            self.sum += input;
            self.weighted_sum += input * weight as f64;
        }
        self.value = self
            .window
            .is_full()
            .then(|| self.weighted_sum / self.divider);
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.window.clear();
        self.sum = 0.0;
        self.weighted_sum = 0.0;
        self.value = None;
    }
}
