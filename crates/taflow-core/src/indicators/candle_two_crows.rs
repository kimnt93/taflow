//! Incremental Two Crows candlestick recognition (CDL2CROWS).

use crate::error::TaResult;
use crate::stream::pattern::*;
#[derive(Clone, Copy, Default)]
struct Candle {
    open: f64,
    close: f64,
    body: f64,
}
/// Incremental CDL2CROWS state.
/// Persistent Rust state or aligned output type for `CandleTwoCrows`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleTwoCrows {
    candles: [Candle; 2],
    candle_head: usize,
    candle_len: usize,
    bodies: [f64; 10],
    body_head: usize,
    body_len: usize,
    sum: f64,
    value: Option<i32>,
}
impl Default for CandleTwoCrows {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleTwoCrows {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: [Candle::default(); 2],
            candle_head: 0,
            candle_len: 0,
            bodies: [0.0; 10],
            body_head: 0,
            body_len: 0,
            sum: 0.0,
            value: None,
        }
    }
    fn push_body(&mut self, value: f64) {
        if self.body_len == 10 {
            self.sum += value - self.bodies[self.body_head];
            self.bodies[self.body_head] = value;
            self.body_head = (self.body_head + 1) % 10;
        } else {
            self.sum += value;
            self.bodies[(self.body_head + self.body_len) % 10] = value;
            self.body_len += 1;
        }
    }
    /// Appends OHLC data and returns -100 for a two-crows pattern after warmup.
    pub fn append(&mut self, open: f64, _high: f64, _low: f64, close: f64) -> Option<i32> {
        let current = Candle {
            open,
            close,
            body: (close - open).abs(),
        };
        let output = if self.body_len == 10 && self.candle_len == 2 {
            let first = self.candles[self.candle_head];
            let second = self.candles[(self.candle_head + 1) % 2];
            let pattern = first.close >= first.open
                && first.body > self.sum / 10.0
                && second.close < second.open
                && second.open.min(second.close) > first.open.max(first.close)
                && close < open
                && open < second.open
                && open > second.close
                && close > first.open
                && close < first.close;
            Some(-(pattern as i32) * 100)
        } else {
            None
        };
        if self.candle_len == 2 {
            self.push_body(self.candles[self.candle_head].body);
            self.candles[self.candle_head] = current;
            self.candle_head = (self.candle_head + 1) % 2;
        } else {
            self.candles[(self.candle_head + self.candle_len) % 2] = current;
            self.candle_len += 1;
        }
        self.value = output;
        output
    }
    /// Bulk-append aligned OHLC slices, pushing one score per bar into `output`.
    ///
    /// From a pristine state this runs directly over the slices and rebuilds
    /// the bounded candle and body rings once after the loop. A non-pristine
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
        const LOOKBACK: usize = 12;
        if self.candle_len != 0 || len <= LOOKBACK {
            output.reserve(len);
            for i in 0..len {
                output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
            }
            return Ok(());
        }

        let start = output.len();
        output.resize(start + len, 0);
        let body = |i: usize| (close[i] - open[i]).abs();
        let mut sum = (0..10).fold(0.0, |sum, i| sum + body(i));
        for i in LOOKBACK..len {
            let first = i - 2;
            let second = i - 1;
            let pattern = close[first] >= open[first]
                && body(first) > sum / 10.0
                && close[second] < open[second]
                && open[second].min(close[second]) > open[first].max(close[first])
                && close[i] < open[i]
                && open[i] < open[second]
                && open[i] > close[second]
                && close[i] > open[first]
                && close[i] < close[first];
            output[start + i] = -(pattern as i32) * 100;
            sum += body(first) - body(i - 12);
        }

        self.sum = sum;
        for (slot, i) in (len - 12..len - 2).enumerate() {
            self.bodies[slot] = body(i);
        }
        self.body_head = 0;
        self.body_len = 10;
        for (slot, i) in (len - 2..len).enumerate() {
            self.candles[slot] = Candle {
                open: open[i],
                close: close[i],
                body: body(i),
            };
        }
        self.candle_head = 0;
        self.candle_len = 2;
        self.value = Some(output[start + len - 1]);
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
        self.candle_head = 0;
        self.candle_len = 0;
        self.body_head = 0;
        self.body_len = 0;
        self.sum = 0.0;
        self.value = None;
    }
}
