use super::operator_states::*;
use crate::error::{TaError, TaResult};

use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

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
    pub(crate) factor: f64,
    pub(crate) fast_ema: ExponentialMovingAverage,
    pub(crate) slow_ema: ExponentialMovingAverage,
    pub(crate) xmacd_low: RollingExtremum,
    pub(crate) xmacd_high: RollingExtremum,
    pub(crate) pf_low: RollingExtremum,
    pub(crate) pf_high: RollingExtremum,
    pub(crate) stoch1: f64,
    pub(crate) pf: f64,
    pub(crate) stoch2: f64,
    pub(crate) pff: f64,
    pub(crate) value: Option<SchaffTrendCycleValue>,
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
        schaff_trend_cycle_bulk(self, close, stc_out, macd_out, stoch_out)
    }
}

impl SchaffTrendCycle {
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
