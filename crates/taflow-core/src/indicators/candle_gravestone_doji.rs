//! Incremental Gravestone Doji candlestick recognition (CDLGRAVESTONEDOJI).

use std::collections::VecDeque;

use crate::error::TaResult;
use crate::stream::pattern::*;
/// Incremental CDLGRAVESTONEDOJI state using TA-Lib's ten-bar range average.
/// Persistent Rust state or aligned output type for `CandleGravestoneDoji`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleGravestoneDoji {
    ranges: VecDeque<f64>,
    sum: f64,
    value: Option<i32>,
}
impl Default for CandleGravestoneDoji {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleGravestoneDoji {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            ranges: VecDeque::with_capacity(10),
            sum: 0.0,
            value: None,
        }
    }
    /// Appends OHLC data and returns +100 for a gravestone doji after warmup.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let range = high - low;
        let body = (close - open).abs();
        let output = if self.ranges.len() == 10 {
            let limit = ca_highlow_scalar(BODY_DOJI, self.sum, high, low);
            Some(
                (body <= limit && open.min(close) - low < limit && high - open.max(close) > limit)
                    as i32
                    * 100,
            )
        } else {
            None
        };
        if self.ranges.len() == 10 {
            // Slide exactly like the batch loop: sum += cr(new) - cr(evicted).
            let old = self.ranges.pop_front().expect("window is full");
            self.sum += range - old;
        } else {
            self.sum += range;
        }
        self.ranges.push_back(range);
        self.value = output;
        output
    }
    /// Bulk-append aligned OHLC slices, pushing one score per bar into `output`.
    ///
    /// From a pristine state this runs the incremental batch kernel over the
    /// slices and then replays only the trailing bars through `append` to
    /// rebuild the window-bounded streaming state; the replayed scores are
    /// discarded because the batch pass already emitted them. A non-pristine
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
        if !self.ranges.is_empty() {
            for i in 0..len {
                output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
            }
            return Ok(());
        }
        for i in 0..len {
            output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
        }
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
        self.ranges.clear();
        self.sum = 0.0;
        self.value = None;
    }
}
