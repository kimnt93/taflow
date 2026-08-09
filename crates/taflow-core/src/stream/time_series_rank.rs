//! Persistent WorldQuant Alpha101 time-series rank state.

use super::{RollingRank, StreamingIndicator};
use crate::error::TaResult;

/// Rank of each value within a trailing chronological window.
#[derive(Debug, Clone)]
pub struct TimeSeriesRank {
    inner: RollingRank,
}

impl TimeSeriesRank {
    /// Construct a validated trailing-rank state.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            inner: RollingRank::new(timeperiod)?,
        })
    }

    /// Extend the state with chronological values and aligned warm-up output.
    pub fn extend_slice_into(&mut self, input: &[f64], output: &mut Vec<f64>) {
        self.inner.extend_slice_into(input, output);
    }
}

impl StreamingIndicator for TimeSeriesRank {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        self.inner.append(input)
    }

    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}
