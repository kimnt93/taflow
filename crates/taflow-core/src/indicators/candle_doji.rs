//! Incremental Doji candlestick recognition (CDLDOJI).

use std::collections::VecDeque;

use crate::error::TaResult;
use crate::stream::pattern::*;
/// Incremental CDLDOJI state using TA-Lib's ten-bar High-Low average.
/// Persistent Rust state or aligned output type for `CandleDoji`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleDoji {
    ranges: VecDeque<f64>,
    sum: f64,
    value: Option<i32>,
}
impl Default for CandleDoji {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleDoji {
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
    /// Appends OHLC data and returns +100 for a doji after the ten-bar warmup.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        if self.ranges.len() < 10 {
            self.ranges.push_back(high - low);
            self.sum += high - low;
            return None;
        }
        let threshold = ca_highlow_scalar(BODY_DOJI, self.sum, high, low);
        self.value = Some(if (close - open).abs() <= threshold {
            100
        } else {
            0
        });
        self.sum += high - low - self.ranges.pop_front().expect("window is full");
        self.ranges.push_back(high - low);
        self.value
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
        if !self.ranges.is_empty() || len <= LOOKBACK {
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
            .fold(0.0, |sum, (&h, &l)| sum + h - l);
        for ((((opens, highs), lows), closes), out) in open
            .windows(LOOKBACK + 1)
            .zip(high.windows(LOOKBACK + 1))
            .zip(low.windows(LOOKBACK + 1))
            .zip(close.windows(LOOKBACK + 1))
            .zip(output[start + LOOKBACK..].iter_mut())
        {
            let range = highs[LOOKBACK] - lows[LOOKBACK];
            let threshold = ca_highlow_scalar(BODY_DOJI, sum, highs[LOOKBACK], lows[LOOKBACK]);
            *out = ((closes[LOOKBACK] - opens[LOOKBACK]).abs() <= threshold) as i32 * 100;
            sum += range - (highs[0] - lows[0]);
        }
        self.sum = sum;
        self.ranges
            .extend((len - LOOKBACK..len).map(|i| high[i] - low[i]));
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
        self.ranges.clear();
        self.sum = 0.0;
        self.value = None;
    }
}
