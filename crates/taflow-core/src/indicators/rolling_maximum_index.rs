//! Stateful trailing maximum-index indicator.

use crate::error::TaResult;

use crate::stream::rolling_extrema::{tracked_index_rescan_into, MonotonicArgmax};
use crate::stream::StreamingIndicator;

/// Persistent TA-Lib-compatible rolling maximum index.
#[derive(Debug, Clone)]
pub struct RollingMaximumIndex {
    extrema: MonotonicArgmax,
    value: Option<f64>,
}

impl RollingMaximumIndex {
    /// Create a rolling maximum-index state with a positive period.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            extrema: MonotonicArgmax::new(period)?,
            value: None,
        })
    }

    /// Append one observation and return its trailing maximum index.
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

impl StreamingIndicator for RollingMaximumIndex {
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
        let tracked = tracked_index_rescan_into::<true>(inputs, period, &mut output[start..]);
        self.extrema.rebuild_from_full_run(inputs, tracked);
        self.value = output.last().copied();
    }
}
