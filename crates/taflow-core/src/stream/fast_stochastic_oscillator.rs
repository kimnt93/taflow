//! Stateful Fast Stochastic Oscillator.
//!
//! STOCHF maintains rolling high/low extrema for fast %K and feeds that value
//! into the selected incremental moving average for fast %D.

use crate::error::{TaError, TaResult};
use crate::indicators::{RollingMaximum, RollingMinimum};
use crate::ma_type::MaType;

use super::{moving_average_dispatcher::MovingAverageDispatcher, StreamingIndicator};

/// One aligned fast %K and fast %D observation.
#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `FastStochasticOscillatorValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct FastStochasticOscillatorValue {
    pub fastk: f64,
    pub fastd: f64,
}

/// Incremental STOCHF with amortized constant work per bar.
/// Persistent Rust state or aligned output type for `FastStochasticOscillator`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct FastStochasticOscillator {
    highest: RollingMaximum,
    lowest: RollingMinimum,
    fastd: MovingAverageDispatcher,
    value: Option<FastStochasticOscillatorValue>,
}

impl FastStochasticOscillator {
    /// Creates a STOCHF state for the selected fast %D moving-average type.
    pub fn new(fastk_period: usize, fastd_period: usize, fastd_matype: MaType) -> TaResult<Self> {
        Ok(Self {
            highest: RollingMaximum::new(fastk_period)?,
            lowest: RollingMinimum::new(fastk_period)?,
            fastd: MovingAverageDispatcher::new(fastd_period, fastd_matype)?,
            value: None,
        })
    }

    /// Appends one high, low, and close bar.
    pub fn append(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
    ) -> Option<FastStochasticOscillatorValue> {
        let highest = self.highest.append(high);
        let lowest = self.lowest.append(low);
        let fastk = highest.zip(lowest).map(|(highest, lowest)| {
            let divisor = (highest - lowest) / 100.0;
            if divisor.abs() >= 1.0e-14 {
                (close - lowest) / divisor
            } else {
                0.0
            }
        });
        self.value = fastk.and_then(|fastk| {
            self.fastd
                .append(fastk)
                .map(|fastd| FastStochasticOscillatorValue { fastk, fastd })
        });
        self.value
    }

    /// Bulk kernel: vHGW sliding extrema for the fast %K window (via the
    /// `RollingMaximum`/`RollingMinimum` bulk paths, which also rebuild their deques),
    /// then the fast %D sub-state is driven per emitted bar exactly as
    /// [`Self::append`] does. Outputs and post-run state are bit-identical to
    /// per-bar [`Self::append`]; warm-up bars are NaN.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        fastk_out: &mut Vec<f64>,
        fastd_out: &mut Vec<f64>,
    ) -> TaResult<()> {
        if high.len() != low.len() || high.len() != close.len() {
            return Err(TaError::LengthMismatch {
                expected: high.len(),
                got: low.len().min(close.len()),
            });
        }
        let n = high.len();
        fastk_out.reserve(n);
        fastd_out.reserve(n);
        let period = self.highest.period();
        let consumed = self.highest.count();
        let mut highest = Vec::with_capacity(n);
        let mut lowest = Vec::with_capacity(n);
        self.highest.extend_slice_into(high, &mut highest);
        self.lowest.extend_slice_into(low, &mut lowest);
        for index in 0..n {
            if consumed + index + 1 < period {
                fastk_out.push(f64::NAN);
                fastd_out.push(f64::NAN);
                continue;
            }
            let (highest, lowest) = (highest[index], lowest[index]);
            let divisor = (highest - lowest) / 100.0;
            let fastk = if divisor.abs() >= 1.0e-14 {
                (close[index] - lowest) / divisor
            } else {
                0.0
            };
            self.value = self
                .fastd
                .append(fastk)
                .map(|fastd| FastStochasticOscillatorValue { fastk, fastd });
            match self.value {
                Some(value) => {
                    fastk_out.push(value.fastk);
                    fastd_out.push(value.fastd);
                }
                None => {
                    fastk_out.push(f64::NAN);
                    fastd_out.push(f64::NAN);
                }
            }
        }
        Ok(())
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<FastStochasticOscillatorValue> {
        self.value
    }

    /// Restores the post-construction state while retaining allocated buffers.
    pub fn reset(&mut self) {
        self.highest.reset();
        self.lowest.reset();
        self.fastd.reset();
        self.value = None;
    }
}
