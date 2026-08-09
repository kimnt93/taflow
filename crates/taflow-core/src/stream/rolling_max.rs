//! Stateful trailing maximum indicator.

use crate::error::TaResult;

use super::rolling_extrema::MonotonicMax;
use super::{vhgw, StreamingIndicator};

/// Persistent trailing maximum over a fixed number of observations.
#[derive(Debug, Clone)]
pub struct RollingMax {
    extrema: MonotonicMax,
    value: Option<f64>,
}

impl RollingMax {
    /// Create a rolling maximum state with the supplied positive period.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            extrema: MonotonicMax::new(period)?,
            value: None,
        })
    }

    /// Append one observation and return the trailing maximum once warm-up completes.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self.extrema.append(input);
        self.value
    }

    pub(crate) fn period(&self) -> usize {
        self.extrema.period()
    }

    pub(crate) fn count(&self) -> usize {
        self.extrema.count()
    }

    /// Return the latest trailing maximum, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Reset the state without reallocating its bounded deque.
    pub fn reset(&mut self) {
        self.extrema.reset();
        self.value = None;
    }
}

impl StreamingIndicator for RollingMax {
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

    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        let period = self.extrema.period();
        if self.extrema.count() != 0 || inputs.len() < period {
            output.reserve(inputs.len());
            output.extend(
                inputs
                    .iter()
                    .copied()
                    .map(|input| self.append(input).unwrap_or(f64::NAN)),
            );
            return;
        }
        let start = output.len();
        output.resize(start + inputs.len(), f64::NAN);
        vhgw::sliding_max_into(inputs, period, &mut output[start + period - 1..]);
        self.extrema.rebuild_from_full_run(inputs);
        self.value = output.last().copied();
    }
}
