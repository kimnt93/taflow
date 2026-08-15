//! Incremental Belt Hold candlestick recognition (CDLBELTHOLD).
use crate::error::TaResult;
use crate::stream::pattern::*;
/// Stateful CandleBeltHold candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleBeltHold {
    bodies: [f64; 10],
    ranges: [f64; 10],
    head: usize,
    len: usize,
    bs: f64,
    rs: f64,
    value: Option<i32>,
}
impl Default for CandleBeltHold {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleBeltHold {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            bodies: [0.0; 10],
            ranges: [0.0; 10],
            head: 0,
            len: 0,
            bs: 0.,
            rs: 0.,
            value: None,
        }
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, o: f64, h: f64, l: f64, c: f64) -> Option<i32> {
        let body = (c - o).abs();
        let range = h - l;
        let upper = h - o.max(c);
        let lower = o.min(c) - l;
        let v = if self.len == 10 {
            let long = body > self.bs / 10.0;
            let lim = ca_highlow_scalar(SHADOW_VERY_SHORT, self.rs, h, l);
            Some(if long && c >= o && lower < lim {
                100
            } else if long && c < o && upper < lim {
                -100
            } else {
                0
            })
        } else {
            None
        };
        if self.len == 10 {
            self.bs += body - self.bodies[self.head];
            self.rs += range - self.ranges[self.head];
            self.bodies[self.head] = body;
            self.ranges[self.head] = range;
            self.head = (self.head + 1) % 10;
        } else {
            let slot = (self.head + self.len) % 10;
            self.bs += body;
            self.rs += range;
            self.bodies[slot] = body;
            self.ranges[slot] = range;
            self.len += 1;
        }
        self.value = v;
        v
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
        let mut bs = (0..LOOKBACK).fold(0.0, |sum, i| sum + body(i));
        let mut rs = (0..LOOKBACK).fold(0.0, |sum, i| sum + range(i));
        let start = output.len();
        output.resize(start + len, 0);
        for i in LOOKBACK..len {
            let current_body = body(i);
            let lim = ca_highlow_scalar(SHADOW_VERY_SHORT, rs, high[i], low[i]);
            output[start + i] = if current_body > bs / 10.0
                && close[i] >= open[i]
                && open[i].min(close[i]) - low[i] < lim
            {
                100
            } else if current_body > bs / 10.0
                && close[i] < open[i]
                && high[i] - open[i].max(close[i]) < lim
            {
                -100
            } else {
                0
            };
            bs += current_body - body(i - LOOKBACK);
            rs += range(i) - range(i - LOOKBACK);
        }
        self.bs = bs;
        self.rs = rs;
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
        self.bs = 0.0;
        self.rs = 0.0;
        self.value = None;
    }
}
