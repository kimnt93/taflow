use std::collections::VecDeque;

use super::{
    AverageTrueRange, CumulativeMaximum, ExponentialMovingAverage, RollingMedian, RollingMidprice,
    RollingMode, RollingStandardDeviation, SimpleMovingAverage, StreamingIndicator, TrueRange,
    Window,
};
use crate::error::{TaError, TaResult};

pub(crate) fn validate_period(timeperiod: usize) -> TaResult<()> {
    if timeperiod == 0 {
        return Err(TaError::InvalidParameter {
            name: "timeperiod",
            value: timeperiod.to_string(),
            reason: "must be >= 1",
        });
    }
    Ok(())
}

pub(crate) fn validate_quantile(quantile: f64) -> TaResult<()> {
    if !(0.0..=1.0).contains(&quantile) {
        return Err(TaError::InvalidParameter {
            name: "quantile",
            value: quantile.to_string(),
            reason: "must be between 0 and 1",
        });
    }
    Ok(())
}

macro_rules! bar_relation_operator {
    ($name:ident, $predicate:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            previous: Option<(f64, f64)>,
            value: Option<f64>,
        }
        impl $name {
            /// Create an empty causal bar-relation state.
            pub fn new() -> Self {
                Self {
                    previous: None,
                    value: None,
                }
            }
            /// Append one high/low bar and return `1`, `0`, or warm-up `None`.
            pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
                self.value = self.previous.map(|(previous_high, previous_low)| {
                    if $predicate(high, low, previous_high, previous_low) {
                        1.0
                    } else {
                        0.0
                    }
                });
                self.previous = Some((high, low));
                self.value
            }
            /// Return the latest relation result.
            pub fn value(&self) -> Option<f64> {
                self.value
            }
            /// Clear the previous bar and latest result.
            pub fn reset(&mut self) {
                self.previous = None;
                self.value = None;
            }
        }
        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}
bar_relation_operator!(HigherHigh, |h: f64, _l: f64, ph: f64, _pl: f64| h > ph);
bar_relation_operator!(LowerLow, |_h: f64, l: f64, _ph: f64, pl: f64| l < pl);
bar_relation_operator!(InsideBar, |h: f64, l: f64, ph: f64, pl: f64| h < ph
    && l > pl);
bar_relation_operator!(OutsideBar, |h: f64, l: f64, ph: f64, pl: f64| h > ph
    && l < pl);
bar_relation_operator!(GapUp, |_h: f64, l: f64, ph: f64, _pl: f64| l > ph);
bar_relation_operator!(GapDown, |h: f64, _l: f64, _ph: f64, pl: f64| h < pl);

#[derive(Debug, Clone)]
/// Stateful bars-since accumulator over a boolean condition stream.
///
/// The output is causal, aligned with each input bar, and resettable.
pub struct BarsSince {
    count: Option<usize>,
    value: Option<f64>,
}
impl BarsSince {
    /// Create a new empty state.
    ///
    pub fn new() -> Self {
        Self {
            count: None,
            value: None,
        }
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, condition: bool) -> Option<f64> {
        self.count = Some(if condition {
            0
        } else {
            self.count.map_or(0, |v| v + 1)
        });
        self.value = self.count.map(|v| v as f64);
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
        self.count = None;
        self.value = None;
    }
}
impl Default for BarsSince {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone)]
/// Stateful value lookup that retains the most recent value at a true condition.
///
/// The lookup is causal and returns an aligned optional result.
pub struct ValueWhen {
    latest: Option<f64>,
    value: Option<f64>,
}
impl ValueWhen {
    /// Create a new empty state.
    ///
    pub fn new() -> Self {
        Self {
            latest: None,
            value: None,
        }
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, condition: bool, input: f64) -> Option<f64> {
        if condition {
            self.latest = Some(input);
        }
        self.value = self.latest;
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
        self.latest = None;
        self.value = None;
    }
}
impl Default for ValueWhen {
    fn default() -> Self {
        Self::new()
    }
}
macro_rules! since_extreme {
    ($name:ident, $operation:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            extreme: Option<f64>,
            value: Option<f64>,
        }
        impl $name {
            /// Create an empty since-extreme state.
            pub fn new() -> Self {
                Self {
                    extreme: None,
                    value: None,
                }
            }
            /// Update the extreme after a condition and return its latest value.
            pub fn append(&mut self, condition: bool, input: f64) -> Option<f64> {
                self.extreme = Some(if condition {
                    input
                } else {
                    self.extreme.map_or(input, |value| $operation(value, input))
                });
                self.value = self.extreme;
                self.value
            }
            /// Return the latest since-extreme value.
            pub fn value(&self) -> Option<f64> {
                self.value
            }
            /// Clear the accumulated extreme.
            pub fn reset(&mut self) {
                self.extreme = None;
                self.value = None;
            }
        }
        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}
since_extreme!(HighestSince, f64::max);
since_extreme!(LowestSince, f64::min);

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingAlpha`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingAlpha {
    values: VecDeque<(f64, f64)>,
    period: usize,
    value: Option<f64>,
}
impl RollingAlpha {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            values: VecDeque::with_capacity(period),
            period,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, input: f64, benchmark: f64) -> Option<f64> {
        if self.values.len() == self.period {
            self.values.pop_front();
        }
        self.values.push_back((input, benchmark));
        self.value = (self.values.len() == self.period).then(|| {
            let n = self.period as f64;
            let mean_input = self.values.iter().map(|&(input, _)| input).sum::<f64>() / n;
            let mean_benchmark = self
                .values
                .iter()
                .map(|&(_, benchmark)| benchmark)
                .sum::<f64>()
                / n;
            let covariance = self
                .values
                .iter()
                .map(|&(input, benchmark)| (input - mean_input) * (benchmark - mean_benchmark))
                .sum::<f64>();
            let variance = self
                .values
                .iter()
                .map(|&(_, benchmark)| (benchmark - mean_benchmark).powi(2))
                .sum::<f64>();
            let beta = if variance > 0.0 {
                covariance / variance
            } else {
                0.0
            };
            mean_input - beta * mean_benchmark
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
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingInformationRatio`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingInformationRatio {
    values: VecDeque<f64>,
    period: usize,
    value: Option<f64>,
}
impl RollingInformationRatio {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            values: VecDeque::with_capacity(period),
            period,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, input: f64, benchmark: f64) -> Option<f64> {
        if self.values.len() == self.period {
            self.values.pop_front();
        }
        self.values.push_back(input - benchmark);
        self.value = (self.values.len() == self.period).then(|| {
            let n = self.period as f64;
            let mean = self.values.iter().sum::<f64>() / n;
            let variance = self
                .values
                .iter()
                .map(|&value| (value - mean).powi(2))
                .sum::<f64>()
                / n;
            if variance > 0.0 {
                mean / variance.sqrt()
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
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Hurst`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Hurst {
    values: VecDeque<f64>,
    period: usize,
    value: Option<f64>,
}

impl Hurst {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: period.to_string(),
                reason: "must be >= 2",
            });
        }
        Ok(Self {
            values: VecDeque::with_capacity(period),
            period,
            value: None,
        })
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    /// Compute the append result for the supplied aligned series.
    ///
    /// # Parameters
    ///
    /// * `&mut self` - Input series or configuration value.
    /// * `input` - Input series or configuration value.
    ///
    /// # Returns
    ///
    /// An aligned result with TA-Lib-compatible validation and warm-up values.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.period {
            self.values.pop_front();
        }
        self.values.push_back(input);
        self.value = (self.values.len() == self.period).then(|| {
            let n = self.period as f64;
            let mean = self.values.iter().sum::<f64>() / n;
            let mut cumulative = 0.0;
            let mut minimum = f64::INFINITY;
            let mut maximum = f64::NEG_INFINITY;
            for &value in &self.values {
                cumulative += value - mean;
                minimum = minimum.min(cumulative);
                maximum = maximum.max(cumulative);
            }
            let standard_deviation = (self
                .values
                .iter()
                .map(|&value| (value - mean).powi(2))
                .sum::<f64>()
                / n)
                .sqrt();
            let rescaled_range = (maximum - minimum) / standard_deviation;
            if rescaled_range > 0.0 {
                (rescaled_range.ln() / n.ln()).clamp(0.0, 1.0)
            } else {
                0.5
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
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingEntropy`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingEntropy {
    values: VecDeque<f64>,
    period: usize,
    value: Option<f64>,
}

impl RollingEntropy {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            values: VecDeque::with_capacity(period),
            period,
            value: None,
        })
    }

    /// Shannon entropy of exact-value frequencies in the rolling window.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.period {
            self.values.pop_front();
        }
        self.values.push_back(input);
        self.value = (self.values.len() == self.period).then(|| {
            let n = self.period as f64;
            let mut entropy = 0.0;
            let mut seen = Vec::new();
            for &candidate in &self.values {
                if seen.contains(&candidate) {
                    continue;
                }
                seen.push(candidate);
                let count = self
                    .values
                    .iter()
                    .filter(|&&value| value == candidate)
                    .count();
                let probability = count as f64 / n;
                entropy -= probability * probability.ln();
            }
            entropy
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
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingAutocorr`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingAutocorr {
    values: VecDeque<f64>,
    period: usize,
    value: Option<f64>,
}

impl RollingAutocorr {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: period.to_string(),
                reason: "must be >= 2",
            });
        }
        Ok(Self {
            values: VecDeque::with_capacity(period),
            period,
            value: None,
        })
    }

    /// Lag-one Pearson autocorrelation over the rolling window.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.period {
            self.values.pop_front();
        }
        self.values.push_back(input);
        self.value = (self.values.len() == self.period).then(|| {
            let n = self.period as f64;
            let left_n = (self.period - 1) as f64;
            let left_mean = self.values.iter().take(self.period - 1).sum::<f64>() / left_n;
            let right_mean = self.values.iter().skip(1).sum::<f64>() / left_n;
            let left_variance = self
                .values
                .iter()
                .take(self.period - 1)
                .map(|&value| (value - left_mean).powi(2))
                .sum::<f64>();
            let right_variance = self
                .values
                .iter()
                .skip(1)
                .map(|&value| (value - right_mean).powi(2))
                .sum::<f64>();
            if left_variance == 0.0 || right_variance == 0.0 {
                return 0.0;
            }
            let covariance = self
                .values
                .iter()
                .take(self.period - 1)
                .zip(self.values.iter().skip(1))
                .map(|(&left, &right)| (left - left_mean) * (right - right_mean))
                .sum::<f64>();
            covariance / (left_variance * right_variance).sqrt()
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
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `HedgeRatio`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct HedgeRatio {
    values: VecDeque<(f64, f64)>,
    period: usize,
    value: Option<f64>,
}

impl HedgeRatio {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            values: VecDeque::with_capacity(period),
            period,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, x: f64, y: f64) -> Option<f64> {
        if self.values.len() == self.period {
            self.values.pop_front();
        }
        self.values.push_back((x, y));
        self.value = if self.values.len() == self.period {
            let n = self.period as f64;
            let mean_x = self.values.iter().map(|&(x, _)| x).sum::<f64>() / n;
            let mean_y = self.values.iter().map(|&(_, y)| y).sum::<f64>() / n;
            let covariance = self
                .values
                .iter()
                .map(|&(x, y)| (x - mean_x) * (y - mean_y))
                .sum::<f64>();
            let variance = self
                .values
                .iter()
                .map(|&(x, _)| (x - mean_x).powi(2))
                .sum::<f64>();
            Some(if variance > 0.0 {
                covariance / variance
            } else {
                0.0
            })
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
        self.values.clear();
        self.value = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `SessionExtremaValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SessionExtremaValue {
    pub high: f64,
    pub low: f64,
}

#[derive(Debug, Clone, Default)]
/// Persistent Rust state or aligned output type for `SessionExtrema`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SessionExtrema {
    high: Option<f64>,
    low: Option<f64>,
    value: Option<SessionExtremaValue>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Zone {
    pub top: f64,
    pub bottom: f64,
    pub birth: usize,
    pub flags: u32,
}

/// Bounded active-zone storage for causal zone-based indicators.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `ActiveZoneList`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct ActiveZoneList {
    zones: Vec<Zone>,
    capacity: usize,
    index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `FairValueGapValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct FairValueGapValue {
    pub signal: f64,
    pub top: f64,
    pub bottom: f64,
    pub mitigated: f64,
}

#[derive(Debug, Clone, Copy)]
struct FvgZone {
    direction: f64,
    top: f64,
    bottom: f64,
}

/// Causal fair-value-gap detection with directional mitigation events.
#[derive(Debug, Clone, Default)]
/// Persistent Rust state or aligned output type for `FairValueGap`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct FairValueGap {
    bars: VecDeque<(f64, f64, f64, f64)>,
    zones: Vec<FvgZone>,
    value: Option<FairValueGapValue>,
}

impl FairValueGap {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append one causal observation and return the latest result.
    ///
    pub fn append(
        &mut self,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
    ) -> Option<FairValueGapValue> {
        let previous = self.bars.back().copied();
        let two_back = self.bars.front().copied();
        let mut signal = f64::NAN;
        let mut top = f64::NAN;
        let mut bottom = f64::NAN;
        if let (Some((middle_open, _, _, middle_close)), Some((_, old_high, old_low, _))) =
            (previous, two_back)
        {
            if old_high < low && middle_close > middle_open {
                signal = 1.0;
                top = low;
                bottom = old_high;
                self.zones.push(FvgZone {
                    direction: signal,
                    top,
                    bottom,
                });
            } else if old_low > high && middle_close < middle_open {
                signal = -1.0;
                top = old_low;
                bottom = high;
                self.zones.push(FvgZone {
                    direction: signal,
                    top,
                    bottom,
                });
            }
        }
        let mut mitigated = f64::NAN;
        self.zones.retain(|zone| {
            let filled = (zone.direction > 0.0 && low <= zone.bottom)
                || (zone.direction < 0.0 && high >= zone.top);
            if filled {
                mitigated = zone.direction;
            }
            !filled
        });
        if signal.is_nan() && !mitigated.is_nan() {
            signal = f64::NAN;
        }
        if self.bars.len() == 2 {
            self.bars.pop_front();
        }
        self.bars.push_back((open, high, low, close));
        let value = FairValueGapValue {
            signal,
            top,
            bottom,
            mitigated,
        };
        self.value = Some(value);
        Some(value)
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<FairValueGapValue> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.bars.clear();
        self.zones.clear();
        self.value = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `BreakOfStructureChangeOfCharacterValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct BreakOfStructureChangeOfCharacterValue {
    pub bos: f64,
    pub choch: f64,
    pub level: f64,
    pub broken: f64,
}

/// Causal break-of-structure and change-of-character events.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `BreakOfStructureChangeOfCharacter`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct BreakOfStructureChangeOfCharacter {
    swing: SwingHighLow,
    swings: VecDeque<(f64, f64)>,
    pending: Option<(f64, f64)>,
    trend: Option<f64>,
    value: Option<BreakOfStructureChangeOfCharacterValue>,
}

impl BreakOfStructureChangeOfCharacter {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(swing_length: usize) -> TaResult<Self> {
        Ok(Self {
            swing: SwingHighLow::new(swing_length)?,
            swings: VecDeque::with_capacity(4),
            pending: None,
            trend: None,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
    ) -> BreakOfStructureChangeOfCharacterValue {
        let mut bos = f64::NAN;
        let mut choch = f64::NAN;
        let mut level = f64::NAN;
        let mut broken = f64::NAN;

        if let Some((direction, pending_level)) = self.pending {
            let crossed = (direction > 0.0 && close > pending_level)
                || (direction < 0.0 && close < pending_level);
            if crossed {
                broken = direction;
                level = pending_level;
                self.pending = None;
                self.trend = Some(direction);
            }
        }

        if let Some(swing) = self.swing.append(high, low) {
            self.swings.push_back((swing.signal, swing.level));
            if self.swings.len() > 4 {
                self.swings.pop_front();
            }
            if self.swings.len() == 4 {
                let items: Vec<_> = self.swings.iter().copied().collect();
                let bullish = items[0].0 < 0.0
                    && items[1].0 > 0.0
                    && items[2].0 < 0.0
                    && items[3].0 > 0.0
                    && items[0].1 < items[2].1
                    && items[1].1 < items[3].1;
                let bearish = items[0].0 > 0.0
                    && items[1].0 < 0.0
                    && items[2].0 > 0.0
                    && items[3].0 < 0.0
                    && items[0].1 > items[2].1
                    && items[1].1 > items[3].1;
                let direction = if bullish {
                    Some(1.0)
                } else if bearish {
                    Some(-1.0)
                } else {
                    None
                };
                if let Some(direction) = direction {
                    bos = direction;
                    choch = if self.trend.is_some_and(|trend| trend != direction) {
                        direction
                    } else {
                        f64::NAN
                    };
                    level = items[1].1;
                    self.pending = Some((direction, level));
                }
            }
        }

        let value = BreakOfStructureChangeOfCharacterValue {
            bos,
            choch,
            level,
            broken,
        };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<BreakOfStructureChangeOfCharacterValue> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.swing.reset();
        self.swings.clear();
        self.pending = None;
        self.trend = None;
        self.value = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `OrderBlockValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct OrderBlockValue {
    pub ob: f64,
    pub top: f64,
    pub bottom: f64,
    pub ob_volume: f64,
    pub mitigated: f64,
}

#[derive(Debug, Clone, Copy)]
struct ObZone {
    direction: f64,
    top: f64,
    bottom: f64,
}

/// Causal order-block detection with volatile-bar exclusion and directional
/// mitigation. Dual pivot scales: `swing_length` locates the structure
/// interval, `internal_length` locates the extreme block within it. Bars
/// whose range is at least `threshold * ATR(atr_period)` are excluded from
/// being order blocks.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `OrderBlock`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct OrderBlock {
    atr: AverageTrueRange,
    internal: SwingHighLow,
    structure: SwingHighLow,
    internal_low: Option<(f64, f64, bool)>,
    internal_high: Option<(f64, f64, bool)>,
    structure_low: Option<f64>,
    structure_high: Option<f64>,
    threshold: f64,
    zones: Vec<ObZone>,
    value: Option<OrderBlockValue>,
}

impl OrderBlock {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(
        swing_length: usize,
        internal_length: usize,
        atr_period: usize,
        threshold: f64,
    ) -> TaResult<Self> {
        validate_period(swing_length)?;
        validate_period(internal_length)?;
        if atr_period == 0 {
            return Err(TaError::InvalidParameter {
                name: "atr_period",
                value: atr_period.to_string(),
                reason: "must be >= 1",
            });
        }
        if threshold < 0.0 {
            return Err(TaError::InvalidParameter {
                name: "threshold",
                value: threshold.to_string(),
                reason: "must be >= 0",
            });
        }
        Ok(Self {
            atr: AverageTrueRange::new(atr_period)?,
            internal: SwingHighLow::new(internal_length)?,
            structure: SwingHighLow::new(swing_length)?,
            internal_low: None,
            internal_high: None,
            structure_low: None,
            structure_high: None,
            threshold,
            zones: Vec::new(),
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> OrderBlockValue {
        let atr = self.atr.append(high, low, close);
        let volatile = atr.is_some_and(|atr| high - low >= self.threshold * atr);

        let mut ob = f64::NAN;
        let mut top = f64::NAN;
        let mut bottom = f64::NAN;
        let mut ob_volume = f64::NAN;

        if let Some(internal_swing) = self.internal.append(high, low) {
            match internal_swing.signal {
                signal if signal > 0.0 => {
                    self.internal_high = Some((internal_swing.level, volume, volatile));
                    if let Some(structure_high) = self.structure_high {
                        if internal_swing.level > structure_high
                            && self.internal_low.is_some_and(|(_, _, volatile)| !volatile)
                        {
                            let (low_level, low_volume, _) =
                                self.internal_low.expect("internal low is set");
                            ob = 1.0;
                            top = internal_swing.level;
                            bottom = low_level;
                            ob_volume = low_volume;
                            self.zones.push(ObZone {
                                direction: ob,
                                top,
                                bottom,
                            });
                            self.structure_high = Some(internal_swing.level);
                        }
                    }
                }
                signal if signal < 0.0 => {
                    self.internal_low = Some((internal_swing.level, volume, volatile));
                    if let Some(structure_low) = self.structure_low {
                        if internal_swing.level < structure_low
                            && self.internal_high.is_some_and(|(_, _, volatile)| !volatile)
                        {
                            let (high_level, high_volume, _) =
                                self.internal_high.expect("internal high is set");
                            ob = -1.0;
                            top = high_level;
                            bottom = internal_swing.level;
                            ob_volume = high_volume;
                            self.zones.push(ObZone {
                                direction: ob,
                                top,
                                bottom,
                            });
                            self.structure_low = Some(internal_swing.level);
                        }
                    }
                }
                _ => {}
            }
        }

        if let Some(structure_swing) = self.structure.append(high, low) {
            match structure_swing.signal {
                signal if signal > 0.0 => self.structure_high = Some(structure_swing.level),
                signal if signal < 0.0 => self.structure_low = Some(structure_swing.level),
                _ => {}
            }
        }

        let mut mitigated = f64::NAN;
        self.zones.retain(|zone| {
            let filled = (zone.direction > 0.0 && low <= zone.bottom)
                || (zone.direction < 0.0 && high >= zone.top);
            if filled {
                mitigated = zone.direction;
            }
            !filled
        });

        let value = OrderBlockValue {
            ob,
            top,
            bottom,
            ob_volume,
            mitigated,
        };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<OrderBlockValue> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.atr.reset();
        self.internal.reset();
        self.structure.reset();
        self.internal_low = None;
        self.internal_high = None;
        self.structure_low = None;
        self.structure_high = None;
        self.zones.clear();
        self.value = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `LiquidityValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct LiquidityValue {
    pub liquidity: f64,
    pub level: f64,
    pub swept: f64,
}

#[derive(Debug, Clone, Copy)]
struct LiquidityPool {
    level: f64,
    count: usize,
}

/// Causal liquidity-pool clustering with sweep detection. SwingHighLow highs and
/// lows are clustered into pools when they fall within a `range_percent`
/// price tolerance; a pool emits a signal once a second swing confirms it.
/// A pool is swept and removed when price trades beyond its level.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Liquidity`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Liquidity {
    swing: SwingHighLow,
    high_pools: Vec<LiquidityPool>,
    low_pools: Vec<LiquidityPool>,
    range_percent: f64,
    value: Option<LiquidityValue>,
}

impl Liquidity {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(swing_length: usize, range_percent: f64) -> TaResult<Self> {
        validate_period(swing_length)?;
        if !(0.0..=1.0).contains(&range_percent) {
            return Err(TaError::InvalidParameter {
                name: "range_percent",
                value: range_percent.to_string(),
                reason: "must be between 0 and 1",
            });
        }
        Ok(Self {
            swing: SwingHighLow::new(swing_length)?,
            high_pools: Vec::new(),
            low_pools: Vec::new(),
            range_percent,
            value: None,
        })
    }

    fn nearest_pool(
        pools: &mut Vec<LiquidityPool>,
        level: f64,
        range_percent: f64,
    ) -> Option<usize> {
        let mut best: Option<(usize, f64)> = None;
        for (index, pool) in pools.iter().enumerate() {
            let distance = (pool.level - level).abs();
            if distance <= range_percent * pool.level
                && best.map_or(true, |(_, best_distance)| distance < best_distance)
            {
                best = Some((index, distance));
            }
        }
        best.map(|(index, _)| index)
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, _close: f64) -> LiquidityValue {
        let mut liquidity = f64::NAN;
        let mut level = f64::NAN;
        let mut swept = f64::NAN;

        if let Some(swing) = self.swing.append(high, low) {
            if swing.signal > 0.0 {
                if let Some(index) =
                    Self::nearest_pool(&mut self.high_pools, swing.level, self.range_percent)
                {
                    let pool = &mut self.high_pools[index];
                    pool.level = pool.level.max(swing.level);
                    pool.count += 1;
                    if pool.count >= 2 {
                        liquidity = 1.0;
                        level = pool.level;
                    }
                } else {
                    self.high_pools.push(LiquidityPool {
                        level: swing.level,
                        count: 1,
                    });
                }
            } else if swing.signal < 0.0 {
                if let Some(index) =
                    Self::nearest_pool(&mut self.low_pools, swing.level, self.range_percent)
                {
                    let pool = &mut self.low_pools[index];
                    pool.level = pool.level.min(swing.level);
                    pool.count += 1;
                    if pool.count >= 2 {
                        liquidity = -1.0;
                        level = pool.level;
                    }
                } else {
                    self.low_pools.push(LiquidityPool {
                        level: swing.level,
                        count: 1,
                    });
                }
            }
        }

        self.high_pools.retain(|pool| {
            let swept_pool = pool.count >= 2 && high >= pool.level;
            if swept_pool {
                swept = 1.0;
                level = pool.level;
            }
            !swept_pool
        });
        self.low_pools.retain(|pool| {
            let swept_pool = pool.count >= 2 && low <= pool.level;
            if swept_pool {
                swept = -1.0;
                level = pool.level;
            }
            !swept_pool
        });

        let value = LiquidityValue {
            liquidity,
            level,
            swept,
        };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<LiquidityValue> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.swing.reset();
        self.high_pools.clear();
        self.low_pools.clear();
        self.value = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `EqualHighsLowsValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct EqualHighsLowsValue {
    pub eqh: f64,
    pub eql: f64,
    pub level: f64,
}

/// Causal equal-high/equal-low detection. Two consecutive confirmed pivots
/// of the same kind are "equal" when their levels differ by less than
/// `eq_threshold * ATR(atr_period)`, matching the LuxAlgo Pine variant.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `EqualHighsLows`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct EqualHighsLows {
    atr: AverageTrueRange,
    swing: SwingHighLow,
    previous_high: Option<f64>,
    previous_low: Option<f64>,
    eq_threshold: f64,
    value: Option<EqualHighsLowsValue>,
}

impl EqualHighsLows {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(eq_len: usize, atr_period: usize, eq_threshold: f64) -> TaResult<Self> {
        validate_period(eq_len)?;
        if atr_period == 0 {
            return Err(TaError::InvalidParameter {
                name: "atr_period",
                value: atr_period.to_string(),
                reason: "must be >= 1",
            });
        }
        if !(0.0..=1.0).contains(&eq_threshold) {
            return Err(TaError::InvalidParameter {
                name: "eq_threshold",
                value: eq_threshold.to_string(),
                reason: "must be between 0 and 1",
            });
        }
        Ok(Self {
            atr: AverageTrueRange::new(atr_period)?,
            swing: SwingHighLow::new(eq_len)?,
            previous_high: None,
            previous_low: None,
            eq_threshold,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> EqualHighsLowsValue {
        let atr = self.atr.append(high, low, close);
        let mut eqh = f64::NAN;
        let mut eql = f64::NAN;
        let mut level = f64::NAN;

        if let Some(swing) = self.swing.append(high, low) {
            if swing.signal > 0.0 {
                if let (Some(previous), Some(atr)) = (self.previous_high, atr) {
                    if (swing.level - previous).abs() < atr * self.eq_threshold {
                        eqh = 1.0;
                        level = swing.level;
                    }
                }
                self.previous_high = Some(swing.level);
            } else if swing.signal < 0.0 {
                if let (Some(previous), Some(atr)) = (self.previous_low, atr) {
                    if (swing.level - previous).abs() < atr * self.eq_threshold {
                        eql = 1.0;
                        level = swing.level;
                    }
                }
                self.previous_low = Some(swing.level);
            }
        }

        let value = EqualHighsLowsValue { eqh, eql, level };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<EqualHighsLowsValue> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.atr.reset();
        self.swing.reset();
        self.previous_high = None;
        self.previous_low = None;
        self.value = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `PreviousHighLowValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct PreviousHighLowValue {
    pub prev_high: f64,
    pub prev_low: f64,
    pub broken_high: f64,
    pub broken_low: f64,
}

/// Causal prior-higher-timeframe high/low tracking with break flags. Given a
/// HTF boundary flag series, running extrema are snapshotted into
/// `prev_high`/`prev_low` at each boundary; breaks are flagged when the
/// current bar trades beyond the previous HTF bar's extrema.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `PreviousHighLow`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct PreviousHighLow {
    running_high: Option<f64>,
    running_low: Option<f64>,
    previous_high: Option<f64>,
    previous_low: Option<f64>,
    value: Option<PreviousHighLowValue>,
}

impl PreviousHighLow {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            running_high: None,
            running_low: None,
            previous_high: None,
            previous_low: None,
            value: None,
        }
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, new_session: bool, high: f64, low: f64) -> PreviousHighLowValue {
        if new_session {
            if self.running_high.is_some() {
                self.previous_high = self.running_high;
                self.previous_low = self.running_low;
            }
            self.running_high = Some(high);
            self.running_low = Some(low);
        } else {
            self.running_high = Some(self.running_high.map_or(high, |running| running.max(high)));
            self.running_low = Some(self.running_low.map_or(low, |running| running.min(low)));
        }

        let broken_high =
            self.previous_high.map_or(
                f64::NAN,
                |previous| if high > previous { 1.0 } else { f64::NAN },
            );
        let broken_low =
            self.previous_low.map_or(
                f64::NAN,
                |previous| if low < previous { 1.0 } else { f64::NAN },
            );

        let value = PreviousHighLowValue {
            prev_high: self.previous_high.unwrap_or(f64::NAN),
            prev_low: self.previous_low.unwrap_or(f64::NAN),
            broken_high,
            broken_low,
        };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<PreviousHighLowValue> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.running_high = None;
        self.running_low = None;
        self.previous_high = None;
        self.previous_low = None;
        self.value = None;
    }
}

impl Default for PreviousHighLow {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `SessionsValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SessionsValue {
    pub active: f64,
    pub session_high: f64,
    pub session_low: f64,
}

/// Causal session-scoped extrema. Given a session-boundary flag series,
/// emits a constant `active` marker and the running high/low since the last
/// boundary — matching the package's causal running extrema.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Sessions`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Sessions {
    session_high: Option<f64>,
    session_low: Option<f64>,
    started: bool,
    value: Option<SessionsValue>,
}

impl Sessions {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            session_high: None,
            session_low: None,
            started: false,
            value: None,
        }
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, new_session: bool, high: f64, low: f64) -> SessionsValue {
        if new_session || !self.started {
            self.session_high = Some(high);
            self.session_low = Some(low);
            self.started = true;
        } else {
            self.session_high = Some(self.session_high.map_or(high, |running| running.max(high)));
            self.session_low = Some(self.session_low.map_or(low, |running| running.min(low)));
        }
        let value = SessionsValue {
            active: 1.0,
            session_high: self.session_high.unwrap_or(f64::NAN),
            session_low: self.session_low.unwrap_or(f64::NAN),
        };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<SessionsValue> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.session_high = None;
        self.session_low = None;
        self.started = false;
        self.value = None;
    }
}

impl Default for Sessions {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `RetracementsValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RetracementsValue {
    pub direction: f64,
    pub current_retracement_pct: f64,
    pub deepest_retracement_pct: f64,
}

/// Causal swing-leg retracement tracking. On each confirmed swing a leg is
/// established from the opposite prior pivot; the retracement percentage is
/// the fraction of that leg already given back by the current close, with
/// the deepest value tracked since the leg began.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Retracements`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Retracements {
    swing: SwingHighLow,
    last_high: Option<f64>,
    last_low: Option<f64>,
    leg_high: Option<f64>,
    leg_low: Option<f64>,
    direction: Option<f64>,
    deepest: f64,
    value: Option<RetracementsValue>,
}

impl Retracements {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(swing_length: usize) -> TaResult<Self> {
        Ok(Self {
            swing: SwingHighLow::new(swing_length)?,
            last_high: None,
            last_low: None,
            leg_high: None,
            leg_low: None,
            direction: None,
            deepest: 0.0,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> RetracementsValue {
        if let Some(swing) = self.swing.append(high, low) {
            if swing.signal > 0.0 {
                self.last_high = Some(swing.level);
                if let Some(last_low) = self.last_low {
                    self.leg_high = Some(swing.level);
                    self.leg_low = Some(last_low);
                    self.direction = Some(1.0);
                    self.deepest = 0.0;
                }
            } else if swing.signal < 0.0 {
                self.last_low = Some(swing.level);
                if let Some(last_high) = self.last_high {
                    self.leg_high = Some(last_high);
                    self.leg_low = Some(swing.level);
                    self.direction = Some(-1.0);
                    self.deepest = 0.0;
                }
            }
        }

        let mut current_retracement_pct = f64::NAN;
        let mut deepest_retracement_pct = f64::NAN;
        if let (Some(leg_high), Some(leg_low), Some(direction)) =
            (self.leg_high, self.leg_low, self.direction)
        {
            let range = leg_high - leg_low;
            if range > 0.0 {
                let pct = if direction > 0.0 {
                    (leg_high - close) / range * 100.0
                } else {
                    (close - leg_low) / range * 100.0
                };
                current_retracement_pct = pct.max(0.0);
                self.deepest = self.deepest.max(current_retracement_pct);
                deepest_retracement_pct = self.deepest;
            }
        }

        let value = RetracementsValue {
            direction: self.direction.unwrap_or(f64::NAN),
            current_retracement_pct,
            deepest_retracement_pct,
        };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<RetracementsValue> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.swing.reset();
        self.last_high = None;
        self.last_low = None;
        self.leg_high = None;
        self.leg_low = None;
        self.direction = None;
        self.deepest = 0.0;
        self.value = None;
    }
}

/// Rolling standard deviation of log returns (close-to-close volatility).
/// Warm-up values are `NaN`.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `CloseToCloseSigma`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CloseToCloseSigma {
    mean: RollingMean,
    squares: RollingMean,
    previous_close: Option<f64>,
    value: Option<f64>,
}

impl CloseToCloseSigma {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            mean: RollingMean::new(timeperiod)?,
            squares: RollingMean::new(timeperiod)?,
            previous_close: None,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, close: f64) -> Option<f64> {
        if let Some(previous_close) = self.previous_close.replace(close) {
            if close > 0.0 && previous_close > 0.0 {
                let log_return = (close / previous_close).ln();
                let _ = self.mean.append(log_return);
                let _ = self.squares.append(log_return * log_return);
                self.value = match (self.mean.value(), self.squares.value()) {
                    (Some(mean), Some(squares)) => Some((squares - mean * mean).max(0.0).sqrt()),
                    _ => None,
                };
            }
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

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.mean.reset();
        self.squares.reset();
        self.previous_close = None;
        self.value = None;
    }
}

/// Rolling mean of `ln(H/L)² / (4 ln 2)` (Parkinson volatility).
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Parkinson`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Parkinson {
    mean: RollingMean,
    value: Option<f64>,
}

impl Parkinson {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            mean: RollingMean::new(timeperiod)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        let term = if high > low && high > 0.0 && low > 0.0 {
            (high / low).ln().powi(2) / (4.0 * 2.0f64.ln())
        } else {
            0.0
        };
        self.value = self.mean.append(term).map(|mean| mean.sqrt());
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
        self.mean.reset();
        self.value = None;
    }
}

/// Rolling mean of `0.5·ln(H/L)² − (2ln2−1)·ln(C/O)²` (Garman-Klass).
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `GarmanKlass`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct GarmanKlass {
    mean: RollingMean,
    value: Option<f64>,
}

impl GarmanKlass {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            mean: RollingMean::new(timeperiod)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<f64> {
        let term = if high > 0.0 && low > 0.0 && open > 0.0 && close > 0.0 {
            0.5 * (high / low).ln().powi(2)
                - (2.0 * 2.0f64.ln() - 1.0) * (close / open).ln().powi(2)
        } else {
            0.0
        };
        self.value = self.mean.append(term).map(|mean| mean.sqrt());
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
        self.mean.reset();
        self.value = None;
    }
}

/// Rolling mean of `ln(H/C)ln(H/O) + ln(L/C)ln(L/O)` (Rogers-Satchell).
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RogersSatchell`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RogersSatchell {
    mean: RollingMean,
    value: Option<f64>,
}

impl RogersSatchell {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            mean: RollingMean::new(timeperiod)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<f64> {
        let term = if open > 0.0 && high > 0.0 && low > 0.0 && close > 0.0 {
            (high / close).ln() * (high / open).ln() + (low / close).ln() * (low / open).ln()
        } else {
            0.0
        };
        self.value = self.mean.append(term).map(|mean| mean.sqrt());
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
        self.mean.reset();
        self.value = None;
    }
}

/// Garman-Klass with the overnight term `ln(O/C_prev)²` added (GK-Yang-Zhang).
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `GarmanKlassYangZhang`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct GarmanKlassYangZhang {
    mean: RollingMean,
    previous_close: Option<f64>,
    value: Option<f64>,
}

impl GarmanKlassYangZhang {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            mean: RollingMean::new(timeperiod)?,
            previous_close: None,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<f64> {
        if let Some(previous_close) = self.previous_close.replace(close) {
            let term =
                if open > 0.0 && high > 0.0 && low > 0.0 && close > 0.0 && previous_close > 0.0 {
                    let gk = 0.5 * (high / low).ln().powi(2)
                        - (2.0 * 2.0f64.ln() - 1.0) * (close / open).ln().powi(2);
                    let overnight = (open / previous_close).ln().powi(2);
                    gk + overnight
                } else {
                    0.0
                };
            self.value = self.mean.append(term).map(|mean| mean.sqrt());
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

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.mean.reset();
        self.previous_close = None;
        self.value = None;
    }
}

/// Yang-Zhang volatility: `σ² = σ²_on + k·σ²_oc + (1−k)·σ²_RS` with
/// `k = 0.34/(1.34 + (n+1)/(n−1))`. Highest-efficiency estimator.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `YangZhang`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct YangZhang {
    overnight: RollingMean,
    open_close: RollingMean,
    rs: RollingMean,
    timeperiod: usize,
    previous_close: Option<f64>,
    value: Option<f64>,
}

impl YangZhang {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        if timeperiod < 2 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: timeperiod.to_string(),
                reason: "must be >= 2 for Yang-Zhang",
            });
        }
        Ok(Self {
            overnight: RollingMean::new(timeperiod)?,
            open_close: RollingMean::new(timeperiod)?,
            rs: RollingMean::new(timeperiod)?,
            timeperiod,
            previous_close: None,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<f64> {
        let previous_close = self.previous_close.replace(close);
        if open > 0.0 && high > 0.0 && low > 0.0 && close > 0.0 {
            if let Some(previous_close) = previous_close {
                if previous_close > 0.0 {
                    let overnight = (open / previous_close).ln().powi(2);
                    let open_close = (close / open).ln().powi(2);
                    let rs = (high / close).ln() * (high / open).ln()
                        + (low / close).ln() * (low / open).ln();
                    let _ = self.overnight.append(overnight);
                    let _ = self.open_close.append(open_close);
                    let _ = self.rs.append(rs);
                }
            }
        }
        self.value = match (
            self.overnight.value(),
            self.open_close.value(),
            self.rs.value(),
        ) {
            (Some(on), Some(oc), Some(rs)) => {
                let n = self.timeperiod as f64;
                let k = 0.34 / (1.34 + (n + 1.0) / (n - 1.0));
                Some((on + k * oc + (1.0 - k) * rs).max(0.0).sqrt())
            }
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
        self.overnight.reset();
        self.open_close.reset();
        self.rs.reset();
        self.previous_close = None;
        self.value = None;
    }
}

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

/// Amihud illiquidity: rolling mean of `|ret| / (close × volume)`.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Amihud`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Amihud {
    mean: RollingMean,
    previous_close: Option<f64>,
    value: Option<f64>,
}

impl Amihud {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            mean: RollingMean::new(timeperiod)?,
            previous_close: None,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, close: f64, volume: f64) -> Option<f64> {
        if let Some(previous_close) = self.previous_close.replace(close) {
            let term = if close > 0.0 && previous_close > 0.0 && volume > 0.0 {
                ((close - previous_close) / previous_close).abs() / (close * volume)
            } else {
                0.0
            };
            self.value = self.mean.append(term);
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

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.mean.reset();
        self.previous_close = None;
        self.value = None;
    }
}

/// Roll spread estimate: `2√max(0, −cov(Δp_t, Δp_{t−1}))`.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollSpread`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollSpread {
    previous_price: Option<f64>,
    delta_previous: Option<f64>,
    moments: RollingPairMoments,
    value: Option<f64>,
}

#[derive(Debug, Clone)]
struct RollingPairMoments {
    x: VecDeque<f64>,
    y: VecDeque<f64>,
    timeperiod: usize,
    value: Option<f64>,
}

impl RollingPairMoments {
    fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self {
            x: VecDeque::with_capacity(timeperiod),
            y: VecDeque::with_capacity(timeperiod),
            timeperiod,
            value: None,
        })
    }

    fn append(&mut self, x: f64, y: f64) -> Option<f64> {
        if self.x.len() == self.timeperiod {
            self.x.pop_front();
            self.y.pop_front();
        }
        self.x.push_back(x);
        self.y.push_back(y);
        self.value = if self.x.len() == self.timeperiod {
            let n = self.timeperiod as f64;
            let mean_x = self.x.iter().sum::<f64>() / n;
            let mean_y = self.y.iter().sum::<f64>() / n;
            let mut cov = 0.0;
            for (x, y) in self.x.iter().zip(self.y.iter()) {
                cov += (x - mean_x) * (y - mean_y);
            }
            Some(cov / (n - 1.0))
        } else {
            None
        };
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.x.clear();
        self.y.clear();
        self.value = None;
    }
}

impl RollSpread {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            previous_price: None,
            delta_previous: None,
            moments: RollingPairMoments::new(timeperiod)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, price: f64) -> Option<f64> {
        let delta = if let Some(previous_price) = self.previous_price.replace(price) {
            price - previous_price
        } else {
            0.0
        };
        if let Some(delta_previous) = self.delta_previous {
            let _ = self.moments.append(delta, delta_previous);
        }
        self.delta_previous = Some(delta);
        self.value = self
            .moments
            .value()
            .map(|cov| 2.0 * (0.0f64 - cov).max(0.0).sqrt());
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
        self.previous_price = None;
        self.delta_previous = None;
        self.moments.reset();
        self.value = None;
    }
}

/// OU half-life: `−ln(2)/λ` where `λ` is the slope of `Δp` on lagged `p`.
/// `λ ≥ 0` yields `NaN`.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `OrnsteinUhlenbeckHalfLife`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct OrnsteinUhlenbeckHalfLife {
    moments: RollingPairMoments,
    previous_price: Option<f64>,
    value: Option<f64>,
}

impl OrnsteinUhlenbeckHalfLife {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            moments: RollingPairMoments::new(timeperiod)?,
            previous_price: None,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, price: f64) -> Option<f64> {
        if let Some(previous_price) = self.previous_price.replace(price) {
            let delta = price - previous_price;
            let _ = self.moments.append(delta, previous_price);
        }
        self.value = if let Some(cov) = self.moments.value() {
            let n = self.moments.timeperiod as f64;
            let mean_y = self.moments.y.iter().sum::<f64>() / n;
            let var_y = self
                .moments
                .y
                .iter()
                .map(|&y| (y - mean_y) * (y - mean_y))
                .sum::<f64>()
                / (n - 1.0);
            if var_y > 0.0 {
                let lambda = -cov / var_y;
                (lambda > 0.0).then_some(2.0f64.ln() / lambda)
            } else {
                None
            }
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
        self.moments.reset();
        self.previous_price = None;
        self.value = None;
    }
}

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

/// Pairs-trading z-score: rolling OLS hedge ratio `β` of `y` on `x`, spread
/// `s = y − β·x`, then `(s − mean(s)) / std(s)` over the same window —
/// composition of the `HedgeRatio` and `RollingZScore` definitions.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `SpreadZScore`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SpreadZScore {
    values: VecDeque<(f64, f64)>,
    timeperiod: usize,
    value: Option<f64>,
}

impl SpreadZScore {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self {
            values: VecDeque::with_capacity(timeperiod),
            timeperiod,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, x: f64, y: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod {
            self.values.pop_front();
        }
        self.values.push_back((x, y));
        self.value = if self.values.len() == self.timeperiod {
            let n = self.timeperiod as f64;
            let mean_x = self.values.iter().map(|&(x, _)| x).sum::<f64>() / n;
            let mean_y = self.values.iter().map(|&(_, y)| y).sum::<f64>() / n;
            let covariance = self
                .values
                .iter()
                .map(|&(x, y)| (x - mean_x) * (y - mean_y))
                .sum::<f64>();
            let variance = self
                .values
                .iter()
                .map(|&(x, _)| (x - mean_x).powi(2))
                .sum::<f64>();
            let beta = if variance > 0.0 {
                covariance / variance
            } else {
                0.0
            };
            let spread = (y - beta * x);
            let mean_spread = self.values.iter().map(|&(x, y)| y - beta * x).sum::<f64>() / n;
            let std_spread = (self
                .values
                .iter()
                .map(|&(x, y)| (y - beta * x - mean_spread).powi(2))
                .sum::<f64>()
                / n)
                .sqrt();
            Some(if std_spread > 0.0 {
                (spread - mean_spread) / std_spread
            } else {
                0.0
            })
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
        self.values.clear();
        self.value = None;
    }
}

/// Fractionally-differentiated series (AFML §5.4, fixed-width window).
///
/// Weights `w_0 = 1`, `w_k = −w_{k−1}·(d−k+1)/k` truncated once
/// `|w_k| < threshold`; each output is the dot product of the weights with the
/// last `len(weights)` inputs — O(w) per bar over a ring buffer.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `FracDiff`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct FracDiff {
    weights: Vec<f64>,
    window: VecDeque<f64>,
    value: Option<f64>,
}

impl FracDiff {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(d: f64, threshold: f64) -> TaResult<Self> {
        if !(d > 0.0) {
            return Err(TaError::InvalidParameter {
                name: "d",
                value: d.to_string(),
                reason: "must be > 0",
            });
        }
        if !(threshold > 0.0) {
            return Err(TaError::InvalidParameter {
                name: "threshold",
                value: threshold.to_string(),
                reason: "must be > 0",
            });
        }
        let mut weights = vec![1.0];
        let mut k = 1usize;
        loop {
            let wk = -weights[k - 1] * (d - k as f64 + 1.0) / k as f64;
            if wk.abs() < threshold {
                break;
            }
            weights.push(wk);
            k += 1;
        }
        let capacity = weights.len();
        Ok(Self {
            weights,
            window: VecDeque::with_capacity(capacity),
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.window.len() == self.weights.len() {
            self.window.pop_front();
        }
        self.window.push_back(input);
        self.value = if self.window.len() == self.weights.len() {
            let mut acc = 0.0;
            for (i, &w) in self.weights.iter().enumerate() {
                acc += w * self.window[self.window.len() - 1 - i];
            }
            Some(acc)
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
        self.window.clear();
        self.value = None;
    }
}

/// Online Kalman estimate of the hedge ratio `β` in `y = α + β·x + v`.
///
/// Two-state filter with random-walk transition (`Q = δ·I`) and observation
/// noise `R` (QuantStart "Dynamic Hedge Ratio"; pykalman `filter_update`).
/// The primary output is `β`; `α`, the innovation, and `√S` are also exposed.
/// O(1) per bar — no linear-algebra dependency.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `KalmanHedgeRatio`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct KalmanHedgeRatio {
    alpha: f64,
    beta: f64,
    p_aa: f64,
    p_ab: f64,
    p_bb: f64,
    delta: f64,
    observation_variance: f64,
    value: Option<f64>,
    alpha_value: Option<f64>,
    innovation: Option<f64>,
    std_value: Option<f64>,
}

impl KalmanHedgeRatio {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(delta: f64, observation_variance: f64) -> TaResult<Self> {
        if !(delta >= 0.0) {
            return Err(TaError::InvalidParameter {
                name: "delta",
                value: delta.to_string(),
                reason: "must be >= 0",
            });
        }
        if !(observation_variance > 0.0) {
            return Err(TaError::InvalidParameter {
                name: "observation_variance",
                value: observation_variance.to_string(),
                reason: "must be > 0",
            });
        }
        Ok(Self {
            alpha: 0.0,
            beta: 1.0,
            p_aa: 1.0,
            p_ab: 0.0,
            p_bb: 1.0,
            delta,
            observation_variance,
            value: None,
            alpha_value: None,
            innovation: None,
            std_value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, x: f64, y: f64) -> Option<f64> {
        // Predict: θ stays, P += Q (Q = delta·I adds to the diagonal).
        let p_aa = self.p_aa + self.delta;
        let p_ab = self.p_ab;
        let p_bb = self.p_bb + self.delta;

        // Innovation and Kalman gain.
        let innovation = y - (self.alpha + self.beta * x);
        let s = p_aa + 2.0 * p_ab * x + p_bb * x * x + self.observation_variance;
        let k1 = (p_aa + p_ab * x) / s;
        let k2 = (p_ab + p_bb * x) / s;

        // Update state.
        self.alpha += k1 * innovation;
        self.beta += k2 * innovation;

        // Update covariance: P = (I - K·H)·P.
        let p_aa_new = (1.0 - k1) * p_aa - k1 * x * p_ab;
        let p_ab_new = (1.0 - k1) * p_ab - k1 * x * p_bb;
        let p_bb_new = -k2 * p_ab + (1.0 - k2 * x) * p_bb;
        self.p_aa = p_aa_new;
        self.p_ab = p_ab_new;
        self.p_bb = p_bb_new;

        self.value = Some(self.beta);
        self.alpha_value = Some(self.alpha);
        self.innovation = Some(innovation);
        self.std_value = Some(s.sqrt());
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

    /// Return the current smoothing factor, if available.
    ///
    pub fn alpha(&self) -> Option<f64> {
        self.alpha_value
    }

    /// Computes or updates `innovation` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn innovation(&self) -> Option<f64> {
        self.innovation
    }

    /// Return the current standard deviation, if available.
    ///
    pub fn std(&self) -> Option<f64> {
        self.std_value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.alpha = 0.0;
        self.beta = 1.0;
        self.p_aa = 1.0;
        self.p_ab = 0.0;
        self.p_bb = 1.0;
        self.value = None;
        self.alpha_value = None;
        self.innovation = None;
        self.std_value = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `SupertrendValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SupertrendValue {
    pub trend: f64,
    pub direction: f64,
    pub long: f64,
    pub short: f64,
}

/// Stateful Supertrend (pandas-ta classic `overlap/supertrend.py`, theory:
/// Olivier Seban). Band = `hl2 ± multiplier·ATR`; the direction flips when
/// close crosses the previous final band, otherwise the band ratchets
/// monotonic while the trend persists.
///
/// ATR uses pandas-ta classic 0.6.52's RMA seed convention: true range of
/// bar 0 is NaN, the seed is the mean of the first `length − 1` true ranges
/// placed at bar `length − 1`, then Wilder smoothing. This differs from the
/// TA-Lib ATR seed (bar `length`, `length` true ranges) — the first output
/// therefore lands at bar `length − 1`. Direction starts at `+1`; `long` is
/// the lower band when direction is `+1`, `short` is the upper band when
/// `−1`, the unused band is NaN.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Supertrend`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Supertrend {
    period: usize,
    multiplier: f64,
    alpha: f64,
    tr_count: usize,
    tr_sum: f64,
    previous_close: Option<f64>,
    atr: Option<f64>,
    direction: f64,
    upper: Option<f64>,
    lower: Option<f64>,
    value: Option<SupertrendValue>,
}

impl Supertrend {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize, multiplier: f64) -> TaResult<Self> {
        validate_period(period)?;
        if !(multiplier > 0.0) {
            return Err(TaError::InvalidParameter {
                name: "multiplier",
                value: multiplier.to_string(),
                reason: "must be > 0",
            });
        }
        Ok(Self {
            period,
            multiplier,
            alpha: 1.0 / period as f64,
            tr_count: 0,
            tr_sum: 0.0,
            previous_close: None,
            atr: None,
            direction: 1.0,
            upper: None,
            lower: None,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<SupertrendValue> {
        let Some(previous_close) = self.previous_close.replace(close) else {
            return None;
        };
        let true_range = (high - low)
            .max((high - previous_close).abs())
            .max((low - previous_close).abs());
        self.tr_count += 1;

        if self.period == 1 {
            self.atr = Some(true_range);
        } else if self.tr_count < self.period - 1 {
            self.tr_sum += true_range;
            return None;
        } else if self.tr_count == self.period - 1 {
            self.atr = Some((self.tr_sum + true_range) / (self.period - 1) as f64);
        } else if let Some(previous) = self.atr {
            self.atr = Some(previous + self.alpha * (true_range - previous));
        }

        let atr = self.atr?;
        let hl2 = (high + low) * 0.5;
        let mut raw_upper = hl2 + self.multiplier * atr;
        let mut raw_lower = hl2 - self.multiplier * atr;

        if let (Some(previous_upper), Some(previous_lower)) = (self.upper, self.lower) {
            let direction = if close > previous_upper {
                1.0
            } else if close < previous_lower {
                -1.0
            } else {
                let direction = self.direction;
                if direction > 0.0 && raw_lower < previous_lower {
                    raw_lower = previous_lower;
                }
                if direction < 0.0 && raw_upper > previous_upper {
                    raw_upper = previous_upper;
                }
                direction
            };
            self.direction = direction;
        }

        self.upper = Some(raw_upper);
        self.lower = Some(raw_lower);

        let (trend, long, short) = if self.direction > 0.0 {
            (raw_lower, raw_lower, f64::NAN)
        } else {
            (raw_upper, f64::NAN, raw_upper)
        };
        let value = SupertrendValue {
            trend,
            direction: self.direction,
            long,
            short,
        };
        self.value = Some(value);
        Some(value)
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<SupertrendValue> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.tr_count = 0;
        self.tr_sum = 0.0;
        self.previous_close = None;
        self.atr = None;
        self.direction = 1.0;
        self.upper = None;
        self.lower = None;
        self.value = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `IchimokuValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct IchimokuValue {
    pub tenkan_sen: f64,
    pub kijun_sen: f64,
    pub span_a: f64,
    pub span_b: f64,
    pub chikou_span: f64,
}

/// Stateful Ichimoku Kinkō Hyō (pandas-ta classic `overlap/ichimoku.py`).
///
/// Tenkan/Kijun are rolling `(max high + min low)/2` over their windows;
/// `span_a = 0.5·(tenkan + kijun)`; `span_b` is the same midpoint over the
/// Senkou window. All components are emitted **causally** at bar `i`: the
/// package displaces `span_a`/`span_b` forward `kijun` bars and chikou
/// backward `kijun` bars for plotting — that shift is presentation, so
/// taflow keeps the raw values and documents the displacement constants
/// instead (re-align in tests by `span.shift(kijun)`, `chikou.shift(-kijun)`).
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Ichimoku`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Ichimoku {
    tenkan: RollingMidprice,
    kijun: RollingMidprice,
    senkou: RollingMidprice,
    value: Option<IchimokuValue>,
}

impl Ichimoku {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(tenkan: usize, kijun: usize, senkou: usize) -> TaResult<Self> {
        validate_period(tenkan)?;
        validate_period(kijun)?;
        validate_period(senkou)?;
        Ok(Self {
            tenkan: RollingMidprice::new(tenkan)?,
            kijun: RollingMidprice::new(kijun)?,
            senkou: RollingMidprice::new(senkou)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> IchimokuValue {
        let tenkan = self.tenkan.append(high, low).unwrap_or(f64::NAN);
        let kijun = self.kijun.append(high, low).unwrap_or(f64::NAN);
        let span_b = self.senkou.append(high, low).unwrap_or(f64::NAN);
        let span_a = if tenkan.is_nan() || kijun.is_nan() {
            f64::NAN
        } else {
            0.5 * (tenkan + kijun)
        };
        let value = IchimokuValue {
            tenkan_sen: tenkan,
            kijun_sen: kijun,
            span_a,
            span_b,
            chikou_span: close,
        };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<IchimokuValue> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.tenkan.reset();
        self.kijun.reset();
        self.senkou.reset();
        self.value = None;
    }
}

/// SMA of the true-range series with pandas-ta's NaN-at-bar-0 convention.
///
/// The true range of bar 0 is NaN and is excluded from every window, so the
/// first valid band lands at bar `period` (windows over bars `1..=period`)
/// instead of `period - 1`.
#[derive(Debug, Clone)]
struct SqueezeTrBand {
    period: usize,
    window: Window,
    sum: f64,
    value: Option<f64>,
}

impl SqueezeTrBand {
    fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            period,
            window: Window::new(period)?,
            sum: 0.0,
            value: None,
        })
    }

    fn append(&mut self, tr: f64) -> Option<f64> {
        if !tr.is_nan() {
            if let Some(old) = self.window.push(tr) {
                self.sum -= old;
            }
            self.sum += tr;
        }
        self.value = self.window.is_full().then(|| self.sum / self.period as f64);
        self.value
    }

    fn reset(&mut self) {
        self.window.clear();
        self.sum = 0.0;
        self.value = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `SqueezeValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SqueezeValue {
    pub squeeze: f64,
    pub on: f64,
    pub off: f64,
    pub no: f64,
}

/// Stateful TTM Squeeze (pandas-ta classic `momentum/squeeze.py`, theory:
/// John Carter). A Bollinger Bands envelope (SMA basis, population std) is
/// compared against a Keltner Channel (SMA of close, SMA of true range) to
/// classify compression states; the momentum line is an SMA of the
/// `close − close[mom_length]` difference.
///
/// All four band components are O(1) incremental states; `on`/`off`/`no` are
/// `0/1` booleans and, like pandas-ta's `&` against NaN, report `no = 1`
/// during warm-up (before both envelopes are defined).
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Squeeze`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Squeeze {
    bb_length: usize,
    bb_std: f64,
    kc_length: usize,
    kc_scalar: f64,
    mom_length: usize,
    mom_smooth: usize,
    bb_mid: SimpleMovingAverage,
    bb_dev: RollingStandardDeviation,
    kc_basis: SimpleMovingAverage,
    tr_band: SqueezeTrBand,
    trange: TrueRange,
    close_window: Window,
    mom_smooth_sma: SimpleMovingAverage,
    value: Option<SqueezeValue>,
}

impl Squeeze {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(
        bb_length: usize,
        bb_std: f64,
        kc_length: usize,
        kc_scalar: f64,
        mom_length: usize,
        mom_smooth: usize,
    ) -> TaResult<Self> {
        validate_period(bb_length)?;
        validate_period(kc_length)?;
        validate_period(mom_length)?;
        validate_period(mom_smooth)?;
        if !(bb_std > 0.0) {
            return Err(TaError::InvalidParameter {
                name: "bb_std",
                value: bb_std.to_string(),
                reason: "must be > 0",
            });
        }
        if !(kc_scalar > 0.0) {
            return Err(TaError::InvalidParameter {
                name: "kc_scalar",
                value: kc_scalar.to_string(),
                reason: "must be > 0",
            });
        }
        Ok(Self {
            bb_length,
            bb_std,
            kc_length,
            kc_scalar,
            mom_length,
            mom_smooth,
            bb_mid: SimpleMovingAverage::new(bb_length)?,
            bb_dev: RollingStandardDeviation::new(bb_length, 1.0)?,
            kc_basis: SimpleMovingAverage::new(kc_length)?,
            tr_band: SqueezeTrBand::new(kc_length)?,
            trange: TrueRange::new(),
            close_window: Window::new(mom_length)?,
            mom_smooth_sma: SimpleMovingAverage::new(mom_smooth)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> SqueezeValue {
        let (bb_lower, bb_upper) = match (self.bb_mid.append(close), self.bb_dev.append(close)) {
            (Some(mid), Some(std)) => (mid - self.bb_std * std, mid + self.bb_std * std),
            _ => (f64::NAN, f64::NAN),
        };

        let kc_basis = self.kc_basis.append(close);
        let tr = self.trange.append(high, low, close).unwrap_or(f64::NAN);
        let kc_band = self.tr_band.append(tr);
        let (kc_lower, kc_upper) = match (kc_basis, kc_band) {
            (Some(basis), Some(band)) => {
                (basis - self.kc_scalar * band, basis + self.kc_scalar * band)
            }
            _ => (f64::NAN, f64::NAN),
        };

        let mom = self.close_window.push(close).map(|old| close - old);
        let squeeze = mom
            .and_then(|mom| self.mom_smooth_sma.append(mom))
            .unwrap_or(f64::NAN);

        let on = (bb_lower > kc_lower && bb_upper < kc_upper) as u8 as f64;
        let off = (bb_lower < kc_lower && bb_upper > kc_upper) as u8 as f64;
        let no = if on == 0.0 && off == 0.0 { 1.0 } else { 0.0 };

        let value = SqueezeValue {
            squeeze,
            on,
            off,
            no,
        };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<SqueezeValue> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.bb_mid.reset();
        self.bb_dev.reset();
        self.kc_basis.reset();
        self.tr_band.reset();
        self.trange.reset();
        self.close_window.clear();
        self.mom_smooth_sma.reset();
        self.value = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `SqueezeProValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SqueezeProValue {
    pub squeeze: f64,
    pub on_wide: f64,
    pub on_normal: f64,
    pub on_narrow: f64,
    pub off: f64,
    pub no: f64,
}

/// Stateful Squeeze PRO (pandas-ta classic `momentum/squeeze_pro.py`): the
/// TTM Squeeze with three Keltner scalar levels (`wide`/`normal`/`narrow`)
/// sharing one SMA basis and one SMA-of-TR band.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `SqueezePro`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SqueezePro {
    bb_length: usize,
    bb_std: f64,
    kc_length: usize,
    kc_scalar_wide: f64,
    kc_scalar_normal: f64,
    kc_scalar_narrow: f64,
    mom_length: usize,
    mom_smooth: usize,
    bb_mid: SimpleMovingAverage,
    bb_dev: RollingStandardDeviation,
    kc_basis: SimpleMovingAverage,
    tr_band: SqueezeTrBand,
    trange: TrueRange,
    close_window: Window,
    mom_smooth_sma: SimpleMovingAverage,
    value: Option<SqueezeProValue>,
}

impl SqueezePro {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(
        bb_length: usize,
        bb_std: f64,
        kc_length: usize,
        kc_scalar_wide: f64,
        kc_scalar_normal: f64,
        kc_scalar_narrow: f64,
        mom_length: usize,
        mom_smooth: usize,
    ) -> TaResult<Self> {
        validate_period(bb_length)?;
        validate_period(kc_length)?;
        validate_period(mom_length)?;
        validate_period(mom_smooth)?;
        if !(bb_std > 0.0) {
            return Err(TaError::InvalidParameter {
                name: "bb_std",
                value: bb_std.to_string(),
                reason: "must be > 0",
            });
        }
        if !(kc_scalar_wide > 0.0 && kc_scalar_normal > 0.0 && kc_scalar_narrow > 0.0) {
            return Err(TaError::InvalidParameter {
                name: "kc_scalar",
                value: format!("{kc_scalar_wide}/{kc_scalar_normal}/{kc_scalar_narrow}"),
                reason: "must all be > 0",
            });
        }
        if !(kc_scalar_wide > kc_scalar_normal && kc_scalar_normal > kc_scalar_narrow) {
            return Err(TaError::InvalidParameter {
                name: "kc_scalar",
                value: format!("{kc_scalar_wide}/{kc_scalar_normal}/{kc_scalar_narrow}"),
                reason: "must satisfy wide > normal > narrow",
            });
        }
        Ok(Self {
            bb_length,
            bb_std,
            kc_length,
            kc_scalar_wide,
            kc_scalar_normal,
            kc_scalar_narrow,
            mom_length,
            mom_smooth,
            bb_mid: SimpleMovingAverage::new(bb_length)?,
            bb_dev: RollingStandardDeviation::new(bb_length, 1.0)?,
            kc_basis: SimpleMovingAverage::new(kc_length)?,
            tr_band: SqueezeTrBand::new(kc_length)?,
            trange: TrueRange::new(),
            close_window: Window::new(mom_length)?,
            mom_smooth_sma: SimpleMovingAverage::new(mom_smooth)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> SqueezeProValue {
        let (bb_lower, bb_upper) = match (self.bb_mid.append(close), self.bb_dev.append(close)) {
            (Some(mid), Some(std)) => (mid - self.bb_std * std, mid + self.bb_std * std),
            _ => (f64::NAN, f64::NAN),
        };

        let kc_basis = self.kc_basis.append(close);
        let tr = self.trange.append(high, low, close).unwrap_or(f64::NAN);
        let kc_band = self.tr_band.append(tr);
        let (
            kc_wide_lower,
            kc_wide_upper,
            kc_norm_lower,
            kc_norm_upper,
            kc_narr_lower,
            kc_narr_upper,
        ) = match (kc_basis, kc_band) {
            (Some(basis), Some(band)) => (
                basis - self.kc_scalar_wide * band,
                basis + self.kc_scalar_wide * band,
                basis - self.kc_scalar_normal * band,
                basis + self.kc_scalar_normal * band,
                basis - self.kc_scalar_narrow * band,
                basis + self.kc_scalar_narrow * band,
            ),
            _ => (f64::NAN, f64::NAN, f64::NAN, f64::NAN, f64::NAN, f64::NAN),
        };

        let mom = self.close_window.push(close).map(|old| close - old);
        let squeeze = mom
            .and_then(|mom| self.mom_smooth_sma.append(mom))
            .unwrap_or(f64::NAN);

        let on_wide = (bb_lower > kc_wide_lower && bb_upper < kc_wide_upper) as u8 as f64;
        let on_normal = (bb_lower > kc_norm_lower && bb_upper < kc_norm_upper) as u8 as f64;
        let on_narrow = (bb_lower > kc_narr_lower && bb_upper < kc_narr_upper) as u8 as f64;
        let off = (bb_lower < kc_wide_lower && bb_upper > kc_wide_upper) as u8 as f64;
        let no = if on_wide == 0.0 && off == 0.0 {
            1.0
        } else {
            0.0
        };

        let value = SqueezeProValue {
            squeeze,
            on_wide,
            on_normal,
            on_narrow,
            off,
            no,
        };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<SqueezeProValue> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.bb_mid.reset();
        self.bb_dev.reset();
        self.kc_basis.reset();
        self.tr_band.reset();
        self.trange.reset();
        self.close_window.clear();
        self.mom_smooth_sma.reset();
        self.value = None;
    }
}

/// Python `round(value, 8)` semantics: round half to even at 1e-8 scale.
pub(crate) fn round8(value: f64) -> f64 {
    const SCALE: f64 = 1e8;
    let scaled = value * SCALE;
    let floor = scaled.floor();
    let diff = scaled - floor;
    let rounded = if diff < 0.5 {
        floor
    } else if diff > 0.5 {
        floor + 1.0
    } else if floor % 2.0 == 0.0 {
        floor
    } else {
        floor + 1.0
    };
    rounded / SCALE
}

/// O(1) amortized rolling extremum (min or max) via a monotonic deque.
///
/// Mirrors pandas `rolling(period).min()/max()`: a NaN input voids the window
/// and the output resumes only after `period` consecutive non-NaN values.
#[derive(Debug, Clone)]
struct RollingExtremum {
    period: usize,
    is_min: bool,
    deque: VecDeque<(usize, f64)>,
    index: usize,
    warm: usize,
    value: Option<f64>,
}

impl RollingExtremum {
    fn new(period: usize, is_min: bool) -> TaResult<Self> {
        if period == 0 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: period.to_string(),
                reason: "must be >= 1",
            });
        }
        Ok(Self {
            period,
            is_min,
            deque: VecDeque::new(),
            index: 0,
            warm: 0,
            value: None,
        })
    }

    fn append(&mut self, x: f64) -> Option<f64> {
        let index = self.index;
        self.index += 1;
        if x.is_nan() {
            self.deque.clear();
            self.warm = 0;
            self.value = None;
            return None;
        }
        self.warm = (self.warm + 1).min(self.period);
        while let Some(&(old, _)) = self.deque.front() {
            if old + self.period <= index {
                self.deque.pop_front();
            } else {
                break;
            }
        }
        while let Some(&(_, value)) = self.deque.back() {
            let dominated = if self.is_min { value >= x } else { value <= x };
            if dominated {
                self.deque.pop_back();
            } else {
                break;
            }
        }
        self.deque.push_back((index, x));
        self.value = (self.warm >= self.period).then(|| self.deque.front().unwrap().1);
        self.value
    }

    fn reset(&mut self) {
        self.deque.clear();
        self.index = 0;
        self.warm = 0;
        self.value = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `SchaffTrendCycleValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SchaffTrendCycleValue {
    pub stc: f64,
    pub macd: f64,
    pub stoch: f64,
}

/// Stateful Schaff Trend Cycle (pandas-ta classic `momentum/stc.py`, theory:
/// Douglas Schaff). MACD line from two SMA-seeded EMAs, then two cascaded
/// stochastics with `round(..., 8)` smoothing at `factor`.
///
/// The `stc`/`stoch` series are fully defined from bar 0 (seeded `0` and
/// carried forward while the rolling windows are cold or non-positive); the
/// `macd` line is NaN until both EMAs are warm.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `SchaffTrendCycle`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SchaffTrendCycle {
    tclength: usize,
    fast: usize,
    slow: usize,
    factor: f64,
    fast_ema: ExponentialMovingAverage,
    slow_ema: ExponentialMovingAverage,
    xmacd_low: RollingExtremum,
    xmacd_high: RollingExtremum,
    pf_low: RollingExtremum,
    pf_high: RollingExtremum,
    stoch1: f64,
    pf: f64,
    stoch2: f64,
    pff: f64,
    value: Option<SchaffTrendCycleValue>,
}

impl SchaffTrendCycle {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(tclength: usize, fast: usize, slow: usize, factor: f64) -> TaResult<Self> {
        validate_period(tclength)?;
        validate_period(fast)?;
        validate_period(slow)?;
        if !(factor > 0.0) {
            return Err(TaError::InvalidParameter {
                name: "factor",
                value: factor.to_string(),
                reason: "must be > 0",
            });
        }
        let (fast, slow) = if slow < fast {
            (slow, fast)
        } else {
            (fast, slow)
        };
        Ok(Self {
            tclength,
            fast,
            slow,
            factor,
            fast_ema: ExponentialMovingAverage::new(fast)?,
            slow_ema: ExponentialMovingAverage::new(slow)?,
            xmacd_low: RollingExtremum::new(tclength, true)?,
            xmacd_high: RollingExtremum::new(tclength, false)?,
            pf_low: RollingExtremum::new(tclength, true)?,
            pf_high: RollingExtremum::new(tclength, false)?,
            stoch1: 0.0,
            pf: 0.0,
            stoch2: 0.0,
            pff: 0.0,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, close: f64) -> SchaffTrendCycleValue {
        let fast = self.fast_ema.append(close);
        let slow = self.slow_ema.append(close);
        let macd = match (fast, slow) {
            (Some(fast), Some(slow)) => fast - slow,
            _ => f64::NAN,
        };

        let lowest = self.xmacd_low.append(macd).unwrap_or(f64::NAN);
        let highest = self.xmacd_high.append(macd).unwrap_or(f64::NAN);
        let range = non_zero(highest - lowest);
        if lowest > 0.0 {
            self.stoch1 = 100.0 * ((macd - lowest) / range);
        }
        self.pf = round8(self.pf + self.factor * (self.stoch1 - self.pf));

        let lowest_pf = self.pf_low.append(self.pf).unwrap_or(f64::NAN);
        let highest_pf = self.pf_high.append(self.pf).unwrap_or(f64::NAN);
        let range_pf = non_zero(highest_pf - lowest_pf);
        if range_pf > 0.0 {
            self.stoch2 = 100.0 * ((self.pf - lowest_pf) / range_pf);
        }
        self.pff = round8(self.pff + self.factor * (self.stoch2 - self.pff));

        let value = SchaffTrendCycleValue {
            stc: self.pff,
            macd,
            stoch: self.pf,
        };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<SchaffTrendCycleValue> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.fast_ema.reset();
        self.slow_ema.reset();
        self.xmacd_low.reset();
        self.xmacd_high.reset();
        self.pf_low.reset();
        self.pf_high.reset();
        self.stoch1 = 0.0;
        self.pf = 0.0;
        self.stoch2 = 0.0;
        self.pff = 0.0;
        self.value = None;
    }
}

/// pandas-ta `non_zero_range(max, min)`: `max − min`, substituting
/// `f64::EPSILON` for an exact zero so flat windows avoid 0/0 division. The
/// package adds the epsilon to the whole series when *any* element is zero;
/// that global perturbation is far below the 1e-8 smoothing precision, so a
/// per-bar guard is equivalent in effect.
pub(crate) fn non_zero(difference: f64) -> f64 {
    if difference == 0.0 {
        f64::EPSILON
    } else {
        difference
    }
}

/// Rolling window sum with pandas `rolling(period).sum()` semantics: NaN
/// inputs are skipped and the output appears once `period` non-NaN values are
/// in the window (used for the Vortex true-range and movement sums).
#[derive(Debug, Clone)]
struct RollingSum {
    period: usize,
    window: Window,
    count: usize,
    sum: f64,
    value: Option<f64>,
}

impl RollingSum {
    fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: period.to_string(),
                reason: "must be >= 1",
            });
        }
        Ok(Self {
            period,
            window: Window::new(period)?,
            count: 0,
            sum: 0.0,
            value: None,
        })
    }

    fn append(&mut self, x: f64) -> Option<f64> {
        if let Some(old) = self.window.push(x) {
            if !old.is_nan() {
                self.sum -= old;
                self.count -= 1;
            }
        }
        if !x.is_nan() {
            self.sum += x;
            self.count += 1;
        }
        self.value = (self.count >= self.period).then_some(self.sum);
        self.value
    }

    fn reset(&mut self) {
        self.window.clear();
        self.count = 0;
        self.sum = 0.0;
        self.value = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `VortexValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct VortexValue {
    pub vp: f64,
    pub vn: f64,
}

/// Stateful Vortex indicator (bukosabino `ta` `trend.VortexIndicator`, theory:
/// Etienne Botes & Douglas Siepman, TASC Jan 2010). +VI/−VI are the ratio of
/// the rolling `n`-sum of positive/negative directional movement to the
/// rolling `n`-sum of true range.
///
/// The first bar's true range uses `close` as its own previous close (the
/// package fills bar 0 with the global close mean, but that value only feeds
/// outputs whose window is not yet complete, so the streaming choice is
/// output-equivalent); the movement terms are NaN at bar 0, so +VI/−VI are
/// first defined at bar `n`.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Vortex`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Vortex {
    period: usize,
    previous_close: Option<f64>,
    previous_low: Option<f64>,
    previous_high: Option<f64>,
    tr_sum: RollingSum,
    vmp_sum: RollingSum,
    vmm_sum: RollingSum,
    value: Option<VortexValue>,
}

impl Vortex {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            period,
            previous_close: None,
            previous_low: None,
            previous_high: None,
            tr_sum: RollingSum::new(period)?,
            vmp_sum: RollingSum::new(period)?,
            vmm_sum: RollingSum::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> VortexValue {
        let (tr, vmp, vmm) = match self.previous_close {
            Some(previous_close) => {
                let tr = (high - low)
                    .max((high - previous_close).abs())
                    .max((low - previous_close).abs());
                let vmp = (high - self.previous_low.unwrap()).abs();
                let vmm = (low - self.previous_high.unwrap()).abs();
                (tr, vmp, vmm)
            }
            None => {
                let tr = (high - low)
                    .max((high - close).abs())
                    .max((low - close).abs());
                (tr, f64::NAN, f64::NAN)
            }
        };
        self.previous_close = Some(close);
        self.previous_low = Some(low);
        self.previous_high = Some(high);

        let trn = self.tr_sum.append(tr);
        let vmp_sum = self.vmp_sum.append(vmp);
        let vmm_sum = self.vmm_sum.append(vmm);
        let vp = match (vmp_sum, trn) {
            (Some(numerator), Some(denominator)) => numerator / denominator,
            _ => f64::NAN,
        };
        let vn = match (vmm_sum, trn) {
            (Some(numerator), Some(denominator)) => numerator / denominator,
            _ => f64::NAN,
        };
        let value = VortexValue { vp, vn };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<VortexValue> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.previous_low = None;
        self.previous_high = None;
        self.tr_sum.reset();
        self.vmp_sum.reset();
        self.vmm_sum.reset();
        self.value = None;
    }
}

/// ROC → SMA pair used by KST: `(close − close[roc]) / close[roc]` fed into an
/// SMA once the shift window is warm.
#[derive(Debug, Clone)]
struct KstRocSma {
    close_window: Window,
    sma: SimpleMovingAverage,
}

impl KstRocSma {
    fn new(roc_period: usize, sma_period: usize) -> TaResult<Self> {
        Ok(Self {
            close_window: Window::new(roc_period)?,
            sma: SimpleMovingAverage::new(sma_period)?,
        })
    }

    fn append(&mut self, close: f64) -> Option<f64> {
        match self.close_window.push(close) {
            Some(previous) => self.sma.append((close - previous) / previous),
            None => None,
        }
    }

    fn reset(&mut self) {
        self.close_window.clear();
        self.sma.reset();
    }
}

/// Rolling mean with pandas `min_periods=0` semantics: defined whenever the
/// window holds at least one non-NaN value (KST signal-line warm-up).
#[derive(Debug, Clone)]
struct RollingMeanMin0 {
    period: usize,
    window: Window,
    count: usize,
    sum: f64,
    value: Option<f64>,
}

impl RollingMeanMin0 {
    fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: period.to_string(),
                reason: "must be >= 1",
            });
        }
        Ok(Self {
            period,
            window: Window::new(period)?,
            count: 0,
            sum: 0.0,
            value: None,
        })
    }

    fn append(&mut self, x: f64) -> Option<f64> {
        if let Some(old) = self.window.push(x) {
            if !old.is_nan() {
                self.sum -= old;
                self.count -= 1;
            }
        }
        if !x.is_nan() {
            self.sum += x;
            self.count += 1;
        }
        self.value = (self.count > 0).then_some(self.sum / self.count as f64);
        self.value
    }

    fn reset(&mut self) {
        self.window.clear();
        self.count = 0;
        self.sum = 0.0;
        self.value = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `KnowSureThingValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct KnowSureThingValue {
    pub kst: f64,
    pub signal: f64,
}

/// Stateful Know Sure Thing (bukosabino `ta` `trend.KSTIndicator`, theory:
/// Martin Pring). `kst = 100·(rocma1 + 2·rocma2 + 3·rocma3 + 4·rocma4)` where
/// each `rocma` is an SMA of the raw ROC ratio over its window; the signal is
/// an `nsig`-period mean of KST (pandas `min_periods=0` warm-up).
///
/// The package fills the ROC shift warm-up with the global close mean; taflow
/// instead leaves those bars NaN, so outputs match the reference exactly from
/// bar `roc4 + sma4 − 1` (KST) and `roc4 + sma4 + nsig − 2` (signal).
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `KnowSureThing`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct KnowSureThing {
    rocs: [KstRocSma; 4],
    nsig: usize,
    signal_state: RollingMeanMin0,
    value: Option<KnowSureThingValue>,
}

impl KnowSureThing {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(
        roc1: usize,
        roc2: usize,
        roc3: usize,
        roc4: usize,
        sma1: usize,
        sma2: usize,
        sma3: usize,
        sma4: usize,
        nsig: usize,
    ) -> TaResult<Self> {
        validate_period(roc1)?;
        validate_period(roc2)?;
        validate_period(roc3)?;
        validate_period(roc4)?;
        validate_period(sma1)?;
        validate_period(sma2)?;
        validate_period(sma3)?;
        validate_period(sma4)?;
        validate_period(nsig)?;
        Ok(Self {
            rocs: [
                KstRocSma::new(roc1, sma1)?,
                KstRocSma::new(roc2, sma2)?,
                KstRocSma::new(roc3, sma3)?,
                KstRocSma::new(roc4, sma4)?,
            ],
            nsig,
            signal_state: RollingMeanMin0::new(nsig)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, close: f64) -> KnowSureThingValue {
        let rocma1 = self.rocs[0].append(close).unwrap_or(f64::NAN);
        let rocma2 = self.rocs[1].append(close).unwrap_or(f64::NAN);
        let rocma3 = self.rocs[2].append(close).unwrap_or(f64::NAN);
        let rocma4 = self.rocs[3].append(close).unwrap_or(f64::NAN);
        let kst = 100.0 * (rocma1 + 2.0 * rocma2 + 3.0 * rocma3 + 4.0 * rocma4);
        let signal = self.signal_state.append(kst).unwrap_or(f64::NAN);
        let value = KnowSureThingValue { kst, signal };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<KnowSureThingValue> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        for roc in &mut self.rocs {
            roc.reset();
        }
        self.signal_state.reset();
        self.value = None;
    }
}

impl ActiveZoneList {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(capacity: usize) -> TaResult<Self> {
        if capacity == 0 {
            return Err(TaError::InvalidParameter {
                name: "capacity",
                value: capacity.to_string(),
                reason: "must be >= 1",
            });
        }
        Ok(Self {
            zones: Vec::with_capacity(capacity),
            capacity,
            index: 0,
        })
    }

    /// Computes or updates `add` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn add(&mut self, top: f64, bottom: f64, flags: u32) -> usize {
        if self.zones.len() == self.capacity {
            self.zones.remove(0);
        }
        let (top, bottom) = if top >= bottom {
            (top, bottom)
        } else {
            (bottom, top)
        };
        self.zones.push(Zone {
            top,
            bottom,
            birth: self.index,
            flags,
        });
        self.zones.len() - 1
    }

    /// Computes or updates `advance` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn advance(&mut self, price: f64, max_age: Option<usize>) -> Vec<bool> {
        self.index = self.index.saturating_add(1);
        let mut mitigated = vec![false; self.zones.len()];
        for (index, zone) in self.zones.iter_mut().enumerate() {
            let expired = max_age.is_some_and(|age| self.index.saturating_sub(zone.birth) > age);
            if !expired && price >= zone.bottom && price <= zone.top {
                zone.flags |= 1;
                mitigated[index] = true;
            }
        }
        self.zones.retain(|zone| {
            let expired = max_age.is_some_and(|age| self.index.saturating_sub(zone.birth) > age);
            !expired && zone.flags & 1 == 0
        });
        mitigated.truncate(self.zones.len());
        mitigated
    }

    /// Computes or updates `zones` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    /// Returns the number of currently active zones.
    pub fn zone_count(&self) -> usize {
        self.zones.len()
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.zones.clear();
        self.index = 0;
    }
}

impl SessionExtrema {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, new_session: bool, high: f64, low: f64) -> SessionExtremaValue {
        if new_session || self.high.is_none() {
            self.high = Some(high);
            self.low = Some(low);
        } else {
            self.high = Some(self.high.expect("session high is initialized").max(high));
            self.low = Some(self.low.expect("session low is initialized").min(low));
        }
        let value = SessionExtremaValue {
            high: self.high.expect("session high is initialized"),
            low: self.low.expect("session low is initialized"),
        };
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<SessionExtremaValue> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.high = None;
        self.low = None;
        self.value = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `SwingValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SwingValue {
    pub signal: f64,
    pub level: f64,
    pub bars_since: f64,
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `SwingHighLow`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SwingHighLow {
    highs: VecDeque<f64>,
    lows: VecDeque<f64>,
    length: usize,
    bars_since: Option<usize>,
    value: Option<SwingValue>,
}

impl SwingHighLow {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(length: usize) -> TaResult<Self> {
        validate_period(length)?;
        let capacity = length.saturating_mul(2).saturating_add(1);
        Ok(Self {
            highs: VecDeque::with_capacity(capacity),
            lows: VecDeque::with_capacity(capacity),
            length,
            bars_since: None,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64) -> Option<SwingValue> {
        let capacity = self.length * 2 + 1;
        if self.highs.len() == capacity {
            self.highs.pop_front();
            self.lows.pop_front();
        }
        self.highs.push_back(high);
        self.lows.push_back(low);

        if self.highs.len() < capacity {
            self.value = None;
            return None;
        }
        let center_high = self.highs[self.length];
        let center_low = self.lows[self.length];
        let is_high = center_high >= self.highs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let is_low = center_low <= self.lows.iter().copied().fold(f64::INFINITY, f64::min);
        let (signal, level) = match (is_high, is_low) {
            (true, false) => (1.0, center_high),
            (false, true) => (-1.0, center_low),
            _ => (f64::NAN, f64::NAN),
        };
        self.bars_since = if signal.is_nan() {
            self.bars_since.map(|bars| bars + 1)
        } else {
            Some(0)
        };
        let value = SwingValue {
            signal,
            level,
            bars_since: self.bars_since.map_or(f64::NAN, |bars| bars as f64),
        };
        self.value = Some(value);
        Some(value)
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<SwingValue> {
        self.value
    }

    /// Return the current bars-since result, if available.
    ///
    pub fn bars_since(&self) -> Option<f64> {
        self.bars_since.map(|bars| bars as f64)
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.highs.clear();
        self.lows.clear();
        self.bars_since = None;
        self.value = None;
    }
}

#[derive(Debug, Clone)]
struct RollingMean {
    values: VecDeque<f64>,
    timeperiod: usize,
    sum: f64,
    value: Option<f64>,
}

impl RollingMean {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self {
            values: VecDeque::with_capacity(timeperiod),
            timeperiod,
            sum: 0.0,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod {
            self.sum -= self.values.pop_front().expect("ring is full");
        }
        self.values.push_back(input);
        self.sum += input;
        self.value = if self.values.len() == self.timeperiod {
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
        self.values.clear();
        self.sum = 0.0;
        self.value = None;
    }
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingQuantile`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingQuantile {
    values: VecDeque<f64>,
    timeperiod: usize,
    quantile: f64,
    value: Option<f64>,
}

impl RollingQuantile {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize, quantile: f64) -> TaResult<Self> {
        validate_period(timeperiod)?;
        validate_quantile(quantile)?;
        Ok(Self {
            values: VecDeque::with_capacity(timeperiod),
            timeperiod,
            quantile,
            value: None,
        })
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod {
            self.values.pop_front();
        }
        self.values.push_back(input);
        self.value = if self.values.len() == self.timeperiod {
            let mut sorted: Vec<f64> = self.values.iter().copied().collect();
            sorted.sort_by(f64::total_cmp);
            let position = self.quantile * (self.timeperiod - 1) as f64;
            let lower = position.floor() as usize;
            let upper = position.ceil() as usize;
            Some(sorted[lower] + (sorted[upper] - sorted[lower]) * (position - lower as f64))
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
        self.values.clear();
        self.value = None;
    }
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingRank`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingRank {
    values: VecDeque<f64>,
    timeperiod: usize,
    value: Option<f64>,
}

impl RollingRank {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self {
            values: VecDeque::with_capacity(timeperiod),
            timeperiod,
            value: None,
        })
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod {
            self.values.pop_front();
        }
        self.values.push_back(input);
        self.value = if self.values.len() == self.timeperiod {
            let less = self.values.iter().filter(|&&value| value < input).count();
            let equal = self.values.iter().filter(|&&value| value == input).count();
            Some((less as f64 + equal as f64) / self.timeperiod as f64)
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
        self.values.clear();
        self.value = None;
    }
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingZScore`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingZScore {
    values: VecDeque<f64>,
    timeperiod: usize,
    value: Option<f64>,
}

impl RollingZScore {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self {
            values: VecDeque::with_capacity(timeperiod),
            timeperiod,
            value: None,
        })
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod {
            self.values.pop_front();
        }
        self.values.push_back(input);
        self.value = if self.values.len() == self.timeperiod {
            let mean = self.values.iter().sum::<f64>() / self.timeperiod as f64;
            let variance = self
                .values
                .iter()
                .map(|&value| (value - mean).powi(2))
                .sum::<f64>()
                / self.timeperiod as f64;
            Some(if variance > 0.0 {
                (input - mean) / variance.sqrt()
            } else {
                0.0
            })
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
        self.values.clear();
        self.value = None;
    }
}

macro_rules! rolling_moment_operator {
    ($name:ident, $formula:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            values: VecDeque<f64>,
            timeperiod: usize,
            nobs: usize,
            mean: f64,
            m2: f64,
            m3: f64,
            m4: f64,
            value: Option<f64>,
        }
        impl $name {
            /// Computes or updates `new` through the native Rust kernel.
            ///
            /// Parameters are the typed series and configuration values in the signature.
            ///
            /// Returns the computed value, aligned history, or a validation error.
            pub fn new(timeperiod: usize) -> TaResult<Self> {
                validate_period(timeperiod)?;
                Ok(Self {
                    values: VecDeque::with_capacity(timeperiod),
                    timeperiod,
                    nobs: 0,
                    mean: 0.0,
                    m2: 0.0,
                    m3: 0.0,
                    m4: 0.0,
                    value: None,
                })
            }
            /// Computes or updates `append` through the native Rust kernel.
            ///
            /// Parameters are the typed series and configuration values in the signature.
            ///
            /// Returns the computed value, aligned history, or a validation error.
            pub fn append(&mut self, input: f64) -> Option<f64> {
                if self.values.len() == self.timeperiod {
                    let old = self.values.pop_front().expect("full moment window");
                    let n = (self.nobs - 1) as f64;
                    let delta = old - self.mean;
                    let delta_n = delta / n;
                    let term1 = delta_n * delta * (n + 1.0);
                    let old_m2 = self.m2;
                    let old_m3 = self.m3;
                    self.m4 += delta_n
                        * (4.0 * old_m3
                            + delta_n * (6.0 * old_m2 - term1 * (n * n + 3.0 * n + 3.0)));
                    self.m3 = old_m3 - delta_n * (term1 * (n + 2.0) - 3.0 * old_m2);
                    self.m2 = old_m2 - term1;
                    self.mean -= delta_n;
                    self.nobs -= 1;
                }
                self.values.push_back(input);
                let n_old = self.nobs as f64;
                let n = n_old + 1.0;
                let delta = input - self.mean;
                let delta_n = delta / n;
                let term1 = delta * delta_n * n_old;
                let old_m2 = self.m2;
                let old_m3 = self.m3;
                self.m4 += delta_n
                    * (-4.0 * old_m3 + delta_n * (6.0 * old_m2 + term1 * (n * n - 3.0 * n + 3.0)));
                self.m3 += delta_n * (term1 * (n - 2.0) - 3.0 * old_m2);
                self.m2 = old_m2 + term1;
                self.mean += delta_n;
                self.nobs += 1;
                self.value = if self.nobs == self.timeperiod {
                    Some($formula(self.nobs as f64, self.m2, self.m3, self.m4))
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
                self.values.clear();
                self.nobs = 0;
                self.mean = 0.0;
                self.m2 = 0.0;
                self.m3 = 0.0;
                self.m4 = 0.0;
                self.value = None;
            }
        }
    };
}

rolling_moment_operator!(RollingSkew, |n: f64, m2: f64, m3: f64, _m4: f64| {
    if m2 > 0.0 {
        n.sqrt() * m3 / m2.powf(1.5)
    } else {
        0.0
    }
});

rolling_moment_operator!(RollingKurtosis, |n: f64, m2: f64, _m3: f64, m4: f64| {
    if m2 > 0.0 {
        n * m4 / m2.powi(2) - 3.0
    } else {
        0.0
    }
});

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingInterquartileRange`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingInterquartileRange {
    quantile: RollingQuantile,
    value: Option<f64>,
}

impl RollingInterquartileRange {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            quantile: RollingQuantile::new(timeperiod, 0.25)?,
            value: None,
        })
    }
    /// Append one value and return the current interquartile range.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.quantile.append(input);
        self.value = if self.quantile.values.len() == self.quantile.timeperiod {
            let mut sorted: Vec<f64> = self.quantile.values.iter().copied().collect();
            sorted.sort_by(f64::total_cmp);
            let quantile = |q: f64| {
                let position = q * (sorted.len() - 1) as f64;
                let lower = position.floor() as usize;
                let upper = position.ceil() as usize;
                sorted[lower] + (sorted[upper] - sorted[lower]) * (position - lower as f64)
            };
            Some(quantile(0.75) - quantile(0.25))
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
        self.quantile.reset();
        self.value = None;
    }
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingCov`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingCov {
    values: VecDeque<(f64, f64)>,
    timeperiod: usize,
    value: Option<f64>,
}

impl RollingCov {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self {
            values: VecDeque::with_capacity(timeperiod),
            timeperiod,
            value: None,
        })
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, left: f64, right: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod {
            self.values.pop_front();
        }
        self.values.push_back((left, right));
        self.value = if self.values.len() == self.timeperiod {
            let n = self.timeperiod as f64;
            let left_mean = self.values.iter().map(|&(left, _)| left).sum::<f64>() / n;
            let right_mean = self.values.iter().map(|&(_, right)| right).sum::<f64>() / n;
            Some(
                self.values
                    .iter()
                    .map(|&(left, right)| (left - left_mean) * (right - right_mean))
                    .sum::<f64>()
                    / n,
            )
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
        self.values.clear();
        self.value = None;
    }
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingWinsorize`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingWinsorize {
    values: VecDeque<f64>,
    timeperiod: usize,
    lower: f64,
    upper: f64,
    value: Option<f64>,
}

impl RollingWinsorize {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize, lower: f64, upper: f64) -> TaResult<Self> {
        validate_period(timeperiod)?;
        validate_quantile(lower)?;
        validate_quantile(upper)?;
        if lower > upper {
            return Err(TaError::InvalidParameter {
                name: "lower/upper",
                value: format!("{lower}/{upper}"),
                reason: "lower must be <= upper",
            });
        }
        Ok(Self {
            values: VecDeque::with_capacity(timeperiod),
            timeperiod,
            lower,
            upper,
            value: None,
        })
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod {
            self.values.pop_front();
        }
        self.values.push_back(input);
        self.value = if self.values.len() == self.timeperiod {
            let mut sorted: Vec<f64> = self.values.iter().copied().collect();
            sorted.sort_by(f64::total_cmp);
            let quantile = |q: f64| {
                let position = q * (sorted.len() - 1) as f64;
                let lower = position.floor() as usize;
                let upper = position.ceil() as usize;
                sorted[lower] + (sorted[upper] - sorted[lower]) * (position - lower as f64)
            };
            Some(input.max(quantile(self.lower)).min(quantile(self.upper)))
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
        self.values.clear();
        self.value = None;
    }
}

pub(crate) fn ewm_alpha(timeperiod: usize) -> TaResult<f64> {
    validate_period(timeperiod)?;
    Ok(2.0 / (timeperiod as f64 + 1.0))
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `ExponentiallyWeightedVariance`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct ExponentiallyWeightedVariance {
    alpha: f64,
    mean: Option<f64>,
    variance: f64,
    value: Option<f64>,
}

impl ExponentiallyWeightedVariance {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            alpha: ewm_alpha(timeperiod)?,
            mean: None,
            variance: 0.0,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, input: f64) -> f64 {
        let variance = match self.mean {
            None => {
                self.mean = Some(input);
                0.0
            }
            Some(previous) => {
                let delta = input - previous;
                self.mean = Some(previous + self.alpha * delta);
                (1.0 - self.alpha) * (self.variance + self.alpha * delta * delta)
            }
        };
        self.variance = variance;
        self.value = Some(variance);
        variance
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
        self.mean = None;
        self.variance = 0.0;
        self.value = None;
    }
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `ExponentiallyWeightedStandardDeviation`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct ExponentiallyWeightedStandardDeviation {
    variance: ExponentiallyWeightedVariance,
    value: Option<f64>,
}

impl ExponentiallyWeightedStandardDeviation {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            variance: ExponentiallyWeightedVariance::new(timeperiod)?,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, input: f64) -> f64 {
        let value = self.variance.append(input).sqrt();
        self.value = Some(value);
        value
    }
    /// Return the latest computed result, if warm-up is complete.
    ///
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.variance.reset();
        self.value = None;
    }
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `ExponentiallyWeightedCovariance`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct ExponentiallyWeightedCovariance {
    alpha: f64,
    mean0: Option<f64>,
    mean1: Option<f64>,
    var0: f64,
    var1: f64,
    covariance: f64,
    value: Option<f64>,
}

impl ExponentiallyWeightedCovariance {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            alpha: ewm_alpha(timeperiod)?,
            mean0: None,
            mean1: None,
            var0: 0.0,
            var1: 0.0,
            covariance: 0.0,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, left: f64, right: f64) -> f64 {
        let covariance = match (self.mean0, self.mean1) {
            (Some(previous0), Some(previous1)) => {
                let delta0 = left - previous0;
                let delta1 = right - previous1;
                self.mean0 = Some(previous0 + self.alpha * delta0);
                self.mean1 = Some(previous1 + self.alpha * delta1);
                self.var0 = (1.0 - self.alpha) * (self.var0 + self.alpha * delta0 * delta0);
                self.var1 = (1.0 - self.alpha) * (self.var1 + self.alpha * delta1 * delta1);
                (1.0 - self.alpha) * (self.covariance + self.alpha * delta0 * delta1)
            }
            _ => {
                self.mean0 = Some(left);
                self.mean1 = Some(right);
                0.0
            }
        };
        self.covariance = covariance;
        self.value = Some(covariance);
        covariance
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
        self.mean0 = None;
        self.mean1 = None;
        self.var0 = 0.0;
        self.var1 = 0.0;
        self.covariance = 0.0;
        self.value = None;
    }
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `ExponentiallyWeightedCorrelation`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct ExponentiallyWeightedCorrelation {
    covariance: ExponentiallyWeightedCovariance,
    value: Option<f64>,
}

impl ExponentiallyWeightedCorrelation {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        Ok(Self {
            covariance: ExponentiallyWeightedCovariance::new(timeperiod)?,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, left: f64, right: f64) -> f64 {
        self.covariance.append(left, right);
        let denominator = (self.covariance.var0 * self.covariance.var1).sqrt();
        let value = if denominator > 0.0 {
            self.covariance.covariance / denominator
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
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.covariance.reset();
        self.value = None;
    }
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Drawdown`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Drawdown {
    maximum: CumulativeMaximum,
    value: Option<f64>,
}
impl Drawdown {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            maximum: CumulativeMaximum::new(),
            value: None,
        }
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, input: f64) -> f64 {
        let maximum = self.maximum.append(input);
        let value = if maximum != 0.0 {
            input / maximum - 1.0
        } else {
            0.0
        };
        self.value = Some(value);
        value
    }
    /// Return the latest computed result, if warm-up is complete.
    ///
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.maximum.reset();
        self.value = None;
    }
}
impl Default for Drawdown {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! rolling_risk_operator {
    ($name:ident, $formula:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            values: VecDeque<f64>,
            timeperiod: usize,
            value: Option<f64>,
        }
        impl $name {
            /// Computes or updates `new` through the native Rust kernel.
            ///
            /// Parameters are the typed series and configuration values in the signature.
            ///
            /// Returns the computed value, aligned history, or a validation error.
            pub fn new(timeperiod: usize) -> TaResult<Self> {
                validate_period(timeperiod)?;
                Ok(Self {
                    values: VecDeque::with_capacity(timeperiod),
                    timeperiod,
                    value: None,
                })
            }
            /// Append one causal observation and return the latest result.
            ///
            pub fn append(&mut self, input: f64) -> Option<f64> {
                if self.values.len() == self.timeperiod {
                    self.values.pop_front();
                }
                self.values.push_back(input);
                self.value = if self.values.len() == self.timeperiod {
                    Some($formula(&self.values))
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
                self.values.clear();
                self.value = None;
            }
        }
    };
}

pub(crate) fn mean(values: &VecDeque<f64>) -> f64 {
    values.iter().sum::<f64>() / values.len() as f64
}

pub(crate) fn weighted_mean(values: &VecDeque<f64>) -> f64 {
    let denominator = (values.len() * (values.len() + 1) / 2) as f64;
    values
        .iter()
        .enumerate()
        .map(|(i, &v)| v * (i + 1) as f64)
        .sum::<f64>()
        / denominator
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `HullMovingAverage`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct HullMovingAverage {
    raw: VecDeque<f64>,
    intermediate: VecDeque<f64>,
    period: usize,
    half: usize,
    smooth: usize,
    value: Option<f64>,
}
impl HullMovingAverage {
    /// Create a new empty state.
    ///
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        let half = (period / 2).max(1);
        let smooth = (period as f64).sqrt().floor() as usize;
        Ok(Self {
            raw: VecDeque::with_capacity(period),
            intermediate: VecDeque::with_capacity(smooth.max(1)),
            period,
            half,
            smooth: smooth.max(1),
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.raw.len() == self.period {
            self.raw.pop_front();
        }
        self.raw.push_back(input);
        if self.raw.len() >= self.half && self.raw.len() >= self.period {
            let half = weighted_mean(
                &self
                    .raw
                    .iter()
                    .skip(self.period - self.half)
                    .copied()
                    .collect(),
            );
            let full = weighted_mean(&self.raw);
            if self.intermediate.len() == self.smooth {
                self.intermediate.pop_front();
            }
            self.intermediate.push_back(2.0 * half - full);
            self.value =
                (self.intermediate.len() == self.smooth).then(|| weighted_mean(&self.intermediate));
        } else {
            self.value = None
        }
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
        self.raw.clear();
        self.intermediate.clear();
        self.value = None;
    }
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `VolumeWeightedMovingAverage`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct VolumeWeightedMovingAverage {
    prices: VecDeque<f64>,
    volumes: VecDeque<f64>,
    period: usize,
    value: Option<f64>,
}
impl VolumeWeightedMovingAverage {
    /// Create a new empty state.
    ///
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            prices: VecDeque::with_capacity(period),
            volumes: VecDeque::with_capacity(period),
            period,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, price: f64, volume: f64) -> Option<f64> {
        if self.prices.len() == self.period {
            self.prices.pop_front();
            self.volumes.pop_front();
        }
        self.prices.push_back(price);
        self.volumes.push_back(volume);
        self.value = (self.prices.len() == self.period).then(|| {
            let volume = self.volumes.iter().sum::<f64>();
            if volume != 0.0 {
                self.prices
                    .iter()
                    .zip(&self.volumes)
                    .map(|(&p, &v)| p * v)
                    .sum::<f64>()
                    / volume
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
        self.prices.clear();
        self.volumes.clear();
        self.value = None;
    }
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `ZeroLagExponentialMovingAverage`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct ZeroLagExponentialMovingAverage {
    values: VecDeque<f64>,
    period: usize,
    lag: usize,
    alpha: f64,
    ema: Option<f64>,
    value: Option<f64>,
}
impl ZeroLagExponentialMovingAverage {
    /// Create a new empty state.
    ///
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            values: VecDeque::with_capacity((period / 2).max(1)),
            period,
            lag: (period - 1) / 2,
            alpha: 2.0 / (period as f64 + 1.0),
            ema: None,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.lag.max(1) {
            self.values.pop_front();
        }
        self.values.push_back(input);
        if self.values.len() <= self.lag {
            self.value = None
        } else {
            let lagged = self.values.front().copied().unwrap_or(input);
            let adjusted = 2.0 * input - lagged;
            self.ema = Some(match self.ema {
                Some(previous) => previous + self.alpha * (adjusted - previous),
                None => adjusted,
            });
            self.value = self.ema;
        }
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
        self.ema = None;
        self.value = None;
    }
}

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

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `TrueStrengthIndex`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct TrueStrengthIndex {
    previous: Option<f64>,
    fast: usize,
    slow: usize,
    alpha_fast: f64,
    alpha_slow: f64,
    momentum: Option<f64>,
    absolute: Option<f64>,
    value: Option<f64>,
}
impl TrueStrengthIndex {
    /// Create a new empty state.
    ///
    pub fn new(fast: usize, slow: usize) -> TaResult<Self> {
        validate_period(fast)?;
        validate_period(slow)?;
        Ok(Self {
            previous: None,
            fast,
            slow,
            alpha_fast: 2.0 / (fast as f64 + 1.0),
            alpha_slow: 2.0 / (slow as f64 + 1.0),
            momentum: None,
            absolute: None,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, input: f64) -> Option<f64> {
        let previous = self.previous.replace(input)?;
        let change = input - previous;
        let abs = change.abs();
        let m1 = self
            .momentum
            .map_or(change, |v| v + self.alpha_fast * (change - v));
        let a1 = self
            .absolute
            .map_or(abs, |v| v + self.alpha_fast * (abs - v));
        self.momentum = Some(m1);
        self.absolute = Some(a1);
        let m2 = self.momentum.map_or(m1, |v| v + self.alpha_slow * (m1 - v));
        let a2 = self.absolute.map_or(a1, |v| v + self.alpha_slow * (a1 - v));
        let value = if a2 != 0.0 {
            Some(100.0 * m2 / a2)
        } else {
            Some(0.0)
        };
        self.value = value;
        value
    }
    /// Return the latest computed result, if warm-up is complete.
    ///
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Reset the state and clear its accumulated history.
    ///
    pub fn reset(&mut self) {
        self.previous = None;
        self.momentum = None;
        self.absolute = None;
        self.value = None;
    }
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `AwesomeOscillator`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct AwesomeOscillator {
    fast: usize,
    slow: usize,
    values: VecDeque<f64>,
    value: Option<f64>,
}
impl AwesomeOscillator {
    /// Create a new empty state.
    ///
    pub fn new(fast: usize, slow: usize) -> TaResult<Self> {
        validate_period(fast)?;
        validate_period(slow)?;
        if fast > slow {
            return Err(TaError::InvalidParameter {
                name: "fast/slow",
                value: format!("{fast}/{slow}"),
                reason: "fast must be <= slow",
            });
        }
        Ok(Self {
            fast,
            slow,
            values: VecDeque::with_capacity(slow),
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        if self.values.len() == self.slow {
            self.values.pop_front();
        }
        self.values.push_back((high + low) * 0.5);
        self.value = (self.values.len() == self.slow).then(|| {
            let fast = self.values.iter().rev().take(self.fast).sum::<f64>() / self.fast as f64;
            let slow = self.values.iter().sum::<f64>() / self.slow as f64;
            fast - slow
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

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `FisherTransform`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct FisherTransform {
    period: usize,
    values: VecDeque<f64>,
    previous: f64,
    value: Option<f64>,
}
impl FisherTransform {
    /// Create a new empty state.
    ///
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            period,
            values: VecDeque::with_capacity(period),
            previous: 0.0,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        if self.values.len() == self.period {
            self.values.pop_front();
        }
        self.values.push_back((high + low) * 0.5);
        self.value = (self.values.len() == self.period).then(|| {
            let high = self
                .values
                .iter()
                .copied()
                .fold(f64::NEG_INFINITY, f64::max);
            let low = self.values.iter().copied().fold(f64::INFINITY, f64::min);
            let normalized = if high != low {
                2.0 * ((self.values.back().copied().unwrap() - low) / (high - low) - 0.5)
            } else {
                0.0
            };
            let x = (0.66 * normalized + 0.67 * self.previous).clamp(-0.999, 0.999);
            self.previous = x;
            0.5 * ((1.0 + x) / (1.0 - x)).ln()
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
        self.previous = 0.0;
        self.value = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `DonchianValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct DonchianValue {
    pub upper: f64,
    pub lower: f64,
    pub middle: f64,
}
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Donchian`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Donchian {
    highs: VecDeque<f64>,
    lows: VecDeque<f64>,
    period: usize,
    value: Option<DonchianValue>,
}
impl Donchian {
    /// Create a new empty state.
    ///
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            highs: VecDeque::with_capacity(period),
            lows: VecDeque::with_capacity(period),
            period,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, high: f64, low: f64) -> Option<DonchianValue> {
        if self.highs.len() == self.period {
            self.highs.pop_front();
            self.lows.pop_front();
        }
        self.highs.push_back(high);
        self.lows.push_back(low);
        self.value = (self.highs.len() == self.period).then(|| {
            let upper = self.highs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            let lower = self.lows.iter().copied().fold(f64::INFINITY, f64::min);
            DonchianValue {
                upper,
                lower,
                middle: (upper + lower) * 0.5,
            }
        });
        self.value
    }
    /// Return the latest computed result, if warm-up is complete.
    ///
    pub fn value(&self) -> Option<DonchianValue> {
        self.value
    }
    /// Reset the state and clear its accumulated history.
    ///
    pub fn reset(&mut self) {
        self.highs.clear();
        self.lows.clear();
        self.value = None;
    }
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `UlcerIndex`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct UlcerIndex {
    values: VecDeque<f64>,
    period: usize,
    value: Option<f64>,
}
impl UlcerIndex {
    /// Create a new empty state.
    ///
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            values: VecDeque::with_capacity(period),
            period,
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
            let mut peak = f64::NEG_INFINITY;
            let sum = self
                .values
                .iter()
                .map(|&v| {
                    peak = peak.max(v);
                    let drawdown = if peak != 0.0 {
                        100.0 * (v - peak) / peak
                    } else {
                        0.0
                    };
                    drawdown * drawdown
                })
                .sum::<f64>();
            (sum / self.period as f64).sqrt()
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

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingVolumeWeightedAveragePrice`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingVolumeWeightedAveragePrice {
    prices: VecDeque<f64>,
    volumes: VecDeque<f64>,
    period: usize,
    value: Option<f64>,
}
impl RollingVolumeWeightedAveragePrice {
    /// Create a new empty state.
    ///
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            prices: VecDeque::with_capacity(period),
            volumes: VecDeque::with_capacity(period),
            period,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> Option<f64> {
        if self.prices.len() == self.period {
            self.prices.pop_front();
            self.volumes.pop_front();
        }
        self.prices.push_back((high + low + close) / 3.0);
        self.volumes.push_back(volume);
        self.value = (self.prices.len() == self.period).then(|| {
            let total = self.volumes.iter().sum::<f64>();
            if total != 0.0 {
                self.prices
                    .iter()
                    .zip(&self.volumes)
                    .map(|(&p, &v)| p * v)
                    .sum::<f64>()
                    / total
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
        self.prices.clear();
        self.volumes.clear();
        self.value = None;
    }
}
#[derive(Debug, Clone)]
/// Stateful Force Index derived from close-to-close change and volume.
///
/// The state preserves warm-up behavior and supports append/reset updates.
pub struct ForceIndex {
    previous: Option<f64>,
    value: Option<f64>,
}
impl ForceIndex {
    /// Create a new empty state.
    ///
    pub fn new() -> Self {
        Self {
            previous: None,
            value: None,
        }
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, close: f64, volume: f64) -> Option<f64> {
        let previous = self.previous.replace(close)?;
        self.value = Some((close - previous) * volume);
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
        self.previous = None;
        self.value = None;
    }
}
impl Default for ForceIndex {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone)]
/// Stateful Ease of Movement oscillator using aligned high, low, and volume.
///
/// The result is causal and retains the latest value between updates.
pub struct EaseOfMovement {
    previous_midpoint: Option<f64>,
    value: Option<f64>,
}
impl EaseOfMovement {
    /// Create a new empty state.
    ///
    pub fn new() -> Self {
        Self {
            previous_midpoint: None,
            value: None,
        }
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, high: f64, low: f64, volume: f64) -> Option<f64> {
        let midpoint = (high + low) * 0.5;
        let previous = self.previous_midpoint.replace(midpoint)?;
        self.value = Some(if volume != 0.0 {
            (midpoint - previous) * (high - low) / volume
        } else {
            0.0
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
        self.previous_midpoint = None;
        self.value = None;
    }
}
impl Default for EaseOfMovement {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `SignalDelay`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct SignalDelay {
    values: VecDeque<f64>,
    period: usize,
    value: Option<f64>,
}
impl SignalDelay {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            values: VecDeque::with_capacity(period),
            period,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.value = if self.values.len() == self.period {
            let value = self.values.pop_front();
            self.values.push_back(input);
            value
        } else {
            self.values.push_back(input);
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
        self.values.clear();
        self.value = None;
    }
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `PositionHold`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct PositionHold {
    position: f64,
    value: Option<f64>,
}
impl PositionHold {
    /// Create a new empty state.
    ///
    pub fn new() -> Self {
        Self {
            position: 0.0,
            value: None,
        }
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, input: f64) -> f64 {
        if input != 0.0 {
            self.position = input;
        }
        self.value = Some(self.position);
        self.position
    }
    /// Return the latest computed result, if warm-up is complete.
    ///
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Reset the state and clear its accumulated history.
    ///
    pub fn reset(&mut self) {
        self.position = 0.0;
        self.value = None;
    }
}
impl Default for PositionHold {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone)]
/// Stateful entry/exit signal helper with causal position transitions.
///
/// The state emits aligned signals and can be reset for replay.
pub struct EntryExit {
    position: f64,
    value: Option<f64>,
}
impl EntryExit {
    /// Create a new empty state.
    ///
    pub fn new() -> Self {
        Self {
            position: 0.0,
            value: None,
        }
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, entry: bool, exit: bool) -> f64 {
        if entry && !exit {
            self.position = 1.0
        } else if exit && !entry {
            self.position = -1.0
        }
        self.value = Some(self.position);
        self.position
    }
    /// Return the latest computed result, if warm-up is complete.
    ///
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Reset the state and clear its accumulated history.
    ///
    pub fn reset(&mut self) {
        self.position = 0.0;
        self.value = None;
    }
}
impl Default for EntryExit {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Crossover`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Crossover {
    previous_left: Option<f64>,
    previous_right: Option<f64>,
    value: Option<f64>,
}
impl Crossover {
    /// Create a new empty state.
    ///
    pub fn new() -> Self {
        Self {
            previous_left: None,
            previous_right: None,
            value: None,
        }
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, left: f64, right: f64) -> f64 {
        let value = match (self.previous_left, self.previous_right) {
            (Some(pl), Some(pr)) if pl <= pr && left > right => 1.0,
            _ => 0.0,
        };
        self.previous_left = Some(left);
        self.previous_right = Some(right);
        self.value = Some(value);
        value
    }
    /// Return the latest computed result, if warm-up is complete.
    ///
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Reset the state and clear its accumulated history.
    ///
    pub fn reset(&mut self) {
        self.previous_left = None;
        self.previous_right = None;
        self.value = None;
    }
}
impl Default for Crossover {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Crossunder`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Crossunder {
    previous_left: Option<f64>,
    previous_right: Option<f64>,
    value: Option<f64>,
}
impl Crossunder {
    /// Create a new empty state.
    ///
    pub fn new() -> Self {
        Self {
            previous_left: None,
            previous_right: None,
            value: None,
        }
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, left: f64, right: f64) -> f64 {
        let value = match (self.previous_left, self.previous_right) {
            (Some(pl), Some(pr)) if pl >= pr && left < right => 1.0,
            _ => 0.0,
        };
        self.previous_left = Some(left);
        self.previous_right = Some(right);
        self.value = Some(value);
        value
    }
    /// Return the latest computed result, if warm-up is complete.
    ///
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Reset the state and clear its accumulated history.
    ///
    pub fn reset(&mut self) {
        self.previous_left = None;
        self.previous_right = None;
        self.value = None;
    }
}
impl Default for Crossunder {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Cross`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Cross {
    crossover: Crossover,
    crossunder: Crossunder,
    value: Option<f64>,
}
impl Cross {
    /// Create a new empty state.
    ///
    pub fn new() -> Self {
        Self {
            crossover: Crossover::new(),
            crossunder: Crossunder::new(),
            value: None,
        }
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, left: f64, right: f64) -> f64 {
        let value =
            (self.crossover.append(left, right) + self.crossunder.append(left, right)).min(1.0);
        self.value = Some(value);
        value
    }
    /// Return the latest computed result, if warm-up is complete.
    ///
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Reset the state and clear its accumulated history.
    ///
    pub fn reset(&mut self) {
        self.crossover.reset();
        self.crossunder.reset();
        self.value = None;
    }
}
impl Default for Cross {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! direction_operator {
    ($name:ident, $predicate:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            values: VecDeque<f64>,
            period: usize,
            value: Option<f64>,
        }
        impl $name {
            /// Create a direction detector with the requested comparison period.
            pub fn new(period: usize) -> TaResult<Self> {
                validate_period(period)?;
                Ok(Self {
                    values: VecDeque::with_capacity(period + 1),
                    period,
                    value: None,
                })
            }
            /// Append a value and return the causal direction flag.
            pub fn append(&mut self, input: f64) -> Option<f64> {
                if self.values.len() == self.period + 1 {
                    self.values.pop_front();
                }
                self.values.push_back(input);
                self.value = (self.values.len() == self.period + 1).then(|| {
                    if $predicate(input, self.values.front().copied().unwrap()) {
                        1.0
                    } else {
                        0.0
                    }
                });
                self.value
            }
            /// Return the latest direction flag.
            pub fn value(&self) -> Option<f64> {
                self.value
            }
            /// Clear the comparison history.
            pub fn reset(&mut self) {
                self.values.clear();
                self.value = None;
            }
        }
    };
}
direction_operator!(Rising, |current: f64, previous: f64| current > previous);
direction_operator!(Falling, |current: f64, previous: f64| current < previous);

rolling_risk_operator!(RollingSharpe, |values: &VecDeque<f64>| {
    let average = mean(values);
    let variance = values
        .iter()
        .map(|&value| (value - average).powi(2))
        .sum::<f64>()
        / values.len() as f64;
    if variance > 0.0 {
        average / variance.sqrt()
    } else {
        0.0
    }
});

rolling_risk_operator!(RollingSortino, |values: &VecDeque<f64>| {
    let average = mean(values);
    let downside = values
        .iter()
        .map(|&value| value.min(0.0).powi(2))
        .sum::<f64>()
        / values.len() as f64;
    if downside > 0.0 {
        average / downside.sqrt()
    } else {
        0.0
    }
});

rolling_risk_operator!(RollingCalmar, |values: &VecDeque<f64>| {
    let average = mean(values);
    let mut peak = values[0];
    let mut drawdown: f64 = 0.0;
    for &value in values {
        peak = peak.max(value);
        drawdown = drawdown.min(if peak != 0.0 { value / peak - 1.0 } else { 0.0 });
    }
    if drawdown < 0.0 {
        average / -drawdown
    } else {
        0.0
    }
});

/// Stateful Mass Index (Dorsey): rolling sum of the ratio between a short EMA
/// of the high-low range and an EMA of that EMA.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `MassIndex`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct MassIndex {
    ema_range: MassEma,
    ema_signal: MassEma,
    ratio_sum: crate::stream::RollingSum,
    value: Option<f64>,
}

#[derive(Debug, Clone)]
struct MassEma {
    period: usize,
    alpha: f64,
    count: usize,
    value: Option<f64>,
}

impl MassEma {
    fn new(period: usize) -> Self {
        Self {
            period,
            alpha: 2.0 / (period as f64 + 1.0),
            count: 0,
            value: None,
        }
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        self.count += 1;
        let value = self
            .value
            .map_or(input, |previous| previous + self.alpha * (input - previous));
        self.value = Some(value);
        (self.count >= self.period).then_some(value)
    }

    fn reset(&mut self) {
        self.count = 0;
        self.value = None;
    }
}

impl MassIndex {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(ema_period: usize, sum_period: usize) -> TaResult<Self> {
        validate_period(ema_period)?;
        validate_period(sum_period)?;
        Ok(Self {
            ema_range: MassEma::new(ema_period),
            ema_signal: MassEma::new(ema_period),
            ratio_sum: crate::stream::RollingSum::new(sum_period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        let range_ema = self.ema_range.append(high - low);
        let signal_ema = range_ema.and_then(|value| self.ema_signal.append(value));
        self.value = signal_ema.and_then(|signal| {
            let range = range_ema?;
            let ratio = if signal == 0.0 { 0.0 } else { range / signal };
            self.ratio_sum.append(ratio)
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

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.ema_range.reset();
        self.ema_signal.reset();
        self.ratio_sum.reset();
        self.value = None;
    }
}

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

/// Stateful Volume-price Trend, aligned to `ta.volume.VolumePriceTrendIndicator`.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `VolumePriceTrend`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct VolumePriceTrend {
    previous_close: Option<f64>,
    total: f64,
    value: Option<f64>,
}

impl VolumePriceTrend {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            previous_close: None,
            total: 0.0,
            value: None,
        }
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, close: f64, volume: f64) -> Option<f64> {
        let previous = self.previous_close.replace(close);
        self.value = previous.map(|previous| {
            if previous != 0.0 {
                self.total += volume * (close - previous) / previous;
            }
            self.total
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
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.total = 0.0;
        self.value = None;
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum VolumeIndexMode {
    Negative,
    Positive,
}

#[derive(Debug, Clone)]
pub(crate) struct VolumeIndex {
    mode: VolumeIndexMode,
    previous_close: Option<f64>,
    previous_volume: Option<f64>,
    value: f64,
}

impl VolumeIndex {
    pub(crate) fn new(mode: VolumeIndexMode) -> Self {
        Self {
            mode,
            previous_close: None,
            previous_volume: None,
            value: 1000.0,
        }
    }
    pub(crate) fn append(&mut self, close: f64, volume: f64) -> f64 {
        if let (Some(previous_close), Some(previous_volume)) =
            (self.previous_close, self.previous_volume)
        {
            let active = match self.mode {
                VolumeIndexMode::Negative => volume < previous_volume,
                VolumeIndexMode::Positive => volume > previous_volume,
            };
            if active && previous_close != 0.0 {
                self.value *= 1.0 + (close - previous_close) / previous_close;
            }
        }
        self.previous_close = Some(close);
        self.previous_volume = Some(volume);
        self.value
    }
    fn reset(&mut self) {
        self.previous_close = None;
        self.previous_volume = None;
        self.value = 1000.0;
    }
}

/// Persistent Rust state or aligned output type for `NegativeVolumeIndex`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct NegativeVolumeIndex(VolumeIndex);
/// Persistent Rust state or aligned output type for `PositiveVolumeIndex`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct PositiveVolumeIndex(VolumeIndex);
impl NegativeVolumeIndex {
    /// Create a new empty state.
    ///
    pub fn new() -> Self {
        Self(VolumeIndex::new(VolumeIndexMode::Negative))
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, close: f64, volume: f64) -> f64 {
        self.0.append(close, volume)
    }
    /// Return the latest computed result, if warm-up is complete.
    ///
    pub fn value(&self) -> f64 {
        self.0.value
    }
    /// Reset the state and clear its accumulated history.
    ///
    pub fn reset(&mut self) {
        self.0.reset();
    }
}
impl PositiveVolumeIndex {
    /// Create a new empty state.
    ///
    pub fn new() -> Self {
        Self(VolumeIndex::new(VolumeIndexMode::Positive))
    }
    /// Append one causal observation and return the latest result.
    ///
    pub fn append(&mut self, close: f64, volume: f64) -> f64 {
        self.0.append(close, volume)
    }
    /// Return the latest computed result, if warm-up is complete.
    ///
    pub fn value(&self) -> f64 {
        self.0.value
    }
    /// Reset the state and clear its accumulated history.
    ///
    pub fn reset(&mut self) {
        self.0.reset();
    }
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `McGinleyDynamic`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct McGinleyDynamic {
    length: usize,
    c: f64,
    value: Option<f64>,
}
impl McGinleyDynamic {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(length: usize, c: f64) -> TaResult<Self> {
        validate_period(length)?;
        if !(0.0 < c && c <= 1.0) {
            return Err(TaError::InvalidParameter {
                name: "c",
                value: c.to_string(),
                reason: "must be in (0, 1]",
            });
        }
        Ok(Self {
            length,
            c,
            value: None,
        })
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, close: f64) -> Option<f64> {
        self.value = Some(match self.value {
            None => close,
            Some(previous) if previous != 0.0 => {
                let mut denominator = self.c * self.length as f64 * (close / previous).powi(4);
                if denominator < 1e-10 {
                    denominator = 1e-10;
                }
                previous + (close - previous) / denominator
            }
            Some(_) => close,
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
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::*;
    use crate::stream::{CumulativeProduct, CumulativeSum, LogReturn, RollingMedian, RollingMode};

    #[test]
    fn batch_and_stream_match() {
        let input = vec![2.0, 4.0, 1.0, 8.0, 2.0];
        assert_eq!(lag(&input, 2).unwrap()[2..], [2.0, 4.0, 1.0]);
        assert_eq!(cumulative_sum(&input), vec![2.0, 6.0, 7.0, 15.0, 17.0]);
        assert_eq!(cumulative_product(&input), vec![2.0, 8.0, 8.0, 64.0, 128.0]);
        assert_eq!(cumulative_maximum(&input), vec![2.0, 4.0, 4.0, 8.0, 8.0]);
        assert_eq!(cumulative_minimum(&input), vec![2.0, 2.0, 1.0, 1.0, 1.0]);
        assert_eq!(drawdown(&input), vec![0.0, 0.0, -0.75, 0.0, -0.75]);
        let expected = log_return(&input, 2).unwrap();
        let mut state = LogReturn::new(2).unwrap();
        for (input, expected) in input.iter().zip(expected) {
            assert_eq!(
                state.append(*input).map(f64::to_bits),
                (!expected.is_nan()).then_some(expected.to_bits())
            );
        }
    }

    #[test]
    fn cumulative_states_reset() {
        let mut sum = CumulativeSum::new();
        let mut product = CumulativeProduct::new();
        assert_eq!(sum.append(2.0), 2.0);
        assert_eq!(product.append(2.0), 2.0);
        sum.reset();
        product.reset();
        assert_eq!(sum.append(3.0), 3.0);
        assert_eq!(product.append(3.0), 3.0);
    }

    #[test]
    fn rolling_statistics_match_batch_and_reset() {
        let input = vec![1.0, 4.0, 2.0, 2.0, 9.0, 4.0];
        let median = rolling_median(&input, 3).unwrap();
        let mode = rolling_mode(&input, 3).unwrap();
        assert!(median[0].is_nan() && median[1].is_nan());
        assert_eq!(&median[2..], &[2.0, 2.0, 2.0, 4.0]);
        assert!(mode[0].is_nan() && mode[1].is_nan());
        assert_eq!(&mode[2..], &[1.0, 2.0, 2.0, 2.0]);

        let mut state = RollingMedian::new(3).unwrap();
        for &value in &input {
            state.append(value);
        }
        state.reset();
        assert!(state.append(7.0).is_none());
    }

    #[test]
    fn rolling_distribution_operators_match_definitions() {
        let input = vec![1.0, 4.0, 2.0, 8.0];
        assert_eq!(rolling_quantile(&input, 3, 0.5).unwrap()[2..], [2.0, 4.0]);
        assert_eq!(
            rolling_percentile(&input, 3, 50.0).unwrap()[2..],
            [2.0, 4.0]
        );
        assert_eq!(rolling_rank(&input, 3).unwrap()[2..], [2.0 / 3.0, 1.0]);
        assert!((rolling_zscore(&input, 3).unwrap()[2] - (-0.2672612419)).abs() < 1e-9);
        assert_eq!(rolling_iqr(&input, 3).unwrap()[2], 1.5);
        assert!(
            (rolling_cov(&input, &[2.0, 8.0, 4.0, 16.0], 3).unwrap()[2] - 28.0 / 9.0).abs() < 1e-12
        );
        assert_eq!(rolling_winsorize(&input, 3, 0.0, 0.5).unwrap()[2], 2.0);
        assert_eq!(ewm_var(&input, 2).unwrap()[0], 0.0);
        assert_eq!(ewm_std(&input, 2).unwrap()[0], 0.0);
    }

    #[test]
    fn quant_family_batch_and_stream_match() {
        let close = vec![100.0, 102.0, 101.0, 105.0, 107.0, 106.0];
        let volume = vec![1000.0, 1100.0, 900.0, 1200.0, 1300.0, 950.0];

        assert_eq!(
            time_series_rank(&close, 3)
                .unwrap()
                .iter()
                .map(|&x| x.to_bits())
                .collect::<Vec<_>>(),
            rolling_rank(&close, 3)
                .unwrap()
                .iter()
                .map(|&x| x.to_bits())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            decay_linear(&close, 3)
                .unwrap()
                .iter()
                .map(|&x| x.to_bits())
                .collect::<Vec<_>>(),
            crate::stream::weighted_moving_average(&close, 3)
                .unwrap()
                .iter()
                .map(|&x| x.to_bits())
                .collect::<Vec<_>>()
        );
        assert_eq!(signed_power(&[2.0, -3.0, 0.5], 2.0), vec![4.0, -9.0, 0.25]);

        let adv_batch = average_daily_dollar_value(&close, &volume, 3).unwrap();
        let mut adv_state = AverageDailyDollarValue::new(3).unwrap();
        for ((close, volume), expected) in close.iter().zip(&volume).zip(&adv_batch) {
            assert_eq!(
                adv_state.append(*close, *volume).map(f64::to_bits),
                (!expected.is_nan()).then_some(expected.to_bits())
            );
        }

        let amihud_batch = amihud(&close, &volume, 3).unwrap();
        let mut amihud_state = Amihud::new(3).unwrap();
        for ((close, volume), expected) in close.iter().zip(&volume).zip(&amihud_batch) {
            assert_eq!(
                amihud_state.append(*close, *volume).map(f64::to_bits),
                (!expected.is_nan()).then_some(expected.to_bits())
            );
        }

        let spread_batch = roll_spread(&close, 3).unwrap();
        let mut spread_state = RollSpread::new(3).unwrap();
        for (price, expected) in close.iter().zip(&spread_batch) {
            assert_eq!(
                spread_state.append(*price).map(f64::to_bits),
                (!expected.is_nan()).then_some(expected.to_bits())
            );
        }

        let hl_batch = ornstein_uhlenbeck_half_life(&close, 3).unwrap();
        let mut hl_state = OrnsteinUhlenbeckHalfLife::new(3).unwrap();
        for (price, expected) in close.iter().zip(&hl_batch) {
            assert_eq!(
                hl_state.append(*price).map(f64::to_bits),
                (!expected.is_nan()).then_some(expected.to_bits())
            );
        }

        let cusum_batch = cumulative_sum_control_chart(&[0.5, -0.5, 2.0, -1.0], 1.0).unwrap();
        assert_eq!(cusum_batch, vec![0.0, 0.0, 1.0, 0.0]);

        assert_eq!(
            average_daily_dollar_value(&close, &volume[..5], 3),
            Err(TaError::LengthMismatch {
                expected: 6,
                got: 5
            })
        );
    }

    #[test]
    fn spread_zscore_matches_hedge_ratio_composition() {
        let x = vec![10.0, 11.0, 9.0, 12.0, 13.0, 11.5];
        let y = vec![20.0, 22.0, 18.5, 23.0, 25.0, 22.0];
        let period = 4;

        let z = spread_zscore(&x, &y, period).unwrap();
        assert!(z[..period - 1].iter().all(|&value| value.is_nan()));

        let beta = hedge_ratio(&x, &y, period).unwrap();
        for i in period - 1..x.len() {
            let window_x = &x[i + 1 - period..=i];
            let window_y = &y[i + 1 - period..=i];
            let spreads: Vec<f64> = window_x
                .iter()
                .zip(window_y)
                .map(|(&x, &y)| y - beta[i] * x)
                .collect();
            let mean = spreads.iter().sum::<f64>() / period as f64;
            let variance = spreads.iter().map(|&s| (s - mean).powi(2)).sum::<f64>() / period as f64;
            let expected = if variance > 0.0 {
                (spreads[period - 1] - mean) / variance.sqrt()
            } else {
                0.0
            };
            assert!((z[i] - expected).abs() < 1e-9, "index {i}");
        }

        let mut state = SpreadZScore::new(period).unwrap();
        let mut replayed = Vec::new();
        for (&x, &y) in x.iter().zip(&y) {
            replayed.push(state.append(x, y).unwrap_or(f64::NAN));
        }
        assert_eq!(
            replayed.iter().map(|&v| v.to_bits()).collect::<Vec<_>>(),
            z.iter().map(|&v| v.to_bits()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn frac_diff_matches_reference_weights() {
        let d = 0.5;
        let threshold = 1e-3;
        let mut weights = vec![1.0];
        let mut k = 1usize;
        loop {
            let wk = -weights[k - 1] * (d - k as f64 + 1.0) / k as f64;
            if wk.abs() < threshold {
                break;
            }
            weights.push(wk);
            k += 1;
        }
        assert!(
            weights.len() > 2,
            "truncation should retain several weights"
        );

        let input: Vec<f64> = (1..=200).map(|x| x as f64).collect();
        let output = frac_diff(&input, d, threshold).unwrap();
        let w = weights.len();
        assert!(output[..w - 1].iter().all(|&v| v.is_nan()));
        for i in w - 1..input.len() {
            let mut expected = 0.0;
            for (j, &weight) in weights.iter().enumerate() {
                expected += weight * input[i - j];
            }
            assert!((output[i] - expected).abs() < 1e-9, "index {i}");
        }

        let mut state = FracDiff::new(d, threshold).unwrap();
        let replayed: Vec<f64> = input
            .iter()
            .map(|&v| state.append(v).unwrap_or(f64::NAN))
            .collect();
        assert_eq!(
            replayed.iter().map(|&v| v.to_bits()).collect::<Vec<_>>(),
            output.iter().map(|&v| v.to_bits()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn frac_diff_rejects_bad_params() {
        assert!(FracDiff::new(0.0, 1e-5).is_err());
        assert!(FracDiff::new(0.5, 0.0).is_err());
        assert!(FracDiff::new(-1.0, 1e-5).is_err());
    }

    #[test]
    fn kalman_hedge_ratio_tracks_synthetic_beta() {
        let true_beta = 2.0;
        let x: Vec<f64> = (0..200).map(|i| i as f64 / 10.0).collect();
        let y: Vec<f64> = x.iter().map(|&x| 1.0 + true_beta * x).collect();

        let delta = 1e-4;
        let observation_variance = 1e-3;
        let beta = kalman_hedge_ratio(&x, &y, delta, observation_variance).unwrap();
        assert_eq!(beta.len(), x.len());
        assert!((beta[0] - 1.0).abs() < 1e-9);
        assert!(
            (beta[beta.len() - 1] - true_beta).abs() < 0.1,
            "final beta {}",
            beta[beta.len() - 1]
        );

        let mut state = KalmanHedgeRatio::new(delta, observation_variance).unwrap();
        let replayed: Vec<f64> = x
            .iter()
            .zip(&y)
            .map(|(&x, &y)| state.append(x, y).unwrap_or(f64::NAN))
            .collect();
        assert_eq!(
            replayed.iter().map(|&v| v.to_bits()).collect::<Vec<_>>(),
            beta.iter().map(|&v| v.to_bits()).collect::<Vec<_>>()
        );
        assert!(state.alpha().unwrap().abs() < 2.0);
        assert!(state.innovation().is_some());
        assert!(state.std().unwrap() > 0.0);

        state.reset();
        assert!(state.append(1.0, 3.0).is_some());
        assert!(state.value().unwrap() > 1.0);
    }

    #[test]
    fn kalman_hedge_ratio_rejects_bad_params() {
        assert!(KalmanHedgeRatio::new(-0.1, 1.0).is_err());
        assert!(KalmanHedgeRatio::new(0.0, 0.0).is_err());
        assert_eq!(
            kalman_hedge_ratio(&[1.0, 2.0], &[1.0], 1e-4, 1e-3),
            Err(TaError::LengthMismatch {
                expected: 2,
                got: 1
            })
        );
    }

    #[test]
    fn quant_family_rejects_bad_periods() {
        assert!(AverageDailyDollarValue::new(0).is_err());
        assert!(Amihud::new(0).is_err());
        assert!(RollSpread::new(0).is_err());
        assert!(OrnsteinUhlenbeckHalfLife::new(0).is_err());
        assert!(CumulativeSumControlChart::new(-1.0).is_err());
    }

    #[test]
    fn supertrend_batch_and_stream_match() {
        let high: Vec<f64> = (0..200)
            .map(|i| 52.0 + (i as f64 * 0.3).sin() * 5.0 + (i as f64 * 0.01).cos())
            .collect();
        let low: Vec<f64> = high.iter().map(|&h| h - 2.0).collect();
        let close: Vec<f64> = high
            .iter()
            .enumerate()
            .map(|(i, &h)| h - 1.0 + (i as f64 * 0.05).sin())
            .collect();

        let (trend, direction, long, short) = supertrend(&high, &low, &close, 7, 3.0).unwrap();
        assert!(trend[..6].iter().all(|&value| value.is_nan()));
        assert!(trend[6..].iter().all(|&value| value.is_finite()));
        assert!(direction[6..]
            .iter()
            .all(|&value| value == 1.0 || value == -1.0));

        let mut state = Supertrend::new(7, 3.0).unwrap();
        let replayed: Vec<f64> = high
            .iter()
            .zip(&low)
            .zip(&close)
            .map(|((&h, &l), &c)| state.append(h, l, c).map_or(f64::NAN, |v| v.trend))
            .collect();
        assert_eq!(
            replayed.iter().map(|&v| v.to_bits()).collect::<Vec<_>>(),
            trend.iter().map(|&v| v.to_bits()).collect::<Vec<_>>()
        );

        let mut flipped = 0;
        for pair in direction.windows(2) {
            if pair[0] != pair[1] {
                flipped += 1;
            }
        }
        assert!(
            flipped >= 2,
            "expected direction flips on the synthetic series"
        );
    }

    #[test]
    fn supertrend_rejects_bad_params() {
        assert!(Supertrend::new(0, 3.0).is_err());
        assert!(Supertrend::new(7, 0.0).is_err());
        assert!(Supertrend::new(7, -1.0).is_err());
        assert_eq!(
            supertrend(&[1.0, 2.0], &[1.0], &[1.0, 2.0], 7, 3.0),
            Err(TaError::LengthMismatch {
                expected: 2,
                got: 1
            })
        );
    }

    #[test]
    fn ichimoku_batch_and_stream_match() {
        let high: Vec<f64> = (0..200)
            .map(|i| 52.0 + (i as f64 * 0.3).sin() * 5.0)
            .collect();
        let low: Vec<f64> = high.iter().map(|&h| h - 2.0).collect();
        let close: Vec<f64> = high
            .iter()
            .enumerate()
            .map(|(i, &h)| h - 1.0 + (i as f64 * 0.02).sin())
            .collect();

        let (tenkan, kijun, span_a, span_b, chikou) =
            ichimoku(&high, &low, &close, 9, 26, 52).unwrap();
        assert!(tenkan[..8].iter().all(|&v| v.is_nan()));
        assert!(kijun[..25].iter().all(|&v| v.is_nan()));
        assert!(span_a[..25].iter().all(|&v| v.is_nan()));
        assert!(span_b[..51].iter().all(|&v| v.is_nan()));
        assert!(tenkan[8..].iter().all(|&v| v.is_finite()));
        assert!(span_b[51..].iter().all(|&v| v.is_finite()));

        // span_a = 0.5 * (tenkan + kijun); chikou = current close (causal).
        for i in 25..close.len() {
            assert!((span_a[i] - 0.5 * (tenkan[i] + kijun[i])).abs() < 1e-12);
            assert_eq!(chikou[i], close[i]);
        }

        let mut state = Ichimoku::new(9, 26, 52).unwrap();
        let replayed: Vec<f64> = high
            .iter()
            .zip(&low)
            .zip(&close)
            .map(|((&h, &l), &c)| state.append(h, l, c).span_b)
            .collect();
        assert_eq!(
            replayed.iter().map(|&v| v.to_bits()).collect::<Vec<_>>(),
            span_b.iter().map(|&v| v.to_bits()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn ichimoku_rejects_bad_params() {
        assert!(Ichimoku::new(0, 26, 52).is_err());
        assert!(Ichimoku::new(9, 0, 52).is_err());
        assert!(Ichimoku::new(9, 26, 0).is_err());
        assert_eq!(
            ichimoku(&[1.0], &[1.0], &[1.0, 2.0], 9, 26, 52),
            Err(TaError::LengthMismatch {
                expected: 1,
                got: 1
            })
        );
    }

    #[test]
    fn squeeze_batch_and_stream_match() {
        let high: Vec<f64> = (0..240)
            .map(|i| 52.0 + (i as f64 * 0.31).sin() * 6.0 + (i as f64 * 0.015).cos())
            .collect();
        let low: Vec<f64> = high.iter().map(|&h| h - 3.0).collect();
        let close: Vec<f64> = high
            .iter()
            .enumerate()
            .map(|(i, &h)| h - 1.5 + (i as f64 * 0.07).sin())
            .collect();

        let (squeeze, on, off, no) = squeeze(&high, &low, &close, 20, 2.0, 20, 1.5, 12, 6).unwrap();
        assert!(squeeze[..16].iter().all(|&v| v.is_nan()));
        assert!(squeeze[17..].iter().all(|&v| v.is_finite()));
        assert!(on[..19].iter().all(|&v| v == 0.0));
        assert!(off[..19].iter().all(|&v| v == 0.0));
        assert!(no[..19].iter().all(|&v| v == 1.0));
        assert!(on[19..].iter().all(|&v| v == 0.0 || v == 1.0));
        assert!(off[19..].iter().all(|&v| v == 0.0 || v == 1.0));
        assert!(no[19..].iter().all(|&v| v == 0.0 || v == 1.0));
        for i in 19..close.len() {
            assert_eq!(on[i] + off[i] + no[i], 1.0);
        }

        let mut state = Squeeze::new(20, 2.0, 20, 1.5, 12, 6).unwrap();
        let replayed: Vec<f64> = high
            .iter()
            .zip(&low)
            .zip(&close)
            .map(|((&h, &l), &c)| state.append(h, l, c).squeeze)
            .collect();
        assert_eq!(
            replayed.iter().map(|&v| v.to_bits()).collect::<Vec<_>>(),
            squeeze.iter().map(|&v| v.to_bits()).collect::<Vec<_>>()
        );

        let mut state = Squeeze::new(20, 2.0, 20, 1.5, 12, 6).unwrap();
        let replayed_on: Vec<f64> = high
            .iter()
            .zip(&low)
            .zip(&close)
            .map(|((&h, &l), &c)| state.append(h, l, c).on)
            .collect();
        assert_eq!(
            replayed_on.iter().map(|&v| v.to_bits()).collect::<Vec<_>>(),
            on.iter().map(|&v| v.to_bits()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn squeeze_rejects_bad_params() {
        assert!(Squeeze::new(0, 2.0, 20, 1.5, 12, 6).is_err());
        assert!(Squeeze::new(20, 2.0, 0, 1.5, 12, 6).is_err());
        assert!(Squeeze::new(20, 2.0, 20, 0.0, 12, 6).is_err());
        assert!(Squeeze::new(20, 0.0, 20, 1.5, 12, 6).is_err());
        assert_eq!(
            squeeze(&[1.0, 2.0], &[1.0], &[1.0, 2.0], 20, 2.0, 20, 1.5, 12, 6),
            Err(TaError::LengthMismatch {
                expected: 2,
                got: 1
            })
        );
    }

    #[test]
    fn squeeze_pro_batch_and_stream_match() {
        let high: Vec<f64> = (0..240)
            .map(|i| 52.0 + (i as f64 * 0.31).sin() * 6.0 + (i as f64 * 0.015).cos())
            .collect();
        let low: Vec<f64> = high.iter().map(|&h| h - 3.0).collect();
        let close: Vec<f64> = high
            .iter()
            .enumerate()
            .map(|(i, &h)| h - 1.5 + (i as f64 * 0.07).sin())
            .collect();

        let (sq, on_wide, on_normal, on_narrow, off, no) =
            squeeze_pro(&high, &low, &close, 20, 2.0, 20, 2.0, 1.5, 1.0, 12, 6).unwrap();
        assert!(sq[..16].iter().all(|&v| v.is_nan()));
        assert!(sq[17..].iter().all(|&v| v.is_finite()));
        for column in [&on_wide, &on_normal, &on_narrow, &off, &no] {
            assert!(column[19..].iter().all(|&v| v == 0.0 || v == 1.0));
        }

        let mut state = SqueezePro::new(20, 2.0, 20, 2.0, 1.5, 1.0, 12, 6).unwrap();
        let replayed: Vec<f64> = high
            .iter()
            .zip(&low)
            .zip(&close)
            .map(|((&h, &l), &c)| state.append(h, l, c).on_narrow)
            .collect();
        assert_eq!(
            replayed.iter().map(|&v| v.to_bits()).collect::<Vec<_>>(),
            on_narrow.iter().map(|&v| v.to_bits()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn squeeze_pro_rejects_bad_params() {
        assert!(SqueezePro::new(0, 2.0, 20, 2.0, 1.5, 1.0, 12, 6).is_err());
        assert!(SqueezePro::new(20, 2.0, 20, 1.5, 1.5, 1.0, 12, 6).is_err());
        assert!(SqueezePro::new(20, 2.0, 20, 2.0, 1.5, 2.0, 12, 6).is_err());
        assert!(SqueezePro::new(20, 2.0, 20, 2.0, 1.5, 0.0, 12, 6).is_err());
        assert_eq!(
            squeeze_pro(
                &[1.0, 2.0],
                &[1.0],
                &[1.0, 2.0],
                20,
                2.0,
                20,
                2.0,
                1.5,
                1.0,
                12,
                6
            ),
            Err(TaError::LengthMismatch {
                expected: 2,
                got: 1
            })
        );
    }

    #[test]
    fn stc_batch_and_stream_match() {
        let close: Vec<f64> = (0..300)
            .map(|i| 100.0 + (i as f64 * 0.07).sin() * 8.0 + (i as f64 * 0.013) * 2.0)
            .collect();

        let (stc, macd, stoch) = schaff_trend_cycle(&close, 10, 12, 26, 0.5).unwrap();
        assert_eq!(stc[0], 0.0);
        assert_eq!(stoch[0], 0.0);
        assert!(macd[..24].iter().all(|&v| v.is_nan()));
        assert!(macd[25..].iter().all(|&v| v.is_finite()));
        assert!(stc
            .iter()
            .all(|&v| v.is_finite() && (0.0..=100.0).contains(&v)));
        assert!(stoch
            .iter()
            .all(|&v| v.is_finite() && (0.0..=100.0).contains(&v)));

        let mut state = SchaffTrendCycle::new(10, 12, 26, 0.5).unwrap();
        let replayed: Vec<f64> = close.iter().map(|&c| state.append(c).stc).collect();
        assert_eq!(
            replayed.iter().map(|&v| v.to_bits()).collect::<Vec<_>>(),
            stc.iter().map(|&v| v.to_bits()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn stc_swaps_fast_slow_and_rejects_bad_params() {
        let close: Vec<f64> = (0..200).map(|i| 100.0 + (i as f64 * 0.03).cos()).collect();
        let (a, _, _) = schaff_trend_cycle(&close, 10, 12, 26, 0.5).unwrap();
        let (b, _, _) = schaff_trend_cycle(&close, 10, 26, 12, 0.5).unwrap();
        assert_eq!(a, b);

        assert!(SchaffTrendCycle::new(0, 12, 26, 0.5).is_err());
        assert!(SchaffTrendCycle::new(10, 0, 26, 0.5).is_err());
        assert!(SchaffTrendCycle::new(10, 12, 26, 0.0).is_err());
    }

    #[test]
    fn vortex_batch_and_stream_match() {
        let high: Vec<f64> = (0..240)
            .map(|i| 52.0 + (i as f64 * 0.31).sin() * 5.0)
            .collect();
        let low: Vec<f64> = high.iter().map(|&h| h - 2.5).collect();
        let close: Vec<f64> = high
            .iter()
            .enumerate()
            .map(|(i, &h)| h - 1.2 + (i as f64 * 0.05).sin())
            .collect();

        let (vp, vn) = vortex(&high, &low, &close, 14).unwrap();
        assert!(vp[..13].iter().all(|&v| v.is_nan()));
        assert!(vn[..13].iter().all(|&v| v.is_nan()));
        assert!(vp[14..].iter().all(|&v| v.is_finite() && v >= 0.0));
        assert!(vn[14..].iter().all(|&v| v.is_finite() && v >= 0.0));

        let mut state = Vortex::new(14).unwrap();
        let replayed: Vec<f64> = high
            .iter()
            .zip(&low)
            .zip(&close)
            .map(|((&h, &l), &c)| state.append(h, l, c).vp)
            .collect();
        assert_eq!(
            replayed.iter().map(|&v| v.to_bits()).collect::<Vec<_>>(),
            vp.iter().map(|&v| v.to_bits()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn vortex_rejects_bad_params() {
        assert!(Vortex::new(0).is_err());
        assert_eq!(
            vortex(&[1.0, 2.0], &[1.0], &[1.0, 2.0], 14),
            Err(TaError::LengthMismatch {
                expected: 2,
                got: 1
            })
        );
    }

    #[test]
    fn kst_batch_and_stream_match() {
        let close: Vec<f64> = (0..400)
            .map(|i| 100.0 + (i as f64 * 0.05).sin() * 6.0 + i as f64 * 0.01)
            .collect();

        let (kst, signal) = know_sure_thing(&close, 10, 15, 20, 30, 10, 10, 10, 15, 9).unwrap();
        assert!(kst[..43].iter().all(|&v| v.is_nan()));
        assert!(signal[..43].iter().all(|&v| v.is_nan()));
        assert!(kst[44..].iter().all(|&v| v.is_finite()));
        assert!(signal[52..].iter().all(|&v| v.is_finite()));

        let mut state = KnowSureThing::new(10, 15, 20, 30, 10, 10, 10, 15, 9).unwrap();
        let replayed: Vec<f64> = close.iter().map(|&c| state.append(c).kst).collect();
        assert_eq!(
            replayed.iter().map(|&v| v.to_bits()).collect::<Vec<_>>(),
            kst.iter().map(|&v| v.to_bits()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn kst_rejects_bad_params() {
        assert!(KnowSureThing::new(0, 15, 20, 30, 10, 10, 10, 15, 9).is_err());
        assert!(KnowSureThing::new(10, 15, 20, 30, 10, 10, 10, 15, 0).is_err());
    }

    #[test]
    fn mass_index_batch_and_stream_match() {
        let high: Vec<f64> = (0..200)
            .map(|i| 100.0 + i as f64 * 0.2 + (i as f64 * 0.13).sin())
            .collect();
        let low: Vec<f64> = high.iter().map(|value| value - 2.0).collect();
        let batch = mass_index(&high, &low, 9, 25).unwrap();
        let mut state = MassIndex::new(9, 25).unwrap();
        let replayed: Vec<f64> = high
            .iter()
            .zip(&low)
            .map(|(&high, &low)| state.append(high, low).unwrap_or(f64::NAN))
            .collect();
        assert_eq!(
            batch.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            replayed.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );
        assert!(batch[..40].iter().all(|value| value.is_nan()));
        assert!(batch[40..].iter().all(|value| value.is_finite()));
    }

    #[test]
    fn dpo_batch_and_stream_match() {
        let input: Vec<f64> = (0..100)
            .map(|i| i as f64 + (i as f64 * 0.2).sin())
            .collect();
        let batch = detrended_price_oscillator(&input, 20).unwrap();
        let mut state = DetrendedPriceOscillator::new(20).unwrap();
        let replayed: Vec<f64> = input
            .iter()
            .map(|&value| state.append(value).unwrap_or(f64::NAN))
            .collect();
        assert_eq!(
            batch.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            replayed.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );
        assert!(batch[..30].iter().all(|value| value.is_nan()));
        assert!(batch[30..].iter().all(|value| value.is_finite()));
    }

    #[test]
    fn cmf_batch_and_stream_match() {
        let close: Vec<f64> = (0..100).map(|i| 100.0 + i as f64 * 0.1).collect();
        let high: Vec<f64> = close.iter().map(|value| value + 1.0).collect();
        let low: Vec<f64> = close.iter().map(|value| value - 1.0).collect();
        let volume: Vec<f64> = (1..=100).map(|value| value as f64).collect();
        let batch = chaikin_money_flow(&high, &low, &close, &volume, 20).unwrap();
        let mut state = ChaikinMoneyFlow::new(20).unwrap();
        let replayed: Vec<f64> = high
            .iter()
            .zip(&low)
            .zip(&close)
            .zip(&volume)
            .map(|(((&h, &l), &c), &v)| state.append(h, l, c, v).unwrap_or(f64::NAN))
            .collect();
        assert_eq!(
            batch.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            replayed.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );
        assert!(batch[..19].iter().all(|value| value.is_nan()));
        assert!(batch[19..].iter().all(|value| value.is_finite()));
    }

    #[test]
    fn vpt_batch_and_stream_match() {
        let close: Vec<f64> = (1..=100).map(|value| value as f64).collect();
        let volume: Vec<f64> = (1..=100).map(|value| value as f64).collect();
        let batch = volume_price_trend(&close, &volume).unwrap();
        let mut state = VolumePriceTrend::new();
        let replayed: Vec<f64> = close
            .iter()
            .zip(&volume)
            .map(|(&close, &volume)| state.append(close, volume).unwrap_or(f64::NAN))
            .collect();
        assert_eq!(
            batch.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            replayed.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );
        assert!(batch[0].is_nan());
        assert!(batch[1..].iter().all(|value| value.is_finite()));
    }
}
