//! Stateful trailing minimum indicator.

use crate::error::TaResult;

use super::rolling_extrema::MonotonicMin;
use super::{vhgw, StreamingIndicator};

/// Persistent trailing minimum over a fixed number of observations.
#[derive(Debug, Clone)]
pub struct RollingMin {
    extrema: MonotonicMin,
    value: Option<f64>,
}

impl RollingMin {
    /// Create a rolling minimum state with the supplied positive period.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            extrema: MonotonicMin::new(period)?,
            value: None,
        })
    }

    /// Append one observation and return the trailing minimum once warm-up completes.
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

    /// Return the latest trailing minimum, or `None` during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Reset the state without reallocating its bounded deque.
    pub fn reset(&mut self) {
        self.extrema.reset();
        self.value = None;
    }
}

impl StreamingIndicator for RollingMin {
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
        vhgw::sliding_min_into(inputs, period, &mut output[start + period - 1..]);
        self.extrema.rebuild_from_full_run(inputs);
        self.value = output.last().copied();
    }
}
