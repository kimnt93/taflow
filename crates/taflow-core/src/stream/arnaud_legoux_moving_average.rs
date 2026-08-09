//! Batch implementation for `arnaud_legoux_moving_average`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes the causal arnaud legoux moving average series.
/// Parameters: aligned input slices followed by indicator parameters.
/// Compute the arnaud legoux moving average result for the supplied aligned series.
///
/// # Parameters
///
/// * `input` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
/// * `offset` - Input series or configuration value.
/// * `sigma` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn arnaud_legoux_moving_average(
    input: &[f64],
    timeperiod: usize,
    offset: f64,
    sigma: f64,
) -> TaResult<Vec<f64>> {
    let mut state = ArnaudLegouxMovingAverage::new(timeperiod, offset, sigma)?;
    Ok(input
        .iter()
        .map(|&v| state.append(v).unwrap_or(f64::NAN))
        .collect())
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `ArnaudLegouxMovingAverage`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct ArnaudLegouxMovingAverage {
    values: VecDeque<f64>,
    period: usize,
    weights: Vec<f64>,
    value: Option<f64>,
}

impl ArnaudLegouxMovingAverage {
    /// Create a new empty state.
    ///
    pub fn new(period: usize, offset: f64, sigma: f64) -> TaResult<Self> {
        validate_period(period)?;
        if !(0.0..=1.0).contains(&offset) || sigma <= 0.0 {
            return Err(TaError::InvalidParameter {
                name: "offset/sigma",
                value: format!("{offset}/{sigma}"),
                reason: "offset must be 0..1 and sigma must be positive",
            });
        }
        let m = offset * (period - 1) as f64;
        let weights = (0..period)
            .map(|i| {
                ((-(i as f64 - m).powi(2) / (2.0 * sigma.powi(2) * (period as f64).powi(2))).exp())
            })
            .collect();
        Ok(Self {
            values: VecDeque::with_capacity(period),
            period,
            weights,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.period {
            self.values.pop_front();
        }
        self.values.push_back(input);
        self.value = (self.values.len() == self.period).then(|| {
            let total = self.weights.iter().sum::<f64>();
            self.values
                .iter()
                .zip(&self.weights)
                .map(|(&v, &w)| v * w)
                .sum::<f64>()
                / total
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
        self.values.clear();
        self.value = None;
    }
}
