use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

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
