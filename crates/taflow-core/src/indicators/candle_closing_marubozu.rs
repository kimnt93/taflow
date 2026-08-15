//! Incremental Closing Marubozu candlestick recognition (CDLCLOSINGMARUBOZU).

use crate::error::TaResult;
use crate::stream::pattern::*;
/// Incremental CDLCLOSINGMARUBOZU state using TA-Lib's rolling body and range averages.
/// Persistent Rust state or aligned output type for `CandleClosingMarubozu`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleClosingMarubozu {
    bodies: [f64; 10],
    body_sum: f64,
    ranges: [f64; 10],
    head: usize,
    len: usize,
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
            bodies: [0.0; 10],
            body_sum: 0.0,
            ranges: [0.0; 10],
            head: 0,
            len: 0,
            range_sum: 0.0,
            value: None,
        }
    }
    /// Appends OHLC data and returns a signed closing-marubozu signal after warmup.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let body = (close - open).abs();
        let range = high - low;
        let output = if self.len == 10 {
            let long_body = body > self.body_sum / 10.0;
            let threshold = ca_highlow_scalar(SHADOW_VERY_SHORT, self.range_sum, high, low);
            let bull = long_body && close >= open && high - open.max(close) < threshold;
            let bear = long_body && close < open && open.min(close) - low < threshold;
            Some((bull as i32) * 100 - (bear as i32) * 100)
        } else {
            None
        };
        if self.len == 10 {
            self.body_sum += body - self.bodies[self.head];
            self.range_sum += range - self.ranges[self.head];
            self.bodies[self.head] = body;
            self.ranges[self.head] = range;
            self.head = (self.head + 1) % 10;
        } else {
            let slot = (self.head + self.len) % 10;
            self.body_sum += body;
            self.range_sum += range;
            self.bodies[slot] = body;
            self.ranges[slot] = range;
            self.len += 1;
        }
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
        if self.len != 0 || len <= LOOKBACK {
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
        for (slot, i) in (len - LOOKBACK..len).enumerate() {
            self.bodies[slot] = body(i);
            self.ranges[slot] = range(i);
        }
        self.head = 0;
        self.len = LOOKBACK;
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
        self.head = 0;
        self.len = 0;
        self.body_sum = 0.0;
        self.range_sum = 0.0;
        self.value = None;
    }
}
