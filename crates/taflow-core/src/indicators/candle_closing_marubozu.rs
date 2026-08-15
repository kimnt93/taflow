//! Incremental Closing Marubozu candlestick recognition (CDLCLOSINGMARUBOZU).

use std::collections::VecDeque;

use crate::error::TaResult;
use crate::stream::pattern::*;
/// Incremental CDLCLOSINGMARUBOZU state using TA-Lib's rolling body and range averages.
/// Persistent Rust state or aligned output type for `CandleClosingMarubozu`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleClosingMarubozu {
    bodies: VecDeque<f64>,
    body_sum: f64,
    ranges: VecDeque<f64>,
    range_sum: f64,
    value: Option<i32>,
}

impl Default for CandleClosingMarubozu {
    fn default() -> Self {
        Self::new()
    }
}

impl CandleClosingMarubozu {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            bodies: VecDeque::with_capacity(10),
            body_sum: 0.0,
            ranges: VecDeque::with_capacity(10),
            range_sum: 0.0,
            value: None,
        }
    }
    fn push(window: &mut VecDeque<f64>, sum: &mut f64, value: f64) {
        if window.len() == 10 {
            // Slide exactly like the batch loop: sum += cr(new) - cr(evicted).
            let old = window.pop_front().expect("window is full");
            *sum += value - old;
        } else {
            *sum += value;
        }
        window.push_back(value);
    }
    /// Appends OHLC data and returns a signed closing-marubozu signal after warmup.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let body = (close - open).abs();
        let range = high - low;
        let output = if self.bodies.len() == 10 && self.ranges.len() == 10 {
            let long_body = body > self.body_sum / 10.0;
            let threshold = ca_highlow_scalar(SHADOW_VERY_SHORT, self.range_sum, high, low);
            let bull = long_body && close >= open && high - open.max(close) < threshold;
            let bear = long_body && close < open && open.min(close) - low < threshold;
            Some((bull as i32) * 100 - (bear as i32) * 100)
        } else {
            None
        };
        Self::push(&mut self.bodies, &mut self.body_sum, body);
        Self::push(&mut self.ranges, &mut self.range_sum, range);
        self.value = output;
        output
    }
    /// Bulk-append aligned OHLC slices, pushing one score per bar into `output`.
    ///
    /// From a pristine state this runs a direct slice kernel and reconstructs
    /// the bounded trailing state without replaying the input. A non-pristine
    /// state falls back to the per-bar loop. Either route is bit-identical to
    /// calling `append` once per bar (warm-up `None` becomes `0`, matching the
    /// batch prologue).
    ///
    /// # Parameters
    ///
    /// * `open`, `high`, `low`, `close` - Equal-length chronological OHLC series.
    /// * `output` - Destination the aligned scores are appended to.
    ///
    /// # Returns
    ///
    /// `Ok(())`, or a validation error when the inputs are not aligned.
    pub fn extend_slices_into(
        &mut self,
        open: &[f64],
        high: &[f64],
        low: &[f64],
        close: &[f64],
        output: &mut Vec<i32>,
    ) -> TaResult<()> {
        let len = validate_ohlc(open, high, low, close)?;
        output.reserve(len);
        const LOOKBACK: usize = 10;
        if !self.bodies.is_empty() || len <= LOOKBACK {
            for i in 0..len {
                output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
            }
            return Ok(());
        }
        let body = |i: usize| (close[i] - open[i]).abs();
        let range = |i: usize| high[i] - low[i];
        let mut body_sum = (0..LOOKBACK).fold(0.0, |sum, i| sum + body(i));
        let mut range_sum = (0..LOOKBACK).fold(0.0, |sum, i| sum + range(i));
        let start = output.len();
        output.resize(start + len, 0);
        for i in LOOKBACK..len {
            let current_body = body(i);
            let threshold = ca_highlow_scalar(SHADOW_VERY_SHORT, range_sum, high[i], low[i]);
            let bull = current_body > body_sum / 10.0
                && close[i] >= open[i]
                && high[i] - open[i].max(close[i]) < threshold;
            let bear = current_body > body_sum / 10.0
                && close[i] < open[i]
                && open[i].min(close[i]) - low[i] < threshold;
            output[start + i] = (bull as i32) * 100 - (bear as i32) * 100;
            body_sum += current_body - body(i - LOOKBACK);
            range_sum += range(i) - range(i - LOOKBACK);
        }
        self.body_sum = body_sum;
        self.range_sum = range_sum;
        self.bodies.extend((len - LOOKBACK..len).map(body));
        self.ranges.extend((len - LOOKBACK..len).map(range));
        self.value = output.last().copied();
        Ok(())
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<i32> {
        self.value
    }
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.bodies.clear();
        self.body_sum = 0.0;
        self.ranges.clear();
        self.range_sum = 0.0;
        self.value = None;
    }
}
