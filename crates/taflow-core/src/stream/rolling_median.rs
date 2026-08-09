//! Rolling median state.

use super::operator_states::validate_period;
use super::sorted_ring::SortedRing;
use super::StreamingIndicator;
use crate::TaResult;

/// Computes the causal median over a fixed trailing window.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingMedian`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingMedian {
    window: SortedRing,
    timeperiod: usize,
    value: Option<f64>,
}

impl RollingMedian {
    /// Creates an empty rolling-median state.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self {
            window: SortedRing::new(timeperiod),
            timeperiod,
            value: None,
        })
    }

    /// Appends one observation and returns the median after warm-up.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.window.push(input);
        self.value = if self.window.is_full() {
            let sorted = self.window.sorted();
            let middle = self.timeperiod / 2;
            Some(if self.timeperiod % 2 == 1 {
                sorted[middle]
            } else {
                (sorted[middle - 1] + sorted[middle]) * 0.5
            })
        } else {
            None
        };
        self.value
    }

    /// Extends the state with one aligned slice and NaN warm-up values.
    pub fn extend_slice_into(&mut self, input: &[f64], output: &mut Vec<f64>) {
        output.reserve(input.len());
        output.extend(
            input
                .iter()
                .map(|&value| self.append(value).unwrap_or(f64::NAN)),
        );
    }

    /// Returns the latest median, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clears the trailing window and latest output.
    pub fn reset(&mut self) {
        self.window.clear();
        self.value = None;
    }
}

impl StreamingIndicator for RollingMedian {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        Self::append(self, input)
    }

    fn value(&self) -> Option<f64> {
        Self::value(self)
    }

    fn reset(&mut self) {
        Self::reset(self);
    }

    fn extend_slice_into(&mut self, input: &[f64], output: &mut Vec<f64>) {
        Self::extend_slice_into(self, input, output);
    }
}
