//! Batch implementation for `chaikin_volatility`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes or updates `chaikin_volatility` through the native Rust kernel.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Compute the chaikin volatility result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
/// * `roc_period` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn chaikin_volatility(
    high: &[f64],
    low: &[f64],
    timeperiod: usize,
    roc_period: usize,
) -> TaResult<Vec<f64>> {
    if high.len() != low.len() {
        return Err(TaError::LengthMismatch {
            expected: high.len(),
            got: low.len(),
        });
    }
    let mut state = ChaikinVolatility::new(timeperiod, roc_period)?;
    Ok(high
        .iter()
        .zip(low)
        .map(|(&h, &l)| state.append(h, l).unwrap_or(f64::NAN))
        .collect())
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `ChaikinVolatility`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct ChaikinVolatility {
    period: usize,
    roc_period: usize,
    alpha: f64,
    ema: Option<f64>,
    history: VecDeque<f64>,
    value: Option<f64>,
}

impl ChaikinVolatility {
    /// Create a new empty state.
    ///
    pub fn new(period: usize, roc_period: usize) -> TaResult<Self> {
        validate_period(period)?;
        validate_period(roc_period)?;
        Ok(Self {
            period,
            roc_period,
            alpha: 2.0 / (period as f64 + 1.0),
            ema: None,
            history: VecDeque::with_capacity(roc_period + 1),
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        let range = high - low;
        let ema = self.ema.map_or(range, |v| v + self.alpha * (range - v));
        self.ema = Some(ema);
        if self.history.len() == self.roc_period + 1 {
            self.history.pop_front();
        }
        self.history.push_back(ema);
        self.value = (self.history.len() == self.roc_period + 1).then(|| {
            let old = self.history.front().copied().unwrap();
            if old != 0.0 {
                (ema - old) / old * 100.0
            } else {
                0.0
            }
        });
        self.value
    }
    /// Return the latest computed result, if warm-up is complete.
    ///
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Reset the state and clear its accumulated history.
    ///
    pub fn reset(&mut self) {
        self.ema = None;
        self.history.clear();
        self.value = None;
    }
}
