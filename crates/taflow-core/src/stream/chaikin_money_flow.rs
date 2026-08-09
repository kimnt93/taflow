//! Batch implementation for `chaikin_money_flow`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes the causal chaikin money flow series.
/// Parameters: aligned input slices followed by indicator parameters.
/// Compute the chaikin money flow result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `close` - Input series or configuration value.
/// * `volume` - Input series or configuration value.
/// * `period` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn chaikin_money_flow(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    volume: &[f64],
    period: usize,
) -> TaResult<Vec<f64>> {
    if high.len() != low.len() || high.len() != close.len() || high.len() != volume.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().min(close.len()).min(volume.len()),
        });
    }
    let mut state = ChaikinMoneyFlow::new(period)?;
    Ok(high
        .iter()
        .zip(low)
        .zip(close)
        .zip(volume)
        .map(|(((&h, &l), &c), &v)| state.append(h, l, c, v).unwrap_or(f64::NAN))
        .collect())
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// Stateful Chaikin Money Flow, aligned to `ta.volume.ChaikinMoneyFlowIndicator`.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `ChaikinMoneyFlow`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct ChaikinMoneyFlow {
    mfv: crate::stream::RollingSum,
    volume: crate::stream::RollingSum,
    value: Option<f64>,
}

impl ChaikinMoneyFlow {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            mfv: crate::stream::RollingSum::new(period)?,
            volume: crate::stream::RollingSum::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> Option<f64> {
        let multiplier = if high != low {
            ((close - low) - (high - close)) / (high - low)
        } else {
            0.0
        };
        let mfv = self.mfv.append(multiplier * volume);
        let volume_sum = self.volume.append(volume);
        self.value = match (mfv, volume_sum) {
            (Some(mfv), Some(volume)) if volume != 0.0 => Some(mfv / volume),
            (Some(_), Some(_)) => Some(0.0),
            _ => None,
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
        self.mfv.reset();
        self.volume.reset();
        self.value = None;
    }
}
