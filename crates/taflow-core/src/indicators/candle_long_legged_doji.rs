//! Incremental Long-Legged Doji candlestick recognition (CDLLONGLEGGEDDOJI).

use crate::error::TaResult;
use crate::stream::pattern::*;
/// Incremental CDLLONGLEGGEDDOJI state using TA-Lib's ten-bar doji range average.
/// Persistent Rust state or aligned output type for `CandleLongLeggedDoji`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleLongLeggedDoji {
    ranges: [f64; 10],
    head: usize,
    len: usize,
    sum: f64,
    value: Option<i32>,
}
impl Default for CandleLongLeggedDoji {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleLongLeggedDoji {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            ranges: [0.0; 10],
            head: 0,
            len: 0,
            sum: 0.0,
            value: None,
        }
    }
    /// Appends OHLC data and returns +100 for a long-legged doji after warmup.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let body = (close - open).abs();
        let range = high - low;
        let output = if self.len == 10 {
            Some(
                (body <= ca_highlow_scalar(BODY_DOJI, self.sum, high, low)
                    && (open.min(close) - low > body || high - open.max(close) > body))
                    as i32
                    * 100,
            )
        } else {
            None
        };
        if self.len == 10 {
            // Slide exactly like the batch loop: sum += cr(new) - cr(evicted).
            let old = self.ranges[self.head];
            self.sum += range - old;
            self.ranges[self.head] = range;
            self.head = (self.head + 1) % 10;
        } else {
            self.sum += range;
            self.ranges[(self.head + self.len) % 10] = range;
            self.len += 1;
        }
        self.value = output;
        output
    }
    /// Bulk-append aligned OHLC slices, pushing one score per bar into `output`.
    ///
    /// From a pristine state this runs directly over the slices and rebuilds
    /// the bounded range window once after the loop. A non-pristine state falls
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
        let mut sum = high[..LOOKBACK]
            .iter()
            .zip(&low[..LOOKBACK])
            .fold(0.0, |sum, (&high, &low)| sum + high - low);
        for i in LOOKBACK..len {
            let body = (close[i] - open[i]).abs();
            output[start + i] = ((body <= ca_highlow_scalar(BODY_DOJI, sum, high[i], low[i])
                && (open[i].min(close[i]) - low[i] > body
                    || high[i] - open[i].max(close[i]) > body))
                as i32)
                * 100;
            sum += (high[i] - low[i]) - (high[i - LOOKBACK] - low[i - LOOKBACK]);
        }

        for (slot, (&high, &low)) in high[len - LOOKBACK..]
            .iter()
            .zip(&low[len - LOOKBACK..])
            .enumerate()
        {
            self.ranges[slot] = high - low;
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
