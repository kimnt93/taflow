//! Batch implementation for `average_true_range`.

use super::aroon_true_range::*;
use crate::error::{TaError, TaResult};

/// Compute the average true range result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `close` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn average_true_range(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    timeperiod: usize,
) -> TaResult<Vec<f64>> {
    if high.len() != low.len() || high.len() != close.len() {
        return Err(crate::TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().min(close.len()),
        });
    }
    let mut state = AverageTrueRange::new(timeperiod)?;
    Ok(high
        .iter()
        .zip(low)
        .zip(close)
        .map(|((high, low), close)| state.append(*high, *low, *close).unwrap_or(f64::NAN))
        .collect())
}
use super::*;

/// Stateful Average True Range.  Each appended bar is `(high, low, close)`.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `AverageTrueRange`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct AverageTrueRange {
    period: usize,
    previous_close: Option<f64>,
    tr_count: usize,
    tr_sum: f64,
    value: Option<f64>,
}

impl AverageTrueRange {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("timeperiod", period, 1));
        }
        Ok(Self {
            period,
            previous_close: None,
            tr_count: 0,
            tr_sum: 0.0,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let Some(previous_close) = self.previous_close.replace(close) else {
            return None;
        };
        let true_range = (high - low)
            .max((high - previous_close).abs())
            .max((low - previous_close).abs());
        self.tr_count += 1;

        if self.tr_count < self.period {
            self.tr_sum += true_range;
            return None;
        }

        if self.tr_count == self.period {
            self.value = Some((self.tr_sum + true_range) / self.period as f64);
        } else if let Some(previous) = self.value {
            let period = self.period as f64;
            self.value = Some((previous * (period - 1.0) + true_range) / period);
        }
        self.value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.tr_count = 0;
        self.tr_sum = 0.0;
        self.value = None;
    }
}
