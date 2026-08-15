//! Persistent `SignalDelay` state.

use std::collections::VecDeque;

use crate::error::TaResult;
use crate::stream::{operator_states::validate_period, StreamingIndicator};

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `SignalDelay`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SignalDelay {
    values: VecDeque<f64>,
    period: usize,
    value: Option<f64>,
}

impl SignalDelay {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            values: VecDeque::with_capacity(period),
            period,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.value = if self.values.len() == self.period {
            let value = self.values.pop_front();
            self.values.push_back(input);
            value
        } else {
            self.values.push_back(input);
            None
        };
        self.value
    }

    /// Append a slice and write aligned values, using `NaN` during warm-up.
    ///
    /// The retained tail and new slice are treated as one logical sequence, so
    /// fresh, warmed, and chunked calls all avoid per-element state dispatch.
    pub fn extend_slice_into(&mut self, input: &[f64], output: &mut Vec<f64>) {
        if input.is_empty() {
            return;
        }
        let output_start = output.len();
        output.resize(output_start + input.len(), f64::NAN);

        let retained = self.values.len();
        let warmup = self.period - retained;
        let retained_output_start = warmup.min(input.len());
        let retained_outputs = input.len().min(self.period).saturating_sub(warmup);
        for (&value, output) in self
            .values
            .iter()
            .take(retained_outputs)
            .zip(&mut output[output_start + retained_output_start..])
        {
            *output = value;
        }
        if input.len() > self.period {
            output[output_start + self.period..]
                .copy_from_slice(&input[..input.len() - self.period]);
        }
        self.value =
            (retained + input.len() > self.period).then(|| output[output_start + input.len() - 1]);

        if input.len() >= self.period {
            self.values.clear();
            self.values.extend(&input[input.len() - self.period..]);
        } else {
            let overflow = (retained + input.len()).saturating_sub(self.period);
            self.values.drain(..overflow);
            self.values.extend(input);
        }
    }
    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}

impl StreamingIndicator for SignalDelay {
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
