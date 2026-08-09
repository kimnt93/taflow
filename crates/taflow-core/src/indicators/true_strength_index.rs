//! Persistent true strength index state.

use crate::error::TaResult;
use crate::stream::operator_states::validate_period;

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `TrueStrengthIndex`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct TrueStrengthIndex {
    previous: Option<f64>,
    fast: usize,
    slow: usize,
    alpha_fast: f64,
    alpha_slow: f64,
    momentum: Option<f64>,
    absolute: Option<f64>,
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
            fast,
            slow,
            alpha_fast: 2.0 / (fast as f64 + 1.0),
            alpha_slow: 2.0 / (slow as f64 + 1.0),
            momentum: None,
            absolute: None,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, input: f64) -> Option<f64> {
        let previous = self.previous.replace(input)?;
        let change = input - previous;
        let abs = change.abs();
        let m1 = self
            .momentum
            .map_or(change, |v| v + self.alpha_fast * (change - v));
        let a1 = self
            .absolute
            .map_or(abs, |v| v + self.alpha_fast * (abs - v));
        self.momentum = Some(m1);
        self.absolute = Some(a1);
        let m2 = self.momentum.map_or(m1, |v| v + self.alpha_slow * (m1 - v));
        let a2 = self.absolute.map_or(a1, |v| v + self.alpha_slow * (a1 - v));
        let value = if a2 != 0.0 {
            Some(100.0 * m2 / a2)
        } else {
            Some(0.0)
        };
        self.value = value;
        value
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
        self.momentum = None;
        self.absolute = None;
        self.value = None;
    }
}
