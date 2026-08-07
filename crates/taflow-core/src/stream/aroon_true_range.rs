//! Aroon and true-range family streaming states.

use crate::error::TaResult;

use super::{invalid_period, RollingExtrema};

/// Compute the true range result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `close` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn true_range(high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<f64>> {
    if high.len() != low.len() || high.len() != close.len() {
        return Err(crate::TaError::LengthMismatch { expected: high.len(), got: low.len().min(close.len()) });
    }
    let mut state = TrueRange::new();
    Ok(high.iter().zip(low).zip(close).map(|((high, low), close)| state.append(*high, *low, *close).unwrap_or(f64::NAN)).collect())
}

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
pub fn average_true_range(high: &[f64], low: &[f64], close: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if high.len() != low.len() || high.len() != close.len() {
        return Err(crate::TaError::LengthMismatch { expected: high.len(), got: low.len().min(close.len()) });
    }
    let mut state = AverageTrueRange::new(timeperiod)?;
    Ok(high.iter().zip(low).zip(close).map(|((high, low), close)| state.append(*high, *low, *close).unwrap_or(f64::NAN)).collect())
}

/// Compute the normalized average true range result for the supplied aligned series.
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
pub fn normalized_average_true_range(high: &[f64], low: &[f64], close: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if high.len() != low.len() || high.len() != close.len() {
        return Err(crate::TaError::LengthMismatch { expected: high.len(), got: low.len().min(close.len()) });
    }
    let mut state = NormalizedAverageTrueRange::new(timeperiod)?;
    Ok(high.iter().zip(low).zip(close).map(|((high, low), close)| state.append(*high, *low, *close).unwrap_or(f64::NAN)).collect())
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AroonValue {
    pub down: f64,
    pub up: f64,
}

/// Stateful Aroon down/up pair over a `period + 1` bar window.
#[derive(Debug, Clone)]
pub struct Aroon {
    period: usize,
    inverse_period: f64,
    index: usize,
    highs: RollingExtrema,
    lows: RollingExtrema,
    value: Option<AroonValue>,
}

impl Aroon {
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
            period,
            inverse_period: 100.0 / period as f64,
            index: 0,
            highs: RollingExtrema::new(period + 1)?,
            lows: RollingExtrema::new(period + 1)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64) -> Option<AroonValue> {
        let current = self.index;
        self.index += 1;
        let highest = self.highs.append_indexed(high).map(|value| value.0 .0);
        let lowest = self.lows.append_indexed(low).map(|value| value.1 .0);
        self.value = highest.zip(lowest).map(|(highest, lowest)| AroonValue {
            down: (self.period - (current - lowest)) as f64 * self.inverse_period,
            up: (self.period - (current - highest)) as f64 * self.inverse_period,
        });
        self.value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<AroonValue> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.index = 0;
        self.highs.reset();
        self.lows.reset();
        self.value = None;
    }
}

#[derive(Debug, Clone)]
pub struct AroonOscillator {
    aroon: Aroon,
    value: Option<f64>,
}

impl AroonOscillator {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            aroon: Aroon::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        self.value = self
            .aroon
            .append(high, low)
            .map(|value| value.up - value.down);
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
        self.aroon.reset();
        self.value = None;
    }
}

/// Stateful Average True Range.  Each appended bar is `(high, low, close)`.
#[derive(Debug, Clone)]
pub struct AverageTrueRange {
    period: usize,
    previous_close: Option<f64>,
    tr_count: usize,
    tr_sum: f64,
    value: Option<f64>,
}

/// Stateful true range. The first bar has no previous close and is not warm.
#[derive(Debug, Clone)]
pub struct TrueRange {
    previous_close: Option<f64>,
    value: Option<f64>,
}

impl TrueRange {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            previous_close: None,
            value: None,
        }
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let previous = self.previous_close.replace(close)?;
        self.value = Some(
            (high - low)
                .max((high - previous).abs())
                .max((low - previous).abs()),
        );
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
        self.value = None;
    }
}

impl Default for TrueRange {
    fn default() -> Self {
        Self::new()
    }
}

/// Stateful normalized ATR, matching `NATR = ATR / close * 100`.
#[derive(Debug, Clone)]
pub struct NormalizedAverageTrueRange {
    atr: AverageTrueRange,
    value: Option<f64>,
}

impl NormalizedAverageTrueRange {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            atr: AverageTrueRange::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        self.value = self.atr.append(high, low, close).map(|atr| {
            if close == 0.0 {
                0.0
            } else {
                atr / close * 100.0
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
        self.atr.reset();
        self.value = None;
    }
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
