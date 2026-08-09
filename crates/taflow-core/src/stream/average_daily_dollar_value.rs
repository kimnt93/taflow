//! Batch implementation for `average_daily_dollar_value`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes the causal average daily dollar value series.
/// Parameters: aligned input slices followed by indicator parameters.
/// Compute the average daily dollar value result for the supplied aligned series.
///
/// # Parameters
///
/// * `close` - Input series or configuration value.
/// * `volume` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn average_daily_dollar_value(
    close: &[f64],
    volume: &[f64],
    timeperiod: usize,
) -> TaResult<Vec<f64>> {
    if close.len() != volume.len() {
        return Err(TaError::LengthMismatch {
            expected: close.len(),
            got: volume.len(),
        });
    }
    let mut state = AverageDailyDollarValue::new(timeperiod)?;
    Ok(close
        .iter()
        .zip(volume)
        .map(|(&close, &volume)| state.append(close, volume).unwrap_or(f64::NAN))
        .collect())
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// Average daily dollar value traded: SMA of `close × volume`.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `AverageDailyDollarValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct AverageDailyDollarValue {
    sum: f64,
    window: VecDeque<f64>,
    timeperiod: usize,
    value: Option<f64>,
}

impl AverageDailyDollarValue {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self {
            sum: 0.0,
            window: VecDeque::with_capacity(timeperiod),
            timeperiod,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, close: f64, volume: f64) -> Option<f64> {
        let term = close * volume;
        if self.window.len() == self.timeperiod {
            self.sum -= self.window.pop_front().expect("ring is full");
        }
        self.window.push_back(term);
        self.sum += term;
        self.value = if self.window.len() == self.timeperiod {
            Some(self.sum / self.timeperiod as f64)
        } else {
            None
        };
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

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.sum = 0.0;
        self.window.clear();
        self.value = None;
    }
}
