//! Causal delay-line state.

use std::collections::VecDeque;

use crate::stream::{operator_states::validate_period, StreamingIndicator};
use crate::TaResult;

/// Delays a scalar series by a fixed number of bars.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Lag`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Lag {
    values: VecDeque<f64>,
    timeperiod: usize,
    value: Option<f64>,
}

impl Lag {
    /// Creates an empty delay line.
    ///
    /// `timeperiod` is the number of bars before the first value is emitted.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self {
            values: VecDeque::with_capacity(timeperiod),
            timeperiod,
            value: None,
        })
    }

    /// Appends one observation and returns the value from `timeperiod` bars ago.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.value = if self.values.len() == self.timeperiod {
            let value = self.values.pop_front().expect("lag window is full");
            self.values.push_back(input);
            Some(value)
        } else {
            self.values.push_back(input);
            None
        };
        self.value
    }

    /// Append a slice into `output` with `NaN` at warm-up positions.
    pub fn extend_slice_into(&mut self, input: &[f64], output: &mut Vec<f64>) {
        if input.is_empty() {
            return;
        }
        let output_start = output.len();
        output.resize(output_start + input.len(), f64::NAN);

        let retained = self.values.len();
        let warmup = self.timeperiod - retained;
        let retained_output_start = warmup.min(input.len());
        let retained_outputs = input.len().min(self.timeperiod).saturating_sub(warmup);
        for (&value, output) in self
            .values
            .iter()
            .take(retained_outputs)
            .zip(&mut output[output_start + retained_output_start..])
        {
            *output = value;
        }
        if input.len() > self.timeperiod {
            output[output_start + self.timeperiod..]
                .copy_from_slice(&input[..input.len() - self.timeperiod]);
        }
        self.value = (retained + input.len() > self.timeperiod)
            .then(|| output[output_start + input.len() - 1]);

        if input.len() >= self.timeperiod {
            self.values.clear();
            self.values.extend(&input[input.len() - self.timeperiod..]);
        } else {
            let overflow = (retained + input.len()).saturating_sub(self.timeperiod);
            self.values.drain(..overflow);
            self.values.extend(input);
        }
    }

    /// Returns the latest delayed value, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Clears buffered observations and the latest value.
    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}

impl StreamingIndicator for Lag {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<Self::Output> {
        Self::append(self, input)
    }

    fn value(&self) -> Option<Self::Output> {
        Self::value(self)
    }

    fn reset(&mut self) {
        Self::reset(self);
    }
}
