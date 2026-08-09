//! Stateful selectable moving average.
//!
//! MA exposes TA-Lib's moving-average type selector over the nine incremental
//! implementations while retaining the selected type's native warm-up.

use crate::error::TaResult;
use crate::ma_type::MaType;

use crate::stream::{moving_average_dispatcher::MovingAverageDispatcher, StreamingIndicator};

/// Incremental moving average selected by [`MaType`].
/// Persistent Rust state or aligned output type for `MovingAverage`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct MovingAverage {
    inner: MovingAverageDispatcher,
    value: Option<f64>,
}

impl MovingAverage {
    /// Creates a selectable moving average with TA-Lib-compatible defaults at
    /// the binding layer.
    pub fn new(period: usize, ma_type: MaType) -> TaResult<Self> {
        Ok(Self {
            inner: MovingAverageDispatcher::new(period, ma_type)?,
            value: None,
        })
    }
}

impl StreamingIndicator for MovingAverage {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self.inner.append(input);
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.value = None;
    }

    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        self.inner.extend_slice_into(inputs, output);
        self.value = output.last().copied().filter(|value| !value.is_nan());
    }
}
