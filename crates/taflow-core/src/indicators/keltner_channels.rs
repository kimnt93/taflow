//! Persistent `KeltnerChannels` state.

use crate::error::TaResult;
use crate::indicators::ExponentialMovingAverage;
use crate::stream::{operator_states::validate_period, StreamingIndicator};

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
    ema: ExponentialMovingAverage,
    previous_close: Option<f64>,
    atr_count: usize,
    atr_sum: f64,
    atr: Option<f64>,
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
            ema: ExponentialMovingAverage::new(period)?,
            previous_close: None,
            atr_count: 0,
            atr_sum: 0.0,
            atr: None,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<KeltnerValue> {
        let typical = (high + low + close) / 3.0;
        let middle = self.ema.append(typical);
        let true_range = self
            .previous_close
            .replace(close)
            .map_or(high - low, |previous| {
                (high - low)
                    .max((high - previous).abs())
                    .max((low - previous).abs())
            });
        self.atr_count += 1;
        if let Some(previous) = self.atr {
            let period = self.period as f64;
            self.atr = Some((previous * (period - 1.0) + true_range) / period);
        } else {
            self.atr_sum += true_range;
            if self.atr_count == self.period {
                self.atr = Some(self.atr_sum / self.period as f64);
            }
        }
        self.value = middle.zip(self.atr).map(|(middle, atr)| KeltnerValue {
            upper: middle + self.multiplier * atr,
            middle,
            lower: middle - self.multiplier * atr,
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
        self.ema.reset();
        self.previous_close = None;
        self.atr_count = 0;
        self.atr_sum = 0.0;
        self.atr = None;
        self.value = None;
    }
}
