//! Stateful Fast Stochastic Oscillator.
//!
//! STOCHF maintains rolling high/low extrema for fast %K and feeds that value
//! into the selected incremental moving average for fast %D.

use crate::error::{TaError, TaResult};
use crate::indicators::{RollingMaximum, RollingMinimum};
use crate::ma_type::MaType;

use crate::stream::{moving_average_dispatcher::MovingAverageDispatcher, StreamingIndicator};

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

    /// Bulk kernel: vHGW sliding extrema are written into the final output
    /// buffers as temporary maxima/minima, then transformed in place to fast
    /// %K/%D. This avoids two full-size scratch allocations while rebuilding
    /// extrema and moving-average state exactly as scalar replay does.
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
        let period = self.highest.period();
        let consumed = self.highest.count();
        let fastk_start = fastk_out.len();
        let fastd_start = fastd_out.len();
        self.highest.extend_slice_into(high, fastk_out);
        self.lowest.extend_slice_into(low, fastd_out);
        for index in 0..n {
            if consumed + index + 1 < period {
                continue;
            }
            let highest = fastk_out[fastk_start + index];
            let lowest = fastd_out[fastd_start + index];
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
                    fastk_out[fastk_start + index] = value.fastk;
                    fastd_out[fastd_start + index] = value.fastd;
                }
                None => {
                    fastk_out[fastk_start + index] = f64::NAN;
                    fastd_out[fastd_start + index] = f64::NAN;
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
