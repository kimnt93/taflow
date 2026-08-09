//! Stateful Chande Momentum Oscillator.
//!
//! CMO separates positive and negative price changes, seeds both Wilder sums
//! from the first `timeperiod` changes, and then updates them in constant time.

use crate::error::TaResult;

use crate::stream::{invalid_period, StreamingIndicator};

/// Incremental Chande Momentum Oscillator with TA-Lib-compatible warm-up.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `ChandeMomentumOscillator`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct ChandeMomentumOscillator {
    period: usize,
    previous_input: Option<f64>,
    changes: usize,
    sum_up: f64,
    sum_down: f64,
    value: Option<f64>,
}

impl ChandeMomentumOscillator {
    /// Creates a CMO state with a period of at least two changes.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            period,
            previous_input: None,
            changes: 0,
            sum_up: 0.0,
            sum_down: 0.0,
            value: None,
        })
    }

    fn oscillator(&self) -> f64 {
        let total = self.sum_up + self.sum_down;
        if total > 0.0 {
            100.0 * (self.sum_up - self.sum_down) / total
        } else {
            0.0
        }
    }

    /// Appends one chronological value and returns the latest oscillator.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        let Some(previous) = self.previous_input.replace(input) else {
            return None;
        };
        let change = input - previous;
        let (up, down) = if change > 0.0 {
            (change, 0.0)
        } else {
            (0.0, -change)
        };
        self.changes += 1;
        if self.changes <= self.period {
            self.sum_up += up;
            self.sum_down += down;
            if self.changes < self.period {
                return None;
            }
        } else {
            let period = self.period as f64;
            self.sum_up = self.sum_up - self.sum_up / period + up;
            self.sum_down = self.sum_down - self.sum_down / period + down;
        }
        self.value = Some(self.oscillator());
        self.value
    }

    /// Extends the state with one aligned input slice and warm-up NaNs.
    pub fn extend_slice_into(&mut self, input: &[f64], output: &mut Vec<f64>) {
        output.reserve(input.len());
        output.extend(
            input
                .iter()
                .map(|&value| self.append(value).unwrap_or(f64::NAN)),
        );
    }

    /// Returns the latest oscillator after warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Restores fresh-state behavior without reallocating.
    pub fn reset(&mut self) {
        self.previous_input = None;
        self.changes = 0;
        self.sum_up = 0.0;
        self.sum_down = 0.0;
        self.value = None;
    }
}

impl StreamingIndicator for ChandeMomentumOscillator {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        Self::append(self, input)
    }

    fn value(&self) -> Option<f64> {
        Self::value(self)
    }

    fn reset(&mut self) {
        Self::reset(self)
    }
}
