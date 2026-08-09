//! Batch implementation for `detrended_price_oscillator`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes the causal detrended price oscillator series.
/// Parameters: aligned input slices followed by indicator parameters.
/// Compute the detrended price oscillator result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `period` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn detrended_price_oscillator(input: &[f64], period: usize) -> TaResult<Vec<f64>> {
    let mut state = DetrendedPriceOscillator::new(period)?;
    Ok(input
        .iter()
        .map(|&value| state.append(value).unwrap_or(f64::NAN))
        .collect())
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// Stateful causal Detrended Price Oscillator. The centered pandas-ta form is
/// intentionally excluded because it shifts future values backward.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `DetrendedPriceOscillator`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct DetrendedPriceOscillator {
    sma: SimpleMovingAverage,
    delay: Window,
    value: Option<f64>,
}

impl DetrendedPriceOscillator {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            sma: SimpleMovingAverage::new(period)?,
            delay: Window::new(period / 2 + 1)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, close: f64) -> Option<f64> {
        self.value = self
            .sma
            .append(close)
            .and_then(|mean| self.delay.push(mean).map(|delayed| close - delayed));
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
        self.sma.reset();
        self.delay.clear();
        self.value = None;
    }
}
