//! Batch implementation for `rolling_std`.

use super::statistic::*;
use crate::error::{TaError, TaResult};

/// Standard Deviation (STDDEV) — fused single-pass var+sqrt
///
/// Eliminates intermediate var Vec (8MB at 1M bars) by computing
/// Compute the rolling std result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
/// * `nbdev` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
use super::rolling_statistics::*;
use super::*;

/// Stateful population standard deviation multiplied by `nbdev`.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingStandardDeviation`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingStandardDeviation {
    moments: RollingMoments,
    nbdev: f64,
    value: Option<f64>,
}

impl RollingStandardDeviation {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize, nbdev: f64) -> TaResult<Self> {
        Ok(Self {
            moments: RollingMoments::new(period)?,
            nbdev,
            value: None,
        })
    }
}

impl StreamingIndicator for RollingStandardDeviation {
    type Output = f64;

    /// Bulk kernel: slice-recurrence sliding moments, bit-identical to
    /// per-bar [`Self::append`] in outputs and post-run state.
    fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        if inputs.is_empty() {
            return;
        }
        let nbdev = self.nbdev;
        self.value = self.moments.extend_map_into(inputs, output, |variance| {
            stddev_from_variance(variance, nbdev)
        });
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        let nbdev = self.nbdev;
        self.value = self
            .moments
            .append(input)
            .map(|variance| stddev_from_variance(variance, nbdev));
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
