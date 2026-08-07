//! Rolling variance, deviation, correlation, and beta states.

use std::collections::VecDeque;

use crate::error::TaResult;

use super::{invalid_period, StreamingIndicator, Window};

#[derive(Debug, Clone)]
struct RollingMoments {
    inverse_period: f64,
    window: Window,
    sum: f64,
    sum_squares: f64,
}

impl RollingMoments {
    fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            inverse_period: 1.0 / period as f64,
            window: Window::new(period)?,
            sum: 0.0,
            sum_squares: 0.0,
        })
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        if self.window.is_full() {
            let old = self.window.push(input).expect("full moments window evicts");
            self.sum += input - old;
            self.sum_squares += (input - old).mul_add(input + old, 0.0);
        } else {
            self.window.push(input);
            self.sum += input;
            self.sum_squares = input.mul_add(input, self.sum_squares);
        }
        self.window.is_full().then(|| {
            let mean = self.sum * self.inverse_period;
            self.sum_squares * self.inverse_period - mean * mean
        })
    }

    fn reset(&mut self) {
        self.window.clear();
        self.sum = 0.0;
        self.sum_squares = 0.0;
    }
}

/// Stateful population variance. TA-Lib accepts but ignores `nbdev` for VAR.
#[derive(Debug, Clone)]
pub struct RollingVariance {
    moments: RollingMoments,
    value: Option<f64>,
}

impl RollingVariance {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize, _nbdev: f64) -> TaResult<Self> {
        Ok(Self {
            moments: RollingMoments::new(period)?,
            value: None,
        })
    }
}

impl StreamingIndicator for RollingVariance {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self.moments.append(input);
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.moments.reset();
        self.value = None;
    }
}

/// Stateful population standard deviation multiplied by `nbdev`.
#[derive(Debug, Clone)]
pub struct RollingStandardDeviation {
    moments: RollingMoments,
    nbdev: f64,
    value: Option<f64>,
}

impl RollingStandardDeviation {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize, nbdev: f64) -> TaResult<Self> {
        Ok(Self {
            moments: RollingMoments::new(period)?,
            nbdev,
            value: None,
        })
    }
}

impl StreamingIndicator for RollingStandardDeviation {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self
            .moments
            .append(input)
            .map(|variance| variance.max(0.0).sqrt() * self.nbdev);
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.moments.reset();
        self.value = None;
    }
}

/// Stateful average absolute deviation with TA-Lib's newest-to-oldest summation order.
#[derive(Debug, Clone)]
pub struct RollingAverageDeviation {
    period: usize,
    window: Window,
    value: Option<f64>,
}

impl RollingAverageDeviation {
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
            window: Window::new(period)?,
            value: None,
        })
    }
}

impl StreamingIndicator for RollingAverageDeviation {
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<f64> {
        self.window.push(input);
        self.value = self.window.is_full().then(|| {
            let period = self.period as f64;
            let mean = self.window.iter().rev().sum::<f64>() / period;
            self.window
                .iter()
                .rev()
                .map(|value| (*value - mean).abs())
                .sum::<f64>()
                / period
        });
        self.value
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn reset(&mut self) {
        self.window.clear();
        self.value = None;
    }
}

#[derive(Debug, Clone, Copy)]
struct PairMoments {
    sx: f64,
    sy: f64,
    sxx: f64,
    syy: f64,
    sxy: f64,
}

#[derive(Debug, Clone)]
struct RollingPairMoments {
    period: usize,
    window: VecDeque<(f64, f64)>,
    moments: PairMoments,
}

impl RollingPairMoments {
    fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            period,
            window: VecDeque::with_capacity(period),
            moments: PairMoments {
                sx: 0.0,
                sy: 0.0,
                sxx: 0.0,
                syy: 0.0,
                sxy: 0.0,
            },
        })
    }

    fn append(&mut self, x: f64, y: f64) -> Option<PairMoments> {
        if self.window.len() == self.period {
            let (old_x, old_y) = self.window.pop_front().expect("pair window is full");
            self.moments.sx += x - old_x;
            self.moments.sy += y - old_y;
            self.moments.sxx += x * x - old_x * old_x;
            self.moments.syy += y * y - old_y * old_y;
            self.moments.sxy += x * y - old_x * old_y;
        } else {
            self.moments.sx += x;
            self.moments.sy += y;
            self.moments.sxx += x * x;
            self.moments.syy += y * y;
            self.moments.sxy += x * y;
        }
        self.window.push_back((x, y));
        (self.window.len() == self.period).then_some(self.moments)
    }

    fn reset(&mut self) {
        self.window.clear();
        self.moments = PairMoments {
            sx: 0.0,
            sy: 0.0,
            sxx: 0.0,
            syy: 0.0,
            sxy: 0.0,
        };
    }

    fn reseed_linear_sums_with_batch_order(&mut self) -> PairMoments {
        let x: Vec<f64> = self.window.iter().map(|value| value.0).collect();
        let y: Vec<f64> = self.window.iter().map(|value| value.1).collect();
        self.moments.sx = crate::simd::sum_f64(&x);
        self.moments.sy = crate::simd::sum_f64(&y);
        self.moments
    }
}

/// Stateful Pearson correlation over paired observations.
#[derive(Debug, Clone)]
pub struct RollingCorrelation {
    period: f64,
    moments: RollingPairMoments,
    seeded: bool,
    value: Option<f64>,
}

impl RollingCorrelation {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            period: period as f64,
            moments: RollingPairMoments::new(period)?,
            seeded: false,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, x: f64, y: f64) -> Option<f64> {
        let moments = self.moments.append(x, y).map(|moments| {
            if self.seeded {
                moments
            } else {
                self.seeded = true;
                self.moments.reseed_linear_sums_with_batch_order()
            }
        });
        self.value = moments.map(|m| {
            let numerator = self.period * m.sxy - m.sx * m.sy;
            let denominator =
                ((self.period * m.sxx - m.sx * m.sx) * (self.period * m.syy - m.sy * m.sy)).sqrt();
            if denominator > 0.0 {
                numerator / denominator
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
        self.moments.reset();
        self.seeded = false;
        self.value = None;
    }
}

/// Stateful TA-Lib BETA over percentage returns of two input series.
#[derive(Debug, Clone)]
pub struct RollingBeta {
    period: f64,
    previous: Option<(f64, f64)>,
    returns: RollingPairMoments,
    value: Option<f64>,
}

impl RollingBeta {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            period: period as f64,
            previous: None,
            returns: RollingPairMoments::new(period)?,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, input0: f64, input1: f64) -> Option<f64> {
        let Some((previous0, previous1)) = self.previous.replace((input0, input1)) else {
            return None;
        };
        let x = (input0 - previous0) / previous0;
        let y = (input1 - previous1) / previous1;
        self.value = self.returns.append(x, y).map(|m| {
            let denominator = self.period * m.sxx - m.sx * m.sx;
            if denominator > 0.0 {
                (self.period * m.sxy - m.sx * m.sy) / denominator
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
        self.previous = None;
        self.returns.reset();
        self.value = None;
    }
}

