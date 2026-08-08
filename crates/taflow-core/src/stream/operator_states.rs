use std::collections::VecDeque;

use super::{
    AverageTrueRange, CumulativeMaximum, ExponentialMovingAverage, MonotonicMax, MonotonicMin,
    RollingMedian, RollingMode, RollingStandardDeviation, SimpleMovingAverage, StreamingIndicator,
    TrueRange, Window,
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
            // Contiguous two-slice scans with fused accumulators: each
            // accumulator adds the same terms in the same order as the
            // original per-quantity passes, so results are bit-identical.
            let (front, back) = self.values.as_slices();
            let mut sum_input = 0.0;
            let mut sum_benchmark = 0.0;
            for &(input, benchmark) in front {
                sum_input += input;
                sum_benchmark += benchmark;
            }
            for &(input, benchmark) in back {
                sum_input += input;
                sum_benchmark += benchmark;
            }
            let mean_input = sum_input / n;
            let mean_benchmark = sum_benchmark / n;
            let mut covariance = 0.0;
            let mut variance = 0.0;
            for &(input, benchmark) in front {
                let delta_benchmark = benchmark - mean_benchmark;
                covariance += (input - mean_input) * delta_benchmark;
                variance += delta_benchmark * delta_benchmark;
            }
            for &(input, benchmark) in back {
                let delta_benchmark = benchmark - mean_benchmark;
                covariance += (input - mean_input) * delta_benchmark;
                variance += delta_benchmark * delta_benchmark;
            }
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
    values: ContiguousWindow,
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
            values: ContiguousWindow::new(period),
            period,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    /// The variance pass needs the window mean, so the two passes cannot be
    /// collapsed into sliding sums without changing the summation order (and
    /// therefore the low bits). Both passes now walk one contiguous ring
    /// slice, so the second pass reads cache-hot memory.
    pub fn append(&mut self, input: f64, benchmark: f64) -> Option<f64> {
        self.values.push(input - benchmark);
        self.value = self.values.is_full().then(|| {
            let window = self.values.window();
            let n = self.period as f64;
            let mean = window.iter().sum::<f64>() / n;
            let variance = window
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
    /// `ln(period)`, invariant for the lifetime of the state; computing it
    /// once at construction removes one `ln` call per bar.
    log_period: f64,
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
            log_period: (period as f64).ln(),
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
        let log_period = self.log_period;
        self.value = (self.values.len() == self.period).then(|| {
            let n = self.period as f64;
            // Contiguous two-slice scans; the R/S walk and the squared-deviation
            // sum are fused into one pass with independent accumulators, each
            // adding the same terms in the same order as before (bit-identical).
            let (front, back) = self.values.as_slices();
            let mut sum = 0.0;
            for &value in front {
                sum += value;
            }
            for &value in back {
                sum += value;
            }
            let mean = sum / n;
            let mut cumulative = 0.0;
            let mut minimum = f64::INFINITY;
            let mut maximum = f64::NEG_INFINITY;
            let mut squared = 0.0;
            // Plain comparisons instead of `f64::min`/`f64::max`: the
            // accumulators start at `±INFINITY` and `f64::min`/`max` never
            // return NaN when one operand is non-NaN, so they can never hold
            // NaN. For a non-NaN accumulator the two forms agree on every
            // input including NaN (`NaN < minimum` is false, and
            // `f64::min(minimum, NaN) == minimum`), so this is bit-identical
            // while dropping the NaN fix-up from the dependency chain.
            for &value in front {
                let deviation = value - mean;
                cumulative += deviation;
                if cumulative < minimum {
                    minimum = cumulative;
                }
                if cumulative > maximum {
                    maximum = cumulative;
                }
                squared += deviation * deviation;
            }
            for &value in back {
                let deviation = value - mean;
                cumulative += deviation;
                if cumulative < minimum {
                    minimum = cumulative;
                }
                if cumulative > maximum {
                    maximum = cumulative;
                }
                squared += deviation * deviation;
            }
            let standard_deviation = (squared / n).sqrt();
            let rescaled_range = (maximum - minimum) / standard_deviation;
            if rescaled_range > 0.0 {
                // `log_period` is `(period as f64).ln()` computed once at
                // construction — the same value `n.ln()` produced per bar.
                (rescaled_range.ln() / log_period).clamp(0.0, 1.0)
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
    ring: Box<[f64]>,
    head: usize,
    len: usize,
    counts: std::collections::HashMap<u64, u32>,
    seen: std::collections::HashSet<u64>,
    period: usize,
    value: Option<f64>,
}

impl RollingEntropy {
    /// Map key with `f64` equality semantics: `-0.0` and `+0.0` share a bin.
    /// NaNs are never inserted (NaN equals nothing, so its count is 0).
    #[inline]
    fn count_key(value: f64) -> u64 {
        if value == 0.0 {
            0.0f64.to_bits()
        } else {
            value.to_bits()
        }
    }

    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            ring: vec![0.0; period].into_boxed_slice(),
            head: 0,
            len: 0,
            counts: std::collections::HashMap::with_capacity(period),
            seen: std::collections::HashSet::with_capacity(period),
            period,
            value: None,
        })
    }

    /// Shannon entropy of exact-value frequencies in the rolling window.
    ///
    /// The exact-value counts are maintained incrementally (integer work on
    /// the two touched bins per bar); the entropy sum itself is recomputed
    /// per bar in the original iteration order (first occurrence in window
    /// order) so the floating-point result stays bit-identical to the
    /// previous full-rescan implementation.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.len == self.period {
            let evicted = self.ring[self.head];
            if !evicted.is_nan() {
                let key = Self::count_key(evicted);
                let count = self.counts.get_mut(&key).expect("evicted value counted");
                *count -= 1;
                if *count == 0 {
                    self.counts.remove(&key);
                }
            }
        } else {
            self.len += 1;
        }
        self.ring[self.head] = input;
        self.head += 1;
        if self.head == self.period {
            self.head = 0;
        }
        if !input.is_nan() {
            *self.counts.entry(Self::count_key(input)).or_insert(0) += 1;
        }
        let value = if self.len == self.period {
            let n = self.period as f64;
            let mut entropy = 0.0;
            self.seen.clear();
            // `head` now points at the oldest value in window order.
            let start = self.head;
            for i in 0..self.period {
                let mut idx = start + i;
                if idx >= self.period {
                    idx -= self.period;
                }
                let candidate = self.ring[idx];
                let probability = if candidate.is_nan() {
                    // NaN never equals anything: count 0, exactly as the
                    // full rescan produced (0.0 * ln(0.0) => NaN result).
                    0.0
                } else {
                    let key = Self::count_key(candidate);
                    if !self.seen.insert(key) {
                        continue;
                    }
                    *self.counts.get(&key).expect("window value counted") as f64 / n
                };
                entropy -= probability * probability.ln();
            }
            Some(entropy)
        } else {
            None
        };
        self.value = value;
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
        self.head = 0;
        self.len = 0;
        self.counts.clear();
        self.seen.clear();
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
    /// Reusable contiguous scratch copy of the window; fixed-size, so the
    /// per-bar refresh is two `copy_from_slice` memcpys with no allocation,
    /// capacity check or length bookkeeping.
    scratch: Box<[f64]>,
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
            scratch: vec![0.0; period].into_boxed_slice(),
            value: None,
        })
    }

    /// Lag-one Pearson autocorrelation over the rolling window.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.period {
            self.values.pop_front();
        }
        self.values.push_back(input);
        if self.values.len() < self.period {
            self.value = None;
            return None;
        }
        // Copy the window into a contiguous scratch buffer (pure memcpy, the
        // buffer is preallocated) so the lagged scans below index plain
        // slices. Every accumulator adds the same terms in the same order as
        // the original take/skip iterator passes, so results are bit-identical.
        let (front, back) = self.values.as_slices();
        self.scratch[..front.len()].copy_from_slice(front);
        self.scratch[front.len()..].copy_from_slice(back);
        let window = &self.scratch[..];
        let period = self.period;
        let left = &window[..period - 1];
        let right = &window[1..];
        let left_n = (period - 1) as f64;
        let mut left_sum = 0.0;
        for &value in left {
            left_sum += value;
        }
        let mut right_sum = 0.0;
        for &value in right {
            right_sum += value;
        }
        let left_mean = left_sum / left_n;
        let right_mean = right_sum / left_n;
        let mut left_variance = 0.0;
        let mut right_variance = 0.0;
        let mut covariance = 0.0;
        for index in 0..period - 1 {
            let left_delta = left[index] - left_mean;
            let right_delta = right[index] - right_mean;
            left_variance += left_delta * left_delta;
            right_variance += right_delta * right_delta;
            covariance += left_delta * right_delta;
        }
        let result = if left_variance == 0.0 || right_variance == 0.0 {
            0.0
        } else {
            covariance / (left_variance * right_variance).sqrt()
        };
        self.value = Some(result);
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
            // Contiguous two-slice scans with fused accumulators: each
            // accumulator adds the same terms in the same order as the
            // original per-quantity passes, so results are bit-identical.
            let (front, back) = self.values.as_slices();
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;
            for &(x, y) in front {
                sum_x += x;
                sum_y += y;
            }
            for &(x, y) in back {
                sum_x += x;
                sum_y += y;
            }
            let mean_x = sum_x / n;
            let mean_y = sum_y / n;
            let mut covariance = 0.0;
            let mut variance = 0.0;
            for &(x, y) in front {
                let delta_x = x - mean_x;
                covariance += delta_x * (y - mean_y);
                variance += delta_x * delta_x;
            }
            for &(x, y) in back {
                let delta_x = x - mean_x;
                covariance += delta_x * (y - mean_y);
                variance += delta_x * delta_x;
            }
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
                // Stack copy instead of a per-event heap allocation.
                let items = [
                    self.swings[0],
                    self.swings[1],
                    self.swings[2],
                    self.swings[3],
                ];
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
                    if let (Some(structure_high), Some((low_level, low_volume, false))) =
                        (self.structure_high, self.internal_low)
                    {
                        if internal_swing.level > structure_high {
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
                    if let (Some(structure_low), Some((high_level, high_volume, false))) =
                        (self.structure_low, self.internal_high)
                    {
                        if internal_swing.level < structure_low {
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
    /// Insertion order across both lists of one side; reproduces the original
    /// single-vector scan order for nearest-pool tie-breaks and sweep output.
    seq: u64,
}

/// Causal liquidity-pool clustering with sweep detection. SwingHighLow highs and
/// lows are clustered into pools when they fall within a `range_percent`
/// price tolerance; a pool emits a signal once a second swing confirms it.
/// A pool is swept and removed when price trades beyond its level.
///
/// Pools are split per side into `*_candidates` (seen once, never sweepable)
/// and `*_confirmed` (seen twice or more, kept sorted by insertion `seq`).
/// The per-bar sweep pass therefore only scans confirmed pools instead of the
/// unbounded historical candidate list; outputs are identical to the previous
/// single-vector implementation.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `Liquidity`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct Liquidity {
    swing: SwingHighLow,
    high_candidates: Vec<LiquidityPool>,
    high_confirmed: Vec<LiquidityPool>,
    low_candidates: Vec<LiquidityPool>,
    low_confirmed: Vec<LiquidityPool>,
    next_seq: u64,
    range_percent: f64,
    value: Option<LiquidityValue>,
}

/// Location of the nearest matching pool: candidate list or confirmed list.
#[derive(Debug, Clone, Copy)]
enum PoolSlot {
    Candidate(usize),
    Confirmed(usize),
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
            high_candidates: Vec::new(),
            high_confirmed: Vec::new(),
            low_candidates: Vec::new(),
            low_confirmed: Vec::new(),
            next_seq: 0,
            range_percent,
            value: None,
        })
    }

    /// Nearest pool by absolute distance; ties resolve to the earliest
    /// inserted pool (`seq`), matching the original first-match-wins scan
    /// over a single insertion-ordered vector.
    fn nearest_pool(
        candidates: &[LiquidityPool],
        confirmed: &[LiquidityPool],
        level: f64,
        range_percent: f64,
    ) -> Option<PoolSlot> {
        let mut best: Option<(PoolSlot, f64, u64)> = None;
        let mut consider = |slot: PoolSlot, pool: &LiquidityPool| {
            let distance = (pool.level - level).abs();
            if distance <= range_percent * pool.level
                && best.map_or(true, |(_, best_distance, best_seq)| {
                    distance < best_distance || (distance == best_distance && pool.seq < best_seq)
                })
            {
                best = Some((slot, distance, pool.seq));
            }
        };
        for (index, pool) in candidates.iter().enumerate() {
            consider(PoolSlot::Candidate(index), pool);
        }
        for (index, pool) in confirmed.iter().enumerate() {
            consider(PoolSlot::Confirmed(index), pool);
        }
        best.map(|(slot, _, _)| slot)
    }

    /// Merge a swing into the pools of one side; returns the emitted level if
    /// the pool is (or becomes) confirmed. `merge_level` is `f64::max` for
    /// highs and `f64::min` for lows.
    fn merge_swing(
        candidates: &mut Vec<LiquidityPool>,
        confirmed: &mut Vec<LiquidityPool>,
        next_seq: &mut u64,
        swing_level: f64,
        range_percent: f64,
        merge_level: fn(f64, f64) -> f64,
    ) -> Option<f64> {
        match Self::nearest_pool(candidates, confirmed, swing_level, range_percent) {
            Some(PoolSlot::Confirmed(index)) => {
                let pool = &mut confirmed[index];
                pool.level = merge_level(pool.level, swing_level);
                Some(pool.level)
            }
            Some(PoolSlot::Candidate(index)) => {
                // Second touch: promote to confirmed. `swap_remove` is fine
                // because candidate order is irrelevant (ties use `seq`);
                // confirmed stays sorted by `seq` to preserve sweep order.
                let mut pool = candidates.swap_remove(index);
                pool.level = merge_level(pool.level, swing_level);
                let position = confirmed.partition_point(|entry| entry.seq < pool.seq);
                confirmed.insert(position, pool);
                Some(confirmed[position].level)
            }
            None => {
                candidates.push(LiquidityPool {
                    level: swing_level,
                    seq: *next_seq,
                });
                *next_seq += 1;
                None
            }
        }
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
                if let Some(emitted) = Self::merge_swing(
                    &mut self.high_candidates,
                    &mut self.high_confirmed,
                    &mut self.next_seq,
                    swing.level,
                    self.range_percent,
                    f64::max,
                ) {
                    liquidity = 1.0;
                    level = emitted;
                }
            } else if swing.signal < 0.0 {
                if let Some(emitted) = Self::merge_swing(
                    &mut self.low_candidates,
                    &mut self.low_confirmed,
                    &mut self.next_seq,
                    swing.level,
                    self.range_percent,
                    f64::min,
                ) {
                    liquidity = -1.0;
                    level = emitted;
                }
            }
        }

        // Sweep pass over confirmed pools only (candidates can never satisfy
        // the original `count >= 2` predicate). Confirmed pools are kept in
        // insertion order, so the last swept pool sets the outputs exactly as
        // the original combined retain did.
        self.high_confirmed.retain(|pool| {
            let swept_pool = high >= pool.level;
            if swept_pool {
                swept = 1.0;
                level = pool.level;
            }
            !swept_pool
        });
        self.low_confirmed.retain(|pool| {
            let swept_pool = low <= pool.level;
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
        self.high_candidates.clear();
        self.high_confirmed.clear();
        self.low_candidates.clear();
        self.low_confirmed.clear();
        self.next_seq = 0;
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
    /// One shared ring of `[overnight, open_close, rs]` triples: the three
    /// variance components are pushed and evicted together (M4), so they need
    /// one window, one length check and one eviction index instead of three.
    window: Box<[[f64; 3]]>,
    head: usize,
    len: usize,
    sums: [f64; 3],
    means: Option<[f64; 3]>,
    timeperiod: usize,
    /// `0.34 / (1.34 + (n + 1) / (n - 1))`, constant for the state's lifetime.
    k: f64,
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
        let n = timeperiod as f64;
        Ok(Self {
            window: vec![[0.0; 3]; timeperiod].into_boxed_slice(),
            head: 0,
            len: 0,
            sums: [0.0; 3],
            means: None,
            timeperiod,
            k: 0.34 / (1.34 + (n + 1.0) / (n - 1.0)),
            previous_close: None,
            value: None,
        })
    }

    /// Pushes one triple through the shared ring, keeping the per-component
    /// arithmetic order of the three former `RollingMean` states
    /// (`sum -= evicted`, then `sum += input`, then `sum / period`).
    #[inline]
    fn push(&mut self, sample: [f64; 3]) {
        let capacity = self.window.len();
        if self.len == capacity {
            let evicted = self.window[self.head];
            for component in 0..3 {
                self.sums[component] -= evicted[component];
            }
            self.window[self.head] = sample;
            self.head += 1;
            if self.head == capacity {
                self.head = 0;
            }
        } else {
            let mut tail = self.head + self.len;
            if tail >= capacity {
                tail -= capacity;
            }
            self.window[tail] = sample;
            self.len += 1;
        }
        for component in 0..3 {
            self.sums[component] += sample[component];
        }
        self.means = (self.len == capacity).then(|| {
            [
                self.sums[0] / self.timeperiod as f64,
                self.sums[1] / self.timeperiod as f64,
                self.sums[2] / self.timeperiod as f64,
            ]
        });
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
                    self.push([overnight, open_close, rs]);
                }
            }
        }
        let k = self.k;
        self.value = self
            .means
            .map(|[on, oc, rs]| (on + k * oc + (1.0 - k) * rs).max(0.0).sqrt());
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
        self.head = 0;
        self.len = 0;
        self.sums = [0.0; 3];
        self.means = None;
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
    values: VecDeque<(f64, f64)>,
    timeperiod: usize,
    /// Sample variance of the `y` window, computed in the covariance pass so
    /// consumers (`OrnsteinUhlenbeckHalfLife`) do not rescan the window.
    var_y: f64,
    value: Option<f64>,
}

impl RollingPairMoments {
    fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self {
            values: VecDeque::with_capacity(timeperiod),
            timeperiod,
            var_y: f64::NAN,
            value: None,
        })
    }

    fn append(&mut self, x: f64, y: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod {
            self.values.pop_front();
        }
        self.values.push_back((x, y));
        self.value = if self.values.len() == self.timeperiod {
            let n = self.timeperiod as f64;
            // Contiguous two-slice scans with fused accumulators: each
            // accumulator adds the same terms in the same order as the
            // original per-quantity passes, so results are bit-identical.
            let (front, back) = self.values.as_slices();
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;
            for &(x, y) in front {
                sum_x += x;
                sum_y += y;
            }
            for &(x, y) in back {
                sum_x += x;
                sum_y += y;
            }
            let mean_x = sum_x / n;
            let mean_y = sum_y / n;
            let mut cov = 0.0;
            let mut squared_y = 0.0;
            for &(x, y) in front {
                let delta_y = y - mean_y;
                cov += (x - mean_x) * delta_y;
                squared_y += delta_y * delta_y;
            }
            for &(x, y) in back {
                let delta_y = y - mean_y;
                cov += (x - mean_x) * delta_y;
                squared_y += delta_y * delta_y;
            }
            self.var_y = squared_y / (n - 1.0);
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
        self.values.clear();
        self.var_y = f64::NAN;
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
            // `var_y` is computed inside `RollingPairMoments::append` from the
            // same window with the same summation order as the scans this
            // replaced, so the result is bit-identical.
            let var_y = self.moments.var_y;
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
            // Contiguous two-slice scans with fused accumulators: each
            // accumulator adds the same terms in the same order as the
            // original per-quantity passes, so results are bit-identical.
            let (front, back) = self.values.as_slices();
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;
            for &(x, y) in front {
                sum_x += x;
                sum_y += y;
            }
            for &(x, y) in back {
                sum_x += x;
                sum_y += y;
            }
            let mean_x = sum_x / n;
            let mean_y = sum_y / n;
            let mut covariance = 0.0;
            let mut variance = 0.0;
            for &(x, y) in front {
                let delta_x = x - mean_x;
                covariance += delta_x * (y - mean_y);
                variance += delta_x * delta_x;
            }
            for &(x, y) in back {
                let delta_x = x - mean_x;
                covariance += delta_x * (y - mean_y);
                variance += delta_x * delta_x;
            }
            let beta = if variance > 0.0 {
                covariance / variance
            } else {
                0.0
            };
            let spread = y - beta * x;
            let mut spread_sum = 0.0;
            for &(x, y) in front {
                spread_sum += y - beta * x;
            }
            for &(x, y) in back {
                spread_sum += y - beta * x;
            }
            let mean_spread = spread_sum / n;
            let mut spread_squared = 0.0;
            for &(x, y) in front {
                let delta = y - beta * x - mean_spread;
                spread_squared += delta * delta;
            }
            for &(x, y) in back {
                let delta = y - beta * x - mean_spread;
                spread_squared += delta * delta;
            }
            let std_spread = (spread_squared / n).sqrt();
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
    weights: Box<[f64]>,
    /// Double-written ring of `2 * weights.len()` slots: each input is
    /// written at `pos` and `pos + width`, so the current window is always
    /// the contiguous slice `buffer[pos..pos + width]` (oldest to newest).
    buffer: Box<[f64]>,
    pos: usize,
    len: usize,
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
            weights: weights.into_boxed_slice(),
            buffer: vec![0.0; 2 * capacity].into_boxed_slice(),
            pos: 0,
            len: 0,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    ///
    /// The dot product accumulates newest-first (`weights[0] * latest`, then
    /// older bars), in the same order and with the same `acc += w * x`
    /// operation as the previous `VecDeque` implementation, so results are
    /// bit-identical; only the storage changed to a contiguous slice.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        let width = self.weights.len();
        self.buffer[self.pos] = input;
        self.buffer[self.pos + width] = input;
        self.pos += 1;
        if self.pos == width {
            self.pos = 0;
        }
        if self.len < width {
            self.len += 1;
        }
        self.value = if self.len == width {
            let window = &self.buffer[self.pos..self.pos + width];
            let mut acc = 0.0;
            for (&w, &x) in self.weights.iter().zip(window.iter().rev()) {
                acc += w * x;
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
        self.pos = 0;
        self.len = 0;
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
    /// One shared max/min staircase per side serving all three windows (M1),
    /// instead of three `RollingMidprice` states with six deques between them.
    highs: MultiPeriodStaircase,
    lows: MultiPeriodStaircase,
    tenkan_period: usize,
    kijun_period: usize,
    senkou_period: usize,
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
        let longest = tenkan.max(kijun).max(senkou);
        Ok(Self {
            highs: MultiPeriodStaircase::new(longest, true),
            lows: MultiPeriodStaircase::new(longest, false),
            tenkan_period: tenkan,
            kijun_period: kijun,
            senkou_period: senkou,
            value: None,
        })
    }

    /// Midpoint of the rolling high max and low min over `period` bars.
    #[inline]
    fn midprice(&self, period: usize) -> f64 {
        match (self.highs.extremum(period), self.lows.extremum(period)) {
            (Some(high), Some(low)) => (high + low) * 0.5,
            _ => f64::NAN,
        }
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    ///
    /// The three extrema pairs share two scans: each bar is pushed once per
    /// side and each window's midprice is read off the shared staircase. The
    /// extrema themselves are comparison-only, so tenkan/kijun/span_b are the
    /// same numbers the three separate `RollingMidprice` states produced.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> IchimokuValue {
        self.highs.push(high);
        self.lows.push(low);
        let tenkan = self.midprice(self.tenkan_period);
        let kijun = self.midprice(self.kijun_period);
        let span_b = self.midprice(self.senkou_period);
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
        self.highs.reset();
        self.lows.reset();
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
    /// `None` when `kc_length == bb_length`: the Keltner basis is then the
    /// same SMA of close as the Bollinger midline, so it is read from
    /// `bb_mid` rather than maintained a second time (M4).
    kc_basis: Option<SimpleMovingAverage>,
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
            kc_basis: (kc_length != bb_length)
                .then(|| SimpleMovingAverage::new(kc_length))
                .transpose()?,
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
    ///
    /// M4: with the default `bb_length == kc_length` the Keltner basis is
    /// literally the Bollinger midline — the same SMA of close over the same
    /// window — so only one of the two is maintained. Same inputs, same
    /// period, same recurrence, therefore the same bits.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> SqueezeValue {
        let bb_mid = self.bb_mid.append(close);
        let (bb_lower, bb_upper) = match (bb_mid, self.bb_dev.append(close)) {
            (Some(mid), Some(std)) => (mid - self.bb_std * std, mid + self.bb_std * std),
            _ => (f64::NAN, f64::NAN),
        };

        let kc_basis = match self.kc_basis.as_mut() {
            Some(kc_basis) => kc_basis.append(close),
            None => bb_mid,
        };
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
        if let Some(kc_basis) = self.kc_basis.as_mut() {
            kc_basis.reset();
        }
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
    /// See [`Squeeze::kc_basis`]: `None` reuses the Bollinger midline when
    /// `kc_length == bb_length` (M4).
    kc_basis: Option<SimpleMovingAverage>,
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
            kc_basis: (kc_length != bb_length)
                .then(|| SimpleMovingAverage::new(kc_length))
                .transpose()?,
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
    ///
    /// M4: the three Keltner levels already share one basis and one TR band;
    /// with `bb_length == kc_length` that basis is also the Bollinger midline,
    /// so the duplicate SMA of close is dropped (identical recurrence, so
    /// identical bits).
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> SqueezeProValue {
        let bb_mid = self.bb_mid.append(close);
        let (bb_lower, bb_upper) = match (bb_mid, self.bb_dev.append(close)) {
            (Some(mid), Some(std)) => (mid - self.bb_std * std, mid + self.bb_std * std),
            _ => (f64::NAN, f64::NAN),
        };

        let kc_basis = match self.kc_basis.as_mut() {
            Some(kc_basis) => kc_basis.append(close),
            None => bb_mid,
        };
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
        if let Some(kc_basis) = self.kc_basis.as_mut() {
            kc_basis.reset();
        }
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

    /// Bulk kernel for the MACD chain: once both EMAs are warm, their scalar
    /// recurrences advance in locals inside one loop; the two cascaded
    /// stochastic stages (rolling extrema + smoothing) advance in place with
    /// the exact per-bar arithmetic. Bit-identical to per-bar [`Self::append`]
    /// in outputs and post-run streaming state.
    pub fn extend_slices_into(
        &mut self,
        close: &[f64],
        stc_out: &mut Vec<f64>,
        macd_out: &mut Vec<f64>,
        stoch_out: &mut Vec<f64>,
    ) {
        stc_out.reserve(close.len());
        macd_out.reserve(close.len());
        stoch_out.reserve(close.len());
        let mut index = 0;
        // Warm-up prologue: per-bar appends until the slow EMA is seeded
        // (the fast EMA warms no later than the slow one).
        while index < close.len() && self.slow_ema.current().is_none() {
            let value = self.append(close[index]);
            stc_out.push(value.stc);
            macd_out.push(value.macd);
            stoch_out.push(value.stoch);
            index += 1;
        }
        if index == close.len() {
            return;
        }

        let fast_k = self.fast_ema.smoothing();
        let slow_k = self.slow_ema.smoothing();
        let mut fast = self.fast_ema.current().expect("warm fast EMA");
        let mut slow = self.slow_ema.current().expect("warm slow EMA");
        let factor = self.factor;
        let mut last = self.value;
        for &close_value in &close[index..] {
            fast = fast_k.mul_add(close_value - fast, fast);
            slow = slow_k.mul_add(close_value - slow, slow);
            let macd = fast - slow;

            let lowest = self.xmacd_low.append(macd).unwrap_or(f64::NAN);
            let highest = self.xmacd_high.append(macd).unwrap_or(f64::NAN);
            let range = non_zero(highest - lowest);
            if lowest > 0.0 {
                self.stoch1 = 100.0 * ((macd - lowest) / range);
            }
            self.pf = round8(self.pf + factor * (self.stoch1 - self.pf));

            let lowest_pf = self.pf_low.append(self.pf).unwrap_or(f64::NAN);
            let highest_pf = self.pf_high.append(self.pf).unwrap_or(f64::NAN);
            let range_pf = non_zero(highest_pf - lowest_pf);
            if range_pf > 0.0 {
                self.stoch2 = 100.0 * ((self.pf - lowest_pf) / range_pf);
            }
            self.pff = round8(self.pff + factor * (self.stoch2 - self.pff));

            let value = SchaffTrendCycleValue {
                stc: self.pff,
                macd,
                stoch: self.pf,
            };
            stc_out.push(value.stc);
            macd_out.push(value.macd);
            stoch_out.push(value.stoch);
            last = Some(value);
        }

        let appended = close.len() - index;
        self.fast_ema.store_bulk_state(fast, appended);
        self.slow_ema.store_bulk_state(slow, appended);
        self.value = last;
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

    /// Bulk kernel: once every ROC/SMA chain is warm, advances the four
    /// sliding-sum recurrences in one loop with the running sums held in
    /// locals while the rings advance in place. Bit-identical to per-bar
    /// [`Self::append`] in outputs and post-run streaming state.
    pub fn extend_slices_into(
        &mut self,
        close: &[f64],
        kst_out: &mut Vec<f64>,
        signal_out: &mut Vec<f64>,
    ) {
        kst_out.reserve(close.len());
        signal_out.reserve(close.len());
        let mut index = 0;
        // Warm-up prologue: per-bar appends until KST is non-NaN, which
        // implies every ROC window and every SMA window is full.
        while index < close.len() && self.value.map_or(true, |value| value.kst.is_nan()) {
            let value = self.append(close[index]);
            kst_out.push(value.kst);
            signal_out.push(value.signal);
            index += 1;
        }
        if index == close.len() {
            return;
        }

        let [chain1, chain2, chain3, chain4] = &mut self.rocs;
        let period1 = chain1.sma.period() as f64;
        let period2 = chain2.sma.period() as f64;
        let period3 = chain3.sma.period() as f64;
        let period4 = chain4.sma.period() as f64;
        let mut sum1 = chain1.sma.raw_sum();
        let mut sum2 = chain2.sma.raw_sum();
        let mut sum3 = chain3.sma.raw_sum();
        let mut sum4 = chain4.sma.raw_sum();
        let (mut rocma1, mut rocma2, mut rocma3, mut rocma4) =
            (f64::NAN, f64::NAN, f64::NAN, f64::NAN);
        let (mut kst, mut signal) = (f64::NAN, f64::NAN);
        for &close_value in &close[index..] {
            let previous1 = chain1
                .close_window
                .push(close_value)
                .expect("full ROC window");
            let ratio1 = (close_value - previous1) / previous1;
            let evicted1 = chain1
                .sma
                .window_mut()
                .push(ratio1)
                .expect("full SMA window");
            sum1 -= evicted1;
            sum1 += ratio1;
            rocma1 = sum1 / period1;

            let previous2 = chain2
                .close_window
                .push(close_value)
                .expect("full ROC window");
            let ratio2 = (close_value - previous2) / previous2;
            let evicted2 = chain2
                .sma
                .window_mut()
                .push(ratio2)
                .expect("full SMA window");
            sum2 -= evicted2;
            sum2 += ratio2;
            rocma2 = sum2 / period2;

            let previous3 = chain3
                .close_window
                .push(close_value)
                .expect("full ROC window");
            let ratio3 = (close_value - previous3) / previous3;
            let evicted3 = chain3
                .sma
                .window_mut()
                .push(ratio3)
                .expect("full SMA window");
            sum3 -= evicted3;
            sum3 += ratio3;
            rocma3 = sum3 / period3;

            let previous4 = chain4
                .close_window
                .push(close_value)
                .expect("full ROC window");
            let ratio4 = (close_value - previous4) / previous4;
            let evicted4 = chain4
                .sma
                .window_mut()
                .push(ratio4)
                .expect("full SMA window");
            sum4 -= evicted4;
            sum4 += ratio4;
            rocma4 = sum4 / period4;

            kst = 100.0 * (rocma1 + 2.0 * rocma2 + 3.0 * rocma3 + 4.0 * rocma4);
            signal = self.signal_state.append(kst).unwrap_or(f64::NAN);
            kst_out.push(kst);
            signal_out.push(signal);
        }

        chain1.sma.store_bulk_state(sum1, Some(rocma1));
        chain2.sma.store_bulk_state(sum2, Some(rocma2));
        chain3.sma.store_bulk_state(sum3, Some(rocma3));
        chain4.sma.store_bulk_state(sum4, Some(rocma4));
        self.value = Some(KnowSureThingValue { kst, signal });
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
    /// Rolling extrema over the confirmation window (`2 * length + 1`).
    high_extrema: MonotonicMax,
    low_extrema: MonotonicMin,
    /// Delay lines of `length + 1` bars: their oldest slot is the center bar
    /// under test once the confirmation window is full.
    center_highs: ContiguousWindow,
    center_lows: ContiguousWindow,
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
            high_extrema: MonotonicMax::new(capacity)?,
            low_extrema: MonotonicMin::new(capacity)?,
            center_highs: ContiguousWindow::new(length + 1),
            center_lows: ContiguousWindow::new(length + 1),
            bars_since: None,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    ///
    /// M1: the two O(2·length) window rescans become amortized-O(1) monotonic
    /// deques, and the center bar comes from a fixed delay ring instead of
    /// indexing a `VecDeque`. Extrema are comparison-only, so the confirmed
    /// signals and levels are bit-identical to the rescan version.
    pub fn append(&mut self, high: f64, low: f64) -> Option<SwingValue> {
        let window_high = self.high_extrema.append(high);
        let window_low = self.low_extrema.append(low);
        self.center_highs.push(high);
        self.center_lows.push(low);

        let (Some(window_high), Some(window_low)) = (window_high, window_low) else {
            self.value = None;
            return None;
        };
        let center_high = self.center_highs.window()[0];
        let center_low = self.center_lows.window()[0];
        let is_high = center_high >= window_high;
        let is_low = center_low <= window_low;
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
        self.high_extrema.reset();
        self.low_extrema.reset();
        self.center_highs.clear();
        self.center_lows.clear();
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
    window: super::sorted_ring::SortedRing,
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
            window: super::sorted_ring::SortedRing::new(timeperiod),
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
    ///
    /// The window is a shared sorted ring; the interpolation arithmetic is
    /// unchanged from the per-bar full-sort implementation, so outputs stay
    /// bit-identical.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.window.push(input);
        self.value = if self.window.is_full() {
            let sorted = self.window.sorted();
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
        self.window.clear();
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
///
/// Carries **no sliding accumulator**: mean and variance are recomputed from
/// the retained window with a fresh two-pass scan on every bar, so there is
/// nothing to reseed and no drift to bound (measured against a long-double
/// reference over 100k AR(1) price bars: 4.6e-14 max absolute error). The
/// residual ~2e-8 mismatch the benchmark reports for this function is the
/// pandas oracle's own `rolling().mean()/std()` Welford drift, amplified at
/// low-variance windows — not an error on this side.
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
            values: Window,
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
                    values: Window::new(timeperiod)?,
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
            ///
            /// The moment recurrences are already O(1); the window is a fixed
            /// ring (never a `VecDeque`) and only supplies the evicted value,
            /// so the arithmetic — and therefore every emitted bit — is
            /// unchanged.
            pub fn append(&mut self, input: f64) -> Option<f64> {
                if let Some(old) = self.values.push(input) {
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
        self.value = if self.quantile.window.is_full() {
            let sorted = self.quantile.window.sorted();
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
    window: super::sorted_ring::SortedRing,
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
            window: super::sorted_ring::SortedRing::new(timeperiod),
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
    ///
    /// The window is a shared sorted ring; the quantile interpolation and
    /// `max`/`min` clamping are unchanged from the per-bar full-sort
    /// implementation, so outputs stay bit-identical.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.window.push(input);
        self.value = if self.window.is_full() {
            let sorted = self.window.sorted();
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
        self.window.clear();
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

/// Slice twin of [`weighted_mean`]: identical iteration order and arithmetic
/// (`v * (i + 1)` accumulated oldest → newest), so results are bit-identical
/// when the slice holds the same values front-to-back as the deque.
fn weighted_mean_slice(values: &[f64]) -> f64 {
    let denominator = (values.len() * (values.len() + 1) / 2) as f64;
    values
        .iter()
        .enumerate()
        .map(|(i, &v)| v * (i + 1) as f64)
        .sum::<f64>()
        / denominator
}

/// Fixed-capacity FIFO whose live window is always one contiguous slice.
///
/// Backed by a double-write buffer of `2 * capacity`: every value is stored
/// at `pos` and `pos + capacity`, so the logical window (oldest → newest) is
/// a single `&[f64]` — per-window rescans read straight-line memory instead
/// of chasing a deque. Allocates exactly once; `clear` never reallocates.
#[derive(Debug, Clone)]
struct ContiguousWindow {
    buf: Box<[f64]>,
    cap: usize,
    len: usize,
    /// Next write slot in `0..cap`.
    pos: usize,
}

impl ContiguousWindow {
    fn new(cap: usize) -> Self {
        debug_assert!(cap >= 1);
        Self {
            buf: vec![0.0; 2 * cap].into_boxed_slice(),
            cap,
            len: 0,
            pos: 0,
        }
    }

    #[inline]
    fn push(&mut self, value: f64) {
        self.buf[self.pos] = value;
        self.buf[self.pos + self.cap] = value;
        self.pos += 1;
        if self.pos == self.cap {
            self.pos = 0;
        }
        if self.len < self.cap {
            self.len += 1;
        }
    }

    #[inline]
    fn len(&self) -> usize {
        self.len
    }

    #[inline]
    fn is_full(&self) -> bool {
        self.len == self.cap
    }

    /// The live window, oldest → newest, as one contiguous slice.
    #[inline]
    fn window(&self) -> &[f64] {
        // Newest element sits at `pos - 1` (mod cap); its double-write copy
        // at `pos - 1 + cap` ends the contiguous run of the last `len` values.
        let end = self.pos + self.cap;
        &self.buf[end - self.len..end]
    }

    #[inline]
    fn clear(&mut self) {
        self.len = 0;
        self.pos = 0;
    }
}

/// One monotonic staircase answering rolling max/min for **several** window
/// lengths at once.
///
/// A single deque built for the longest period `P` holds exactly the elements
/// not dominated by a later one, values strictly monotone from front to back.
/// For any `p <= P` the extremum of the last `p` bars is the first entry whose
/// index is still inside that shorter window — found by binary search. Three
/// nested windows therefore need one staircase per side instead of one deque
/// each. Backed by a fixed ring (`Box<[(usize, f64)]>`), never a `VecDeque`.
#[derive(Debug, Clone)]
struct MultiPeriodStaircase {
    buf: Box<[(usize, f64)]>,
    head: usize,
    len: usize,
    /// Number of observations consumed since construction/reset.
    index: usize,
    /// Longest window this staircase serves.
    longest: usize,
    /// `true` for a max staircase, `false` for a min staircase.
    maximum: bool,
}

impl MultiPeriodStaircase {
    fn new(longest: usize, maximum: bool) -> Self {
        debug_assert!(longest >= 1);
        Self {
            buf: vec![(0usize, 0.0f64); longest].into_boxed_slice(),
            head: 0,
            len: 0,
            index: 0,
            longest,
            maximum,
        }
    }

    #[inline]
    fn entry(&self, offset: usize) -> (usize, f64) {
        let capacity = self.buf.len();
        let mut slot = self.head + offset;
        if slot >= capacity {
            slot -= capacity;
        }
        self.buf[slot]
    }

    /// Pushes one observation, evicting entries that can never be an
    /// extremum again. Pop-on-equal (newest wins) matches `MonotonicMax`.
    fn push(&mut self, value: f64) {
        let capacity = self.buf.len();
        let index = self.index;
        self.index += 1;
        while self.len > 0 {
            let (_, back) = self.entry(self.len - 1);
            let dominated = if self.maximum {
                back <= value
            } else {
                back >= value
            };
            if !dominated {
                break;
            }
            self.len -= 1;
        }
        // Drop aged-out entries *before* inserting: the live entries then all
        // carry distinct indices inside the longest window, so they always fit
        // the ring's `longest` slots.
        let first_valid = index.saturating_add(1).saturating_sub(self.longest);
        while self.len > 0 && self.entry(0).0 < first_valid {
            self.head += 1;
            if self.head == capacity {
                self.head = 0;
            }
            self.len -= 1;
        }
        let mut tail = self.head + self.len;
        if tail >= capacity {
            tail -= capacity;
        }
        self.buf[tail] = (index, value);
        self.len += 1;
    }

    /// The extremum over the last `period` observations, or `None` while
    /// fewer than `period` observations have been seen.
    fn extremum(&self, period: usize) -> Option<f64> {
        debug_assert!(period <= self.longest);
        if self.index < period {
            return None;
        }
        let first_valid = self.index - period;
        // Entries are index-ascending: find the first one inside the window.
        let mut low = 0;
        let mut high = self.len;
        while low < high {
            let middle = (low + high) / 2;
            if self.entry(middle).0 < first_valid {
                low = middle + 1;
            } else {
                high = middle;
            }
        }
        Some(self.entry(low).1)
    }

    fn reset(&mut self) {
        self.head = 0;
        self.len = 0;
        self.index = 0;
    }
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `HullMovingAverage`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct HullMovingAverage {
    raw: ContiguousWindow,
    intermediate: ContiguousWindow,
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
        let smooth = ((period as f64).sqrt().floor() as usize).max(1);
        Ok(Self {
            raw: ContiguousWindow::new(period),
            intermediate: ContiguousWindow::new(smooth),
            period,
            half,
            smooth,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    /// The WMA rescans run over contiguous ring slices in the same
    /// oldest-to-newest order (and therefore the same rounding) as the
    /// historical deque implementation, without its per-bar allocation.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.raw.push(input);
        if self.raw.is_full() {
            let window = self.raw.window();
            let half = weighted_mean_slice(&window[self.period - self.half..]);
            let full = weighted_mean_slice(window);
            self.intermediate.push(2.0 * half - full);
            self.value = self
                .intermediate
                .is_full()
                .then(|| weighted_mean_slice(self.intermediate.window()));
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
    prices: ContiguousWindow,
    volumes: ContiguousWindow,
    value: Option<f64>,
}
impl VolumeWeightedMovingAverage {
    /// Create a new empty state.
    ///
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            prices: ContiguousWindow::new(period),
            volumes: ContiguousWindow::new(period),
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    /// The two window sums stay per-bar rescans over contiguous ring slices:
    /// converting them to sliding add/evict sums would reassociate the
    /// additions and change the low bits versus the historical fresh
    /// oldest-to-newest summation this state has always emitted.
    pub fn append(&mut self, price: f64, volume: f64) -> Option<f64> {
        self.prices.push(price);
        self.volumes.push(volume);
        self.value = self.prices.is_full().then(|| {
            let prices = self.prices.window();
            let volumes = self.volumes.window();
            let volume = volumes.iter().sum::<f64>();
            if volume != 0.0 {
                prices
                    .iter()
                    .zip(volumes)
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
    values: ContiguousWindow,
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
            values: ContiguousWindow::new(slow),
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    /// Both means read one contiguous ring slice (the fast leg is the tail of
    /// the slow window), so the two SMAs share a single pass over the same
    /// cache lines. The summation orders are unchanged — the fast sum still
    /// runs newest → oldest and the slow sum oldest → newest — because
    /// reassociating either one moves the low bits of the difference.
    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        self.values.push((high + low) * 0.5);
        self.value = self.values.is_full().then(|| {
            let window = self.values.window();
            let fast = window[self.slow - self.fast..].iter().rev().sum::<f64>() / self.fast as f64;
            let slow = window.iter().sum::<f64>() / self.slow as f64;
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
    highs: MonotonicMax,
    lows: MonotonicMin,
    previous: f64,
    value: Option<f64>,
}
impl FisherTransform {
    /// Create a new empty state.
    ///
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            highs: MonotonicMax::new(period)?,
            lows: MonotonicMin::new(period)?,
            previous: 0.0,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    /// The window max/min feed is a pair of monotonic deques (amortized O(1))
    /// instead of an O(period) rescan of a value deque; the extrema they
    /// report are the same numbers the rescan folded, so the transform is
    /// bit-identical.
    pub fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        let midpoint = (high + low) * 0.5;
        let maximum = self.highs.append(midpoint);
        let minimum = self.lows.append(midpoint);
        self.value = maximum.zip(minimum).map(|(high, low)| {
            let normalized = if high != low {
                2.0 * ((midpoint - low) / (high - low) - 0.5)
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
        self.highs.reset();
        self.lows.reset();
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
    highs: MonotonicMax,
    lows: MonotonicMin,
    value: Option<DonchianValue>,
}
impl Donchian {
    /// Create a new empty state.
    ///
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            highs: MonotonicMax::new(period)?,
            lows: MonotonicMin::new(period)?,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    /// M1: the two O(period) extrema rescans become amortized-O(1) monotonic
    /// deques. Extrema are comparison-only, so the emitted bands are the same
    /// values the rescans produced.
    pub fn append(&mut self, high: f64, low: f64) -> Option<DonchianValue> {
        let upper = self.highs.append(high);
        let lower = self.lows.append(low);
        self.value = upper.zip(lower).map(|(upper, lower)| DonchianValue {
            upper,
            lower,
            middle: (upper + lower) * 0.5,
        });
        self.value
    }
    /// Bulk kernel: one vHGW max pass over `high` and one vHGW min pass over
    /// `low`, with the midline derived in the same flat loop. The trailing
    /// `period` inputs are replayed to rebuild the monotonic deques, so outputs
    /// and post-run state are bit-identical to per-bar [`Self::append`];
    /// warm-up bars are NaN.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        upper_out: &mut Vec<f64>,
        lower_out: &mut Vec<f64>,
        middle_out: &mut Vec<f64>,
    ) -> TaResult<()> {
        if high.len() != low.len() {
            return Err(TaError::LengthMismatch {
                expected: high.len(),
                got: low.len(),
            });
        }
        let n = high.len();
        let period = self.highs.period();
        if self.highs.count() != 0 || n < period {
            upper_out.reserve(n);
            lower_out.reserve(n);
            middle_out.reserve(n);
            for index in 0..n {
                match self.append(high[index], low[index]) {
                    Some(value) => {
                        upper_out.push(value.upper);
                        lower_out.push(value.lower);
                        middle_out.push(value.middle);
                    }
                    None => {
                        upper_out.push(f64::NAN);
                        lower_out.push(f64::NAN);
                        middle_out.push(f64::NAN);
                    }
                }
            }
            return Ok(());
        }
        let upper_start = upper_out.len();
        let lower_start = lower_out.len();
        let middle_start = middle_out.len();
        upper_out.resize(upper_start + n, f64::NAN);
        lower_out.resize(lower_start + n, f64::NAN);
        middle_out.resize(middle_start + n, f64::NAN);
        super::vhgw::sliding_max_into(high, period, &mut upper_out[upper_start + period - 1..]);
        super::vhgw::sliding_min_into(low, period, &mut lower_out[lower_start + period - 1..]);
        for (slot, (&upper, &lower)) in middle_out[middle_start + period - 1..].iter_mut().zip(
            upper_out[upper_start + period - 1..]
                .iter()
                .zip(&lower_out[lower_start + period - 1..]),
        ) {
            *slot = (upper + lower) * 0.5;
        }
        self.highs.rebuild_from_full_run(high);
        self.lows.rebuild_from_full_run(low);
        self.value = Some(DonchianValue {
            upper: *upper_out.last().expect("at least one warmed bar"),
            lower: *lower_out.last().expect("at least one warmed bar"),
            middle: *middle_out.last().expect("at least one warmed bar"),
        });
        Ok(())
    }
    /// Return the latest computed result, if warm-up is complete.
    ///
    pub fn value(&self) -> Option<DonchianValue> {
        self.value
    }
    /// Reset the state and clear its accumulated history.
    ///
    pub fn reset(&mut self) {
        self.highs.reset();
        self.lows.reset();
        self.value = None;
    }
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `UlcerIndex`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct UlcerIndex {
    values: ContiguousWindow,
    period: usize,
    value: Option<f64>,
}
impl UlcerIndex {
    /// Create a new empty state.
    ///
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            values: ContiguousWindow::new(period),
            period,
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    /// The drawdown peak is a *prefix* maximum inside the window, so every
    /// squared-drawdown term is re-derived when the window slides: neither a
    /// rolling-window max structure nor a sliding sum can reproduce this
    /// series. The scan therefore stays O(period), but over one contiguous
    /// ring slice instead of a deque, preserving the summation order exactly.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.values.push(input);
        self.value = self.values.is_full().then(|| {
            let mut peak = f64::NEG_INFINITY;
            let sum = self
                .values
                .window()
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
    prices: ContiguousWindow,
    volumes: ContiguousWindow,
    value: Option<f64>,
}
impl RollingVolumeWeightedAveragePrice {
    /// Create a new empty state.
    ///
    pub fn new(period: usize) -> TaResult<Self> {
        validate_period(period)?;
        Ok(Self {
            prices: ContiguousWindow::new(period),
            volumes: ContiguousWindow::new(period),
            value: None,
        })
    }
    /// Append one causal observation and return the latest result.
    ///
    /// Like [`VolumeWeightedMovingAverage`], both window sums are contiguous
    /// per-bar rescans: sliding add/evict sums would reassociate them and
    /// perturb the low bits of the historical output.
    pub fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> Option<f64> {
        self.prices.push((high + low + close) / 3.0);
        self.volumes.push(volume);
        self.value = self.prices.is_full().then(|| {
            let prices = self.prices.window();
            let volumes = self.volumes.window();
            let total = volumes.iter().sum::<f64>();
            if total != 0.0 {
                prices
                    .iter()
                    .zip(volumes)
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

/// Rolling Calmar ratio: window mean over the window's maximum drawdown.
///
/// Split out of `rolling_risk_operator!` because the maximum drawdown is
/// driven by a *prefix* maximum inside the window — no rolling-extrema
/// structure and no sliding sum can reproduce the series when the window
/// slides. The O(period) rescan is therefore inherent; what this version
/// removes is the deque (one contiguous ring slice instead) and the second
/// pass: the window sum is accumulated in the same oldest-to-newest order,
/// inside the drawdown loop, so the emitted ratio is bit-identical.
#[derive(Debug, Clone)]
pub struct RollingCalmar {
    values: ContiguousWindow,
    timeperiod: usize,
    value: Option<f64>,
}

impl RollingCalmar {
    /// Creates the state for a positive rolling window.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self {
            values: ContiguousWindow::new(timeperiod),
            timeperiod,
            value: None,
        })
    }

    /// Append one causal observation and return the latest result.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.values.push(input);
        self.value = self.values.is_full().then(|| {
            let window = self.values.window();
            let mut sum = 0.0;
            let mut peak = window[0];
            let mut drawdown: f64 = 0.0;
            for &value in window {
                sum += value;
                peak = peak.max(value);
                drawdown = drawdown.min(if peak != 0.0 { value / peak - 1.0 } else { 0.0 });
            }
            let average = sum / self.timeperiod as f64;
            if drawdown < 0.0 {
                average / -drawdown
            } else {
                0.0
            }
        });
        self.value
    }

    /// Return the latest computed result, if warm-up is complete.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.values.clear();
        self.value = None;
    }
}

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

    fn bulk_lcg_series(n: usize, mut state: u64) -> Vec<f64> {
        (0..n)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                90.0 + (state >> 11) as f64 / (1u64 << 53) as f64 * 20.0
            })
            .collect()
    }

    fn assert_same_bits(actual: &[f64], expected: &[f64], label: &str) {
        assert_eq!(actual.len(), expected.len(), "{label}: length");
        for (i, (a, b)) in actual.iter().zip(expected).enumerate() {
            assert_eq!(a.to_bits(), b.to_bits(), "{label}: bar {i}");
        }
    }

    #[test]
    fn know_sure_thing_bulk_is_bitwise_identical_to_per_bar_append() {
        let input = bulk_lcg_series(5_000, 0x5EED_0357);
        let tail = bulk_lcg_series(128, 0x7A11_0357);
        let combos = [
            (
                10usize, 15usize, 20usize, 30usize, 10usize, 10usize, 10usize, 15usize, 9usize,
            ),
            (1, 1, 1, 1, 1, 1, 1, 1, 1),
            (2, 3, 4, 5, 2, 2, 2, 2, 3),
            (5, 5, 5, 5, 8, 8, 8, 8, 2),
        ];
        for (r1, r2, r3, r4, s1, s2, s3, s4, nsig) in combos {
            let mut per_bar = KnowSureThing::new(r1, r2, r3, r4, s1, s2, s3, s4, nsig).unwrap();
            let mut ref_kst = Vec::new();
            let mut ref_signal = Vec::new();
            for &x in &input {
                let value = per_bar.append(x);
                ref_kst.push(value.kst);
                ref_signal.push(value.signal);
            }
            let mut tail_kst = Vec::new();
            let mut tail_signal = Vec::new();
            for &x in &tail {
                let value = per_bar.append(x);
                tail_kst.push(value.kst);
                tail_signal.push(value.signal);
            }

            for chunk in [usize::MAX, 1, 7, 97] {
                let mut state = KnowSureThing::new(r1, r2, r3, r4, s1, s2, s3, s4, nsig).unwrap();
                let (mut kst_out, mut signal_out) = (Vec::new(), Vec::new());
                for piece in input.chunks(chunk.min(input.len())) {
                    state.extend_slices_into(piece, &mut kst_out, &mut signal_out);
                }
                let label = format!("kst {r1}/{s1}/{nsig} chunk {chunk}");
                assert_same_bits(&kst_out, &ref_kst, &label);
                assert_same_bits(&signal_out, &ref_signal, &label);
                let (mut tk, mut ts) = (Vec::new(), Vec::new());
                for &x in &tail {
                    let value = state.append(x);
                    tk.push(value.kst);
                    ts.push(value.signal);
                }
                assert_same_bits(&tk, &tail_kst, &format!("{label} tail"));
                assert_same_bits(&ts, &tail_signal, &format!("{label} tail"));
            }
        }
    }

    #[test]
    fn schaff_trend_cycle_bulk_is_bitwise_identical_to_per_bar_append() {
        let input = bulk_lcg_series(5_000, 0x5EED_057C);
        let tail = bulk_lcg_series(128, 0x7A11_057C);
        let combos = [
            (10usize, 23usize, 50usize, 0.5),
            (1, 2, 2, 0.5),
            (3, 5, 5, 1.0),
            (2, 2, 30, 0.25),
        ];
        for (tclength, fast, slow, factor) in combos {
            let mut per_bar = SchaffTrendCycle::new(tclength, fast, slow, factor).unwrap();
            let (mut ref_stc, mut ref_macd, mut ref_stoch) = (Vec::new(), Vec::new(), Vec::new());
            for &x in &input {
                let value = per_bar.append(x);
                ref_stc.push(value.stc);
                ref_macd.push(value.macd);
                ref_stoch.push(value.stoch);
            }
            let (mut tail_stc, mut tail_macd, mut tail_stoch) =
                (Vec::new(), Vec::new(), Vec::new());
            for &x in &tail {
                let value = per_bar.append(x);
                tail_stc.push(value.stc);
                tail_macd.push(value.macd);
                tail_stoch.push(value.stoch);
            }

            for chunk in [usize::MAX, 1, 7, 97] {
                let mut state = SchaffTrendCycle::new(tclength, fast, slow, factor).unwrap();
                let (mut s, mut m, mut t) = (Vec::new(), Vec::new(), Vec::new());
                for piece in input.chunks(chunk.min(input.len())) {
                    state.extend_slices_into(piece, &mut s, &mut m, &mut t);
                }
                let label = format!("stc {tclength}/{fast}/{slow} chunk {chunk}");
                assert_same_bits(&s, &ref_stc, &label);
                assert_same_bits(&m, &ref_macd, &label);
                assert_same_bits(&t, &ref_stoch, &label);
                let (mut xs, mut xm, mut xt) = (Vec::new(), Vec::new(), Vec::new());
                for &x in &tail {
                    let value = state.append(x);
                    xs.push(value.stc);
                    xm.push(value.macd);
                    xt.push(value.stoch);
                }
                assert_same_bits(&xs, &tail_stc, &format!("{label} tail"));
                assert_same_bits(&xm, &tail_macd, &format!("{label} tail"));
                assert_same_bits(&xt, &tail_stoch, &format!("{label} tail"));
            }
        }
    }

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

#[cfg(test)]
mod donchian_bulk_tests {
    use super::Donchian;

    fn lcg_series(len: usize, seed: u64) -> Vec<f64> {
        let mut state = seed;
        (0..len)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                ((state >> 33) % 100_003) as f64 / 101.0
            })
            .collect()
    }

    #[test]
    fn donchian_bulk_matches_append_bitwise() {
        let base = lcg_series(5_000, 0x00DC_1A11_2233_4455);
        let high: Vec<f64> = base.iter().map(|v| v + 0.5).collect();
        let low: Vec<f64> = base.iter().map(|v| v - 0.5).collect();
        for period in [2usize, 5, 14, 30, 200] {
            let mut reference = Donchian::new(period).unwrap();
            let expected: Vec<(f64, f64, f64)> = (0..base.len())
                .map(|i| match reference.append(high[i], low[i]) {
                    Some(value) => (value.upper, value.lower, value.middle),
                    None => (f64::NAN, f64::NAN, f64::NAN),
                })
                .collect();
            for chunk in [1usize, 7, 97, base.len()] {
                let mut state = Donchian::new(period).unwrap();
                let (mut upper, mut lower, mut middle) = (Vec::new(), Vec::new(), Vec::new());
                let mut offset = 0;
                while offset < base.len() {
                    let end = (offset + chunk).min(base.len());
                    state
                        .extend_slices_into(
                            &high[offset..end],
                            &low[offset..end],
                            &mut upper,
                            &mut lower,
                            &mut middle,
                        )
                        .unwrap();
                    offset = end;
                }
                assert_eq!(upper.len(), base.len());
                for (i, (eu, el, em)) in expected.iter().enumerate() {
                    assert_eq!(
                        eu.to_bits(),
                        upper[i].to_bits(),
                        "upper p={period} c={chunk} i={i}"
                    );
                    assert_eq!(
                        el.to_bits(),
                        lower[i].to_bits(),
                        "lower p={period} c={chunk} i={i}"
                    );
                    assert_eq!(
                        em.to_bits(),
                        middle[i].to_bits(),
                        "middle p={period} c={chunk} i={i}"
                    );
                }
                let mut follow = reference.clone();
                for i in 0..256 {
                    assert_eq!(
                        follow.append(high[i], low[i]),
                        state.append(high[i], low[i]),
                        "continue p={period} c={chunk}"
                    );
                }
            }
        }
    }

    #[test]
    fn donchian_bulk_validates_lengths() {
        let mut state = Donchian::new(3).unwrap();
        let (mut u, mut l, mut m) = (Vec::new(), Vec::new(), Vec::new());
        assert!(state
            .extend_slices_into(&[1.0, 2.0], &[1.0], &mut u, &mut l, &mut m)
            .is_err());
    }
}

#[cfg(test)]
mod rolling_zscore_tests {
    use super::RollingZScore;

    fn lcg_series(n: usize, mut state: u64) -> Vec<f64> {
        (0..n)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                90.0 + (state >> 11) as f64 / (1u64 << 53) as f64 * 20.0
            })
            .collect()
    }

    /// Fresh two-pass z-score over `input[end + 1 - period..=end]`.
    fn exact_zscore(input: &[f64], end: usize, period: usize) -> f64 {
        let window = &input[end + 1 - period..=end];
        let period_f = period as f64;
        let mut sum = 0.0;
        for &value in window {
            sum += value;
        }
        let mean = sum / period_f;
        let mut variance = 0.0;
        for &value in window {
            variance += (value - mean) * (value - mean);
        }
        variance /= period_f;
        if variance > 0.0 {
            (input[end] - mean) / variance.sqrt()
        } else {
            0.0
        }
    }

    /// `RollingZScore` slides no accumulator, so it carries no drift at all:
    /// every bar is already a fresh window recomputation. This test pins that
    /// property so a future "optimisation" to O(1) sliding sums cannot silently
    /// introduce the drift the other rolling-moment states have to manage.
    #[test]
    fn streaming_has_zero_drift_over_1m_bars() {
        let input = lcg_series(1_000_000, 0x2500_D21F);
        for period in [14usize, 30] {
            let mut state = RollingZScore::new(period).unwrap();
            for i in 0..input.len() {
                let Some(value) = state.append(input[i]) else {
                    continue;
                };
                if (i + 1) % 50_000 != 0 && i + 1 != input.len() {
                    continue;
                }
                let exact = exact_zscore(&input, i, period);
                let drift = (value - exact).abs();
                assert!(
                    drift < 1e-12,
                    "RollingZScore p{period} bar {i}: drift {drift:e} vs a fresh window"
                );
            }
        }
    }

    /// Chunked `append` replay stays bitwise identical (there is no bulk kernel
    /// to diverge, but the state must not depend on where a run is split).
    #[test]
    fn chunked_replay_is_bitwise_identical() {
        let input = lcg_series(5_000, 0x2500_5EED);
        for period in [2usize, 14, 200] {
            let mut reference_state = RollingZScore::new(period).unwrap();
            let reference: Vec<f64> = input
                .iter()
                .map(|&x| reference_state.append(x).unwrap_or(f64::NAN))
                .collect();
            for chunk in [1usize, 7, 10, 97, 1000] {
                let mut state = RollingZScore::new(period).unwrap();
                let mut actual = Vec::new();
                for piece in input.chunks(chunk) {
                    for &x in piece {
                        actual.push(state.append(x).unwrap_or(f64::NAN));
                    }
                }
                for (i, (a, b)) in actual.iter().zip(&reference).enumerate() {
                    assert_eq!(
                        a.to_bits(),
                        b.to_bits(),
                        "zscore p{period} c{chunk} bar {i}"
                    );
                }
            }
        }
    }
}
