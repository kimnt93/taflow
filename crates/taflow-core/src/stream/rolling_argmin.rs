//! Stateful trailing minimum-index indicator.

use crate::error::TaResult;

use super::rolling_extrema::{tracked_index_rescan_into, MonotonicArgmin};
use super::StreamingIndicator;

/// Persistent TA-Lib-compatible rolling minimum index.
#[derive(Debug, Clone)]
pub struct RollingArgmin {
    extrema: MonotonicArgmin,
    value: Option<f64>,
}

impl RollingArgmin {
    /// Create a rolling minimum-index state with a positive period.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            extrema: MonotonicArgmin::new(period)?,
            value: None,
        })
    }

    /// Append one observation and return its trailing minimum index.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.value = Some(self.extrema.append(input).unwrap_or(0) as f64);
        self.value
    }

    /// Return the latest index, including zero during warm-up.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Reset the state.
    pub fn reset(&mut self) {
        self.extrema.reset();
        self.value = None;
    }
}

impl StreamingIndicator for RollingArgmin {
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
            output.extend(
                inputs
                    .iter()
                    .copied()
                    .map(|input| self.append(input).unwrap_or(0.0)),
            );
            return;
        }
        let start = output.len();
        output.resize(start + inputs.len(), 0.0);
        let tracked = tracked_index_rescan_into::<false>(inputs, period, &mut output[start..]);
        self.extrema.rebuild_from_full_run(inputs, tracked);
        self.value = output.last().copied();
    }
}
