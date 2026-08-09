//! Persistent `Donchian` state.

use super::*;
use crate::error::{TaError, TaResult};
use crate::stream::operator_states::*;
use crate::stream::{vhgw, MonotonicMax, MonotonicMin};
use std::collections::{HashMap, HashSet, VecDeque};

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
        vhgw::sliding_max_into(high, period, &mut upper_out[upper_start + period - 1..]);
        vhgw::sliding_min_into(low, period, &mut lower_out[lower_start + period - 1..]);
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
