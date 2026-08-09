//! Stateful Stochastic Oscillator.
//!
//! STOCH maintains rolling high/low extrema for fast %K, then feeds each
//! warmed value through independently selectable slow-%K and slow-%D moving
//! averages.

use crate::error::{TaError, TaResult};
use crate::indicators::{RollingMaximum, RollingMinimum};
use crate::ma_type::MaType;

use super::{moving_average_dispatcher::MovingAverageDispatcher, StreamingIndicator};

/// One aligned slow %K and slow %D observation.
#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `StochasticOscillatorValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct StochasticOscillatorValue {
    pub slowk: f64,
    pub slowd: f64,
}

/// Incremental STOCH with amortized constant work per bar.
/// Persistent Rust state or aligned output type for `StochasticOscillator`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct StochasticOscillator {
    highest: RollingMaximum,
    lowest: RollingMinimum,
    slowk: MovingAverageDispatcher,
    slowd: MovingAverageDispatcher,
    value: Option<StochasticOscillatorValue>,
}

impl StochasticOscillator {
    /// Creates a STOCH state for the selected smoothing types.
    pub fn new(
        fastk_period: usize,
        slowk_period: usize,
        slowk_matype: MaType,
        slowd_period: usize,
        slowd_matype: MaType,
    ) -> TaResult<Self> {
        Ok(Self {
            highest: RollingMaximum::new(fastk_period)?,
            lowest: RollingMinimum::new(fastk_period)?,
            slowk: MovingAverageDispatcher::new(slowk_period, slowk_matype)?,
            slowd: MovingAverageDispatcher::new(slowd_period, slowd_matype)?,
            value: None,
        })
    }

    /// Appends one high, low, and close bar.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<StochasticOscillatorValue> {
        let fastk =
            self.highest
                .append(high)
                .zip(self.lowest.append(low))
                .map(|(highest, lowest)| {
                    let divisor = (highest - lowest) / 100.0;
                    if divisor.abs() >= 1.0e-14 {
                        (close - lowest) / divisor
                    } else {
                        0.0
                    }
                });
        self.value = fastk
            .and_then(|fastk| self.slowk.append(fastk))
            .and_then(|slowk| {
                self.slowd
                    .append(slowk)
                    .map(|slowd| StochasticOscillatorValue { slowk, slowd })
            });
        self.value
    }

    /// Bulk kernel: vHGW sliding extrema for the fast %K window (via the
    /// `RollingMaximum`/`RollingMinimum` bulk paths, which also rebuild their deques),
    /// then the slow %K and slow %D moving averages are each driven by ONE
    /// bulk call instead of a per-bar dispatch.
    ///
    /// The MA seeds are order dependent, so each stage is fed its own inputs in
    /// the exact order [`Self::append`] would: the warm-up bars go through
    /// `append` one at a time (that is where `Option` warm-up and a genuine
    /// `NaN` output are indistinguishable), and the warmed remainder goes
    /// through the state's own bulk kernel, which is itself bit-identical to
    /// per-bar `append`. Outputs and post-run state are bit-identical to
    /// per-bar [`Self::append`]; warm-up bars are NaN.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        slowk_out: &mut Vec<f64>,
        slowd_out: &mut Vec<f64>,
    ) -> TaResult<()> {
        if high.len() != low.len() || high.len() != close.len() {
            return Err(TaError::LengthMismatch {
                expected: high.len(),
                got: low.len().min(close.len()),
            });
        }
        let n = high.len();
        slowk_out.reserve(n);
        slowd_out.reserve(n);
        let period = self.highest.period();
        let consumed = self.highest.count();
        let mut highest = Vec::with_capacity(n);
        let mut lowest = Vec::with_capacity(n);
        self.highest.extend_slice_into(high, &mut highest);
        self.lowest.extend_slice_into(low, &mut lowest);

        // Bars before the fast %K window fills emit NaN and feed nothing.
        let unwarmed = (period - 1).saturating_sub(consumed).min(n);
        // Fast %K in place over the extrema buffer; the warmed bars are then a
        // contiguous slice.
        for index in unwarmed..n {
            let high_extreme = highest[index];
            let low_extreme = lowest[index];
            let divisor = (high_extreme - low_extreme) / 100.0;
            highest[index] = if divisor.abs() >= 1.0e-14 {
                (close[index] - low_extreme) / divisor
            } else {
                0.0
            };
        }
        let fastk = &highest[unwarmed..];

        // Slow %K straight into its output cache, then slow %D over the warmed
        // slow %K values only — exactly the `and_then` chain `append` walks.
        // Neither stage needs a temporary buffer.
        let slowk_start = slowk_out.len();
        slowk_out.resize(slowk_start + unwarmed, f64::NAN);
        let slowk_unwarmed = Self::drive_until_warm(&mut self.slowk, fastk, slowk_out);
        let warmed_slowk = slowk_start + unwarmed + slowk_unwarmed;
        slowd_out.resize(slowd_out.len() + unwarmed + slowk_unwarmed, f64::NAN);
        let slowd_unwarmed =
            Self::drive_until_warm(&mut self.slowd, &slowk_out[warmed_slowk..], slowd_out);

        // A bar is emitted only once BOTH stages are warm, matching the
        // `and_then` chain: slow %K is NaN too while slow %D still warms.
        let emitted = unwarmed + slowk_unwarmed + slowd_unwarmed;
        debug_assert!(emitted <= n);
        slowk_out[warmed_slowk..warmed_slowk + slowd_unwarmed].fill(f64::NAN);
        if n != 0 {
            self.value = (emitted < n).then(|| StochasticOscillatorValue {
                slowk: slowk_out[slowk_out.len() - 1],
                slowd: slowd_out[slowd_out.len() - 1],
            });
        }
        Ok(())
    }

    /// Feeds `inputs` through `state`, appending one `f64` per bar, and returns
    /// the number of leading warm-up bars.
    ///
    /// Warm-up bars go one at a time so a warmed `NaN` output can never be
    /// mistaken for warm-up; the warmed tail runs through the state's bulk
    /// kernel in a single dispatch.
    fn drive_until_warm(
        state: &mut MovingAverageDispatcher,
        inputs: &[f64],
        output: &mut Vec<f64>,
    ) -> usize {
        if state.is_warm() {
            state.extend_slice_into(inputs, output);
            return 0;
        }
        let mut unwarmed = 0usize;
        let mut consumed = 0usize;
        while consumed < inputs.len() {
            let value = state.append(inputs[consumed]);
            consumed += 1;
            match value {
                Some(value) => {
                    output.push(value);
                    break;
                }
                None => {
                    output.push(f64::NAN);
                    unwarmed += 1;
                }
            }
        }
        state.extend_slice_into(&inputs[consumed..], output);
        unwarmed
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<StochasticOscillatorValue> {
        self.value
    }

    /// Restores the post-construction state while retaining allocated buffers.
    pub fn reset(&mut self) {
        self.highest.reset();
        self.lowest.reset();
        self.slowk.reset();
        self.slowd.reset();
        self.value = None;
    }
}
