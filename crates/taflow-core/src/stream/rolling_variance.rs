//! Batch implementation for `rolling_var`.

use super::statistic::*;
use crate::error::{TaError, TaResult};

/// Compute the rolling var result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
use super::rolling_statistics::*;
use super::*;

/// Stateful population variance. TA-Lib accepts but ignores `nbdev` for VAR.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingVariance`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingVariance {
    moments: RollingMoments,
    value: Option<f64>,
}

impl RollingVariance {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize, _nbdev: f64) -> TaResult<Self> {
        Ok(Self {
            moments: RollingMoments::new(period)?,
            value: None,
        })
    }
}

impl StreamingIndicator for RollingVariance {
    type Output = f64;

    /// Bulk kernel: slice-recurrence sliding moments, bit-identical to
    /// per-bar [`Self::append`] in outputs and post-run state.
    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        if inputs.is_empty() {
            return;
        }
        self.value = self
            .moments
            .extend_map_into(inputs, output, |variance| variance);
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self.moments.append(input);
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.moments.reset();
        self.value = None;
    }
}
