//! Stateful volume and volume-derived streaming indicators.

use crate::error::{TaError, TaResult};

use super::{invalid_period, RollingExtrema};

pub(crate) fn ad_increment(high: f64, low: f64, close: f64, volume: f64) -> f64 {
    let range = high - low;
    if range > 0.0 {
        ((close - low) - (high - close)) / range * volume
    } else {
        0.0
    }
}

/// Stateful Chaikin accumulation/distribution line.
#[derive(Debug, Clone, Default)]
/// Persistent Rust state or aligned output type for `AccumulationDistribution`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct AccumulationDistribution {
    total: f64,
    value: Option<f64>,
}

impl AccumulationDistribution {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self::default()
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> f64 {
        self.total += ad_increment(high, low, close, volume);
        self.value = Some(self.total);
        self.total
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
        self.total = 0.0;
        self.value = None;
    }
}

/// Stateful Chaikin A/D oscillator with first-value EMA seeds.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `AccumulationDistributionOscillator`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct AccumulationDistributionOscillator {
    lookback: usize,
    index: usize,
    fast_k: f64,
    slow_k: f64,
    ad: f64,
    fast_ema: Option<f64>,
    slow_ema: Option<f64>,
    value: Option<f64>,
}

impl AccumulationDistributionOscillator {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(fast_period: usize, slow_period: usize) -> TaResult<Self> {
        if fast_period < 2 || slow_period < 2 {
            return Err(TaError::InvalidParameter {
                name: "fastperiod/slowperiod",
                value: format!("{fast_period}/{slow_period}"),
                reason: "both periods must be >= 2",
            });
        }
        Ok(Self {
            lookback: fast_period.max(slow_period) - 1,
            index: 0,
            fast_k: 2.0 / (fast_period as f64 + 1.0),
            slow_k: 2.0 / (slow_period as f64 + 1.0),
            ad: 0.0,
            fast_ema: None,
            slow_ema: None,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> Option<f64> {
        self.ad += ad_increment(high, low, close, volume);
        match (self.fast_ema, self.slow_ema) {
            (Some(fast), Some(slow)) => {
                self.fast_ema = Some(self.fast_k.mul_add(self.ad - fast, fast));
                self.slow_ema = Some(self.slow_k.mul_add(self.ad - slow, slow));
            }
            _ => {
                self.fast_ema = Some(self.ad);
                self.slow_ema = Some(self.ad);
            }
        }
        if self.index >= self.lookback {
            self.value = Some(
                self.fast_ema.expect("fast EMA is initialized")
                    - self.slow_ema.expect("slow EMA is initialized"),
            );
        }
        self.index += 1;
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
        self.index = 0;
        self.ad = 0.0;
        self.fast_ema = None;
        self.slow_ema = None;
        self.value = None;
    }
}

/// Stateful on-balance volume.
#[derive(Debug, Clone, Default)]
/// Persistent Rust state or aligned output type for `OnBalanceVolume`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct OnBalanceVolume {
    previous_close: Option<f64>,
    total: f64,
    value: Option<f64>,
}

impl OnBalanceVolume {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self::default()
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, close: f64, volume: f64) -> f64 {
        match self.previous_close.replace(close) {
            None => self.total = volume,
            Some(previous) if close > previous => self.total += volume,
            Some(previous) if close < previous => self.total -= volume,
            Some(_) => {}
        }
        self.value = Some(self.total);
        self.total
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
        self.total = 0.0;
        self.value = None;
    }
}

/// Stateful balance of power.
#[derive(Debug, Clone, Default)]
/// Persistent Rust state or aligned output type for `BalanceOfPower`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct BalanceOfPower {
    value: Option<f64>,
}

impl BalanceOfPower {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self::default()
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> f64 {
        let range = high - low;
        let value = if range > 0.0 {
            (close - open) / range
        } else {
            0.0
        };
        self.value = Some(value);
        value
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
        self.value = None;
    }
}

/// Stateful Williams %R.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `WilliamsPercentR`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct WilliamsPercentR {
    highs: RollingExtrema,
    lows: RollingExtrema,
    value: Option<f64>,
}

impl WilliamsPercentR {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            highs: RollingExtrema::new(period)?,
            lows: RollingExtrema::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let maximum = self.highs.append(high).map(|value| value.0);
        let minimum = self.lows.append(low).map(|value| value.1);
        self.value = maximum.zip(minimum).map(|(maximum, minimum)| {
            let range = maximum - minimum;
            if range > 0.0 {
                -100.0 * (maximum - close) / range
            } else {
                0.0
            }
        });
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
        self.highs.reset();
        self.lows.reset();
        self.value = None;
    }
}
