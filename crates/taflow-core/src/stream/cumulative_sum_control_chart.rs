//! Batch implementation for `cumulative_sum_control_chart`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes the cumulative-sum control-chart signal.
///
/// Parameters are the typed series and configuration values in the signature.
///
/// Returns the computed value, aligned history, or a validation error.
pub fn cumulative_sum_control_chart(input: &[f64], threshold: f64) -> TaResult<Vec<f64>> {
    let mut state = CumulativeSumControlChart::new(threshold)?;
    Ok(input.iter().map(|&change| state.append(change)).collect())
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// CUSUM event flags (AFML §2.5.2): `+1` when the cumulative deviation from
/// `threshold` (daily volatility) exceeds it, `-1` on the downside, else `0`.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `CumulativeSumControlChart`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CumulativeSumControlChart {
    threshold: f64,
    s_positive: f64,
    s_negative: f64,
    value: Option<f64>,
}

impl CumulativeSumControlChart {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(threshold: f64) -> TaResult<Self> {
        if threshold < 0.0 {
            return Err(TaError::InvalidParameter {
                name: "threshold",
                value: threshold.to_string(),
                reason: "must be >= 0",
            });
        }
        Ok(Self {
            threshold,
            s_positive: 0.0,
            s_negative: 0.0,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, change: f64) -> f64 {
        self.s_positive = (self.s_positive + change).max(0.0);
        self.s_negative = (self.s_negative - change).max(0.0);
        let flag = if self.s_positive > self.threshold {
            self.s_positive = 0.0;
            1.0
        } else if self.s_negative > self.threshold {
            self.s_negative = 0.0;
            -1.0
        } else {
            0.0
        };
        self.value = Some(flag);
        flag
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
        self.s_positive = 0.0;
        self.s_negative = 0.0;
        self.value = None;
    }
}
