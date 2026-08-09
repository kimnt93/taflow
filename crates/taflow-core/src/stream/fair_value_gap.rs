//! Batch implementation for `fair_value_gap`.

use super::operator_states::*;
use super::*;
use crate::error::{TaError, TaResult};

/// Computes the causal fair value gap series.
/// Parameters: aligned input slices followed by indicator parameters.
/// Returns: an aligned series, with NaN during warm-up, or a parameter error.
pub fn fair_value_gap(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> {
    if open.len() != high.len() || high.len() != low.len() || low.len() != close.len() {
        return Err(TaError::LengthMismatch {
            expected: open.len(),
            got: high.len().max(low.len()).max(close.len()),
        });
    }
    let mut state = FairValueGap::new();
    let mut signal = Vec::with_capacity(open.len());
    let mut top = Vec::with_capacity(open.len());
    let mut bottom = Vec::with_capacity(open.len());
    let mut mitigated = Vec::with_capacity(open.len());
    for (((&open, &high), &low), &close) in open.iter().zip(high).zip(low).zip(close) {
        let value = state
            .append(open, high, low, close)
            .expect("FVG always emits an aligned value");
        signal.push(value.signal);
        top.push(value.top);
        bottom.push(value.bottom);
        mitigated.push(value.mitigated);
    }
    Ok((signal, top, bottom, mitigated))
}
use super::operator_states::*;
use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

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
        // smartmoneyconcepts starts looking for mitigation one bar after the
        // causal detection bar.  Scan existing zones before adding a newly
        // detected one so a gap cannot mitigate itself on its birth bar.
        let mut mitigated = f64::NAN;
        self.zones.retain(|zone| {
            let filled = (zone.direction > 0.0 && low <= zone.top)
                || (zone.direction < 0.0 && high >= zone.bottom);
            if filled {
                mitigated = zone.direction;
            }
            !filled
        });
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
