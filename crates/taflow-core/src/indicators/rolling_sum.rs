use crate::error::TaResult;

use crate::stream::{StreamingIndicator, Window};

/// Stateful rolling sum.
///
/// The value is emitted after `period` observations and then updated by
/// subtracting the evicted observation and adding the current one.
///
/// # Parameters
///
/// * `period` - Number of observations in the trailing window.
///
/// # Returns
///
/// A state object whose latest value is `None` during warm-up and `Some(sum)`
/// once the window is full.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingSum`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingSum {
    window: Window,
    sum: f64,
    value: Option<f64>,
}

impl RollingSum {
    /// Create an empty rolling-sum state for the requested window length.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            window: Window::new(period)?,
            sum: 0.0,
            value: None,
        })
    }
}

impl StreamingIndicator for RollingSum {
    type Output = f64;

    /// Append one observation and return the current sum when warm.
    fn append(&mut self, input: f64) -> Option<f64> {
        if let Some(old) = self.window.push(input) {
            self.sum -= old;
        }
        self.sum += input;
        self.value = self.window.is_full().then_some(self.sum);
        self.value
    }

    /// Return the most recently computed rolling sum.
    fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clear the window and accumulated sum.
    fn reset(&mut self) {
        self.window.clear();
        self.sum = 0.0;
        self.value = None;
    }
}
