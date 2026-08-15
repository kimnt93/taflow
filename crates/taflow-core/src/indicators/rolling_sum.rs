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
    period: usize,
    window: Window,
    sum: f64,
    value: Option<f64>,
}

impl RollingSum {
    /// Create an empty rolling-sum state for the requested window length.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            period,
            window: Window::new(period)?,
            sum: 0.0,
            value: None,
        })
    }
}

impl StreamingIndicator for RollingSum {
    type Output = f64;

    /// Append a slice with a direct steady-state add/evict recurrence.
    ///
    /// At most one window-length prologue uses [`Self::append`] so an already
    /// warmed state is displaced in exactly scalar order. The remaining
    /// evictions come directly from `inputs`, and the bounded ring is rebuilt
    /// once for exact subsequent continuation.
    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        let n = inputs.len();
        if n == 0 {
            return;
        }
        let period = self.period;
        output.reserve(n);

        let prologue = n.min(period);
        for &input in &inputs[..prologue] {
            output.push(self.append(input).unwrap_or(f64::NAN));
        }
        if n <= period {
            return;
        }

        let mut sum = self.sum;
        for index in period..n {
            sum -= inputs[index - period];
            sum += inputs[index];
            output.push(sum);
        }

        self.window.clear();
        for &input in &inputs[n - period..] {
            self.window.push(input);
        }
        self.sum = sum;
        self.value = Some(sum);
    }

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
