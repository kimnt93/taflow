//! Incremental Spinning Top candlestick recognition (CDLSPINNINGTOP).

use crate::error::TaResult;
use crate::stream::pattern::*;
/// Incremental CDLSPINNINGTOP state using TA-Lib's ten-bar short-body average.
/// Persistent Rust state or aligned output type for `CandleSpinningTop`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleSpinningTop {
    bodies: [f64; 10],
    head: usize,
    len: usize,
    sum: f64,
    value: Option<i32>,
}
impl Default for CandleSpinningTop {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleSpinningTop {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            bodies: [0.0; 10],
            head: 0,
            len: 0,
            sum: 0.0,
            value: None,
        }
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let body = (close - open).abs();
        let value = if self.len == 10 {
            Some(
                (body < self.sum / 10.0
                    && high - open.max(close) > body
                    && open.min(close) - low > body) as i32
                    * if close >= open { 100 } else { -100 },
            )
        } else {
            None
        };
        if self.len == 10 {
            // Slide exactly like the batch loop: sum += cr(new) - cr(evicted).
            let old = self.bodies[self.head];
            self.sum += body - old;
            self.bodies[self.head] = body;
            self.head = (self.head + 1) % 10;
        } else {
            self.sum += body;
            self.bodies[(self.head + self.len) % 10] = body;
            self.len += 1;
        }
        self.value = value;
        value
    }
    /// Bulk-append aligned OHLC slices, pushing one score per bar into `output`.
    ///
    /// From a pristine state this runs directly over the slices and rebuilds
    /// the bounded body window once after the loop. A non-pristine state falls
    /// back to the per-bar loop. Either route is bit-identical to calling
    /// `append` once per bar (warm-up `None` becomes `0`, matching the batch
    /// prologue).
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
        const LOOKBACK: usize = 10;
        if self.len != 0 || len <= LOOKBACK {
            output.reserve(len);
            for i in 0..len {
                output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
            }
            return Ok(());
        }

        let start = output.len();
        output.resize(start + len, 0);
        let mut sum = open[..LOOKBACK]
            .iter()
            .zip(&close[..LOOKBACK])
            .fold(0.0, |sum, (&open, &close)| sum + (close - open).abs());
        for i in LOOKBACK..len {
            let body = (close[i] - open[i]).abs();
            output[start + i] = ((body < sum / 10.0
                && high[i] - open[i].max(close[i]) > body
                && open[i].min(close[i]) - low[i] > body) as i32)
                * if close[i] >= open[i] { 100 } else { -100 };
            sum += body - (close[i - LOOKBACK] - open[i - LOOKBACK]).abs();
        }

        for (slot, (&open, &close)) in open[len - LOOKBACK..]
            .iter()
            .zip(&close[len - LOOKBACK..])
            .enumerate()
        {
            self.bodies[slot] = (close - open).abs();
        }
        self.head = 0;
        self.len = LOOKBACK;
        self.sum = sum;
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
        self.head = 0;
        self.len = 0;
        self.sum = 0.0;
        self.value = None;
    }
}
