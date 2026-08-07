//! Rolling extrema and rolling extremum-index streaming states.

use std::collections::VecDeque;

use crate::error::TaResult;

use super::{invalid_period, StreamingIndicator};

#[derive(Debug, Clone)]
pub(crate) struct RollingExtrema {
    period: usize,
    index: usize,
    maximum: VecDeque<(usize, f64)>,
    minimum: VecDeque<(usize, f64)>,
}

impl RollingExtrema {
    pub(super) fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("timeperiod", period, 1));
        }
        Ok(Self {
            period,
            index: 0,
            maximum: VecDeque::with_capacity(period),
            minimum: VecDeque::with_capacity(period),
        })
    }

    pub(super) fn append_indexed(&mut self, input: f64) -> Option<((usize, f64), (usize, f64))> {
        let index = self.index;
        self.index += 1;
        while self
            .maximum
            .back()
            .is_some_and(|&(_, value)| value <= input)
        {
            self.maximum.pop_back();
        }
        while self
            .minimum
            .back()
            .is_some_and(|&(_, value)| value >= input)
        {
            self.minimum.pop_back();
        }
        self.maximum.push_back((index, input));
        self.minimum.push_back((index, input));
        let first_valid = index.saturating_add(1).saturating_sub(self.period);
        while self.maximum.front().is_some_and(|&(i, _)| i < first_valid) {
            self.maximum.pop_front();
        }
        while self.minimum.front().is_some_and(|&(i, _)| i < first_valid) {
            self.minimum.pop_front();
        }
        (index + 1 >= self.period).then(|| {
            (
                *self.maximum.front().expect("maximum queue is populated"),
                *self.minimum.front().expect("minimum queue is populated"),
            )
        })
    }

    pub(super) fn append(&mut self, input: f64) -> Option<(f64, f64)> {
        self.append_indexed(input)
            .map(|(maximum, minimum)| (maximum.1, minimum.1))
    }

    pub(super) fn reset(&mut self) {
        self.index = 0;
        self.maximum.clear();
        self.minimum.clear();
    }
}

macro_rules! rolling_extrema_indicator {
    ($name:ident, $select:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            extrema: RollingExtrema,
            value: Option<f64>,
        }

        impl $name {
            /// Computes or updates `new` through the native Rust kernel.
            ///
            /// Parameters are the typed series and configuration values in the signature.
            ///
            /// Returns the computed value, aligned history, or a validation error.
            pub fn new(period: usize) -> TaResult<Self> {
                Ok(Self {
                    extrema: RollingExtrema::new(period)?,
                    value: None,
                })
            }
        }

        impl StreamingIndicator for $name {
            type Output = f64;

            fn append(&mut self, input: f64) -> Option<f64> {
                self.value = self.extrema.append(input).map($select);
                self.value
            }

            fn value(&self) -> Option<f64> {
                self.value
            }

            fn reset(&mut self) {
                self.extrema.reset();
                self.value = None;
            }
        }
    };
}

rolling_extrema_indicator!(RollingMax, |(maximum, _)| maximum);
rolling_extrema_indicator!(RollingMin, |(_, minimum)| minimum);

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `RollingMinmaxValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingMinmaxValue {
    pub minimum: f64,
    pub maximum: f64,
}

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingMinmax`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingMinmax {
    extrema: RollingExtrema,
    value: Option<RollingMinmaxValue>,
}

impl RollingMinmax {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            extrema: RollingExtrema::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, input: f64) -> Option<RollingMinmaxValue> {
        self.value = self
            .extrema
            .append(input)
            .map(|(maximum, minimum)| RollingMinmaxValue { minimum, maximum });
        self.value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<RollingMinmaxValue> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.extrema.reset();
        self.value = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `RollingMinmaxIndexValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingMinmaxIndexValue {
    pub minimum: usize,
    pub maximum: usize,
}

#[derive(Debug, Clone)]
struct RollingIndexExtrema {
    period: usize,
    index: usize,
    window: VecDeque<(usize, f64)>,
    maximum: Option<(usize, f64)>,
    minimum: Option<(usize, f64)>,
}

impl RollingIndexExtrema {
    fn new(period: usize) -> TaResult<Self> {
        if period == 0 {
            return Err(invalid_period("timeperiod", period, 1));
        }
        Ok(Self {
            period,
            index: 0,
            window: VecDeque::with_capacity(period),
            maximum: None,
            minimum: None,
        })
    }

    fn append(&mut self, input: f64) -> RollingMinmaxIndexValue {
        let index = self.index;
        self.index += 1;
        if self.window.len() == self.period {
            self.window.pop_front();
        }
        self.window.push_back((index, input));
        if self.window.len() < self.period {
            return RollingMinmaxIndexValue {
                minimum: 0,
                maximum: 0,
            };
        }

        let first_valid = index + 1 - self.period;
        if self.maximum.is_none() || self.maximum.is_some_and(|(i, _)| i < first_valid) {
            self.maximum =
                self.window
                    .iter()
                    .copied()
                    .reduce(|best, current| if current.1 > best.1 { current } else { best });
        } else if self.maximum.is_some_and(|(_, value)| input >= value) {
            self.maximum = Some((index, input));
        }
        if self.minimum.is_none() || self.minimum.is_some_and(|(i, _)| i < first_valid) {
            self.minimum =
                self.window
                    .iter()
                    .copied()
                    .reduce(|best, current| if current.1 < best.1 { current } else { best });
        } else if self.minimum.is_some_and(|(_, value)| input <= value) {
            self.minimum = Some((index, input));
        }
        RollingMinmaxIndexValue {
            minimum: self.minimum.expect("full window has a minimum").0,
            maximum: self.maximum.expect("full window has a maximum").0,
        }
    }

    fn reset(&mut self) {
        self.index = 0;
        self.window.clear();
        self.maximum = None;
        self.minimum = None;
    }
}

macro_rules! rolling_index_indicator {
    ($name:ident, $select:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            extrema: RollingIndexExtrema,
            value: Option<f64>,
        }

        impl $name {
            /// Computes or updates `new` through the native Rust kernel.
            ///
            /// Parameters are the typed series and configuration values in the signature.
            ///
            /// Returns the computed value, aligned history, or a validation error.
            pub fn new(period: usize) -> TaResult<Self> {
                Ok(Self {
                    extrema: RollingIndexExtrema::new(period)?,
                    value: None,
                })
            }
        }

        impl StreamingIndicator for $name {
            type Output = f64;

            fn append(&mut self, input: f64) -> Option<f64> {
                let indices = self.extrema.append(input);
                self.value = Some($select(indices) as f64);
                self.value
            }

            fn value(&self) -> Option<f64> {
                self.value
            }

            fn reset(&mut self) {
                self.extrema.reset();
                self.value = None;
            }
        }
    };
}

rolling_index_indicator!(RollingArgmax, |value: RollingMinmaxIndexValue| value
    .maximum);
rolling_index_indicator!(RollingArgmin, |value: RollingMinmaxIndexValue| value
    .minimum);

#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `RollingMinmaxIndex`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct RollingMinmaxIndex {
    extrema: RollingIndexExtrema,
    value: Option<RollingMinmaxIndexValue>,
}

impl RollingMinmaxIndex {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            extrema: RollingIndexExtrema::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, input: f64) -> RollingMinmaxIndexValue {
        let value = self.extrema.append(input);
        self.value = Some(value);
        value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<RollingMinmaxIndexValue> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.extrema.reset();
        self.value = None;
    }
}
