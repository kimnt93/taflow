//! Persistent `KeltnerChannels` state.

use super::*;
use crate::error::{TaError, TaResult};
use crate::stream::operator_states::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `KeltnerValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct KeltnerValue {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `KeltnerChannels`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct KeltnerChannels {
    period: usize,
    multiplier: f64,
    ema: Option<f64>,
    range_ema: Option<f64>,
    alpha: f64,
    value: Option<KeltnerValue>,
}

impl KeltnerChannels {
    /// Create a new empty state.
    ///
    pub fn new(period: usize, multiplier: f64) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            period,
            multiplier,
            ema: None,
            range_ema: None,
            alpha: 2.0 / (period as f64 + 1.0),
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<KeltnerValue> {
        let typical = (high + low + close) / 3.0;
        let range = high - low;
        let ema = self.ema.map_or(typical, |v| v + self.alpha * (typical - v));
        let re = self
            .range_ema
            .map_or(range, |v| v + self.alpha * (range - v));
        self.ema = Some(ema);
        self.range_ema = Some(re);
        self.value = Some(KeltnerValue {
            upper: ema + self.multiplier * re,
            middle: ema,
            lower: ema - self.multiplier * re,
        });
        self.value
    }
    /// Return the latest computed result, if warm-up is complete.
    ///
    pub fn value(&self) -> Option<KeltnerValue> {
        self.value
    }
    /// Reset the state and clear its accumulated history.
    ///
    pub fn reset(&mut self) {
        self.ema = None;
        self.range_ema = None;
        self.value = None;
    }
}
