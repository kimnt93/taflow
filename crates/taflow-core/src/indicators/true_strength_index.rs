//! Persistent true strength index state.

use crate::error::TaResult;
use crate::indicators::ExponentialMovingAverage;
use crate::stream::{operator_states::validate_period, StreamingIndicator};

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `TrueStrengthIndex`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct TrueStrengthIndex {
    previous: Option<f64>,
    long_momentum: ExponentialMovingAverage,
    short_momentum: ExponentialMovingAverage,
    long_absolute: ExponentialMovingAverage,
    short_absolute: ExponentialMovingAverage,
    value: Option<f64>,
}

impl TrueStrengthIndex {
    /// Create a new empty state.
    ///
    pub fn new(fast: usize, slow: usize) -> TaResult<Self> {
        validate_period(fast)?;
        validate_period(slow)?;
        Ok(Self {
            previous: None,
            long_momentum: ExponentialMovingAverage::new(slow)?,
            short_momentum: ExponentialMovingAverage::new(fast)?,
            long_absolute: ExponentialMovingAverage::new(slow)?,
            short_absolute: ExponentialMovingAverage::new(fast)?,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, input: f64) -> Option<f64> {
        let previous = self.previous.replace(input)?;
        let change = input - previous;
        let momentum = self
            .long_momentum
            .append(change)
            .and_then(|value| self.short_momentum.append(value));
        let absolute = self
            .long_absolute
            .append(change.abs())
            .and_then(|value| self.short_absolute.append(value));
        self.value = momentum.zip(absolute).map(|(momentum, absolute)| {
            if absolute == 0.0 {
                0.0
            } else {
                100.0 * momentum / absolute
            }
        });
        self.value
    }
    /// Return the latest computed result, if warm-up is complete.
    ///
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Reset the state and clear its accumulated history.
    ///
    pub fn reset(&mut self) {
        self.previous = None;
        self.long_momentum.reset();
        self.short_momentum.reset();
        self.long_absolute.reset();
        self.short_absolute.reset();
        self.value = None;
    }
}
