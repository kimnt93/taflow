//! Incremental Engulfing candlestick recognition (CDLENGULFING).

use crate::error::TaResult;
use crate::stream::pattern::*;
/// Incremental CDLENGULFING state.
/// Persistent Rust state or aligned output type for `CandleEngulfing`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleEngulfing {
    previous: Option<(f64, f64)>,
    value: Option<i32>,
}
impl Default for CandleEngulfing {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleEngulfing {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            previous: None,
            value: None,
        }
    }
    /// Appends OHLC data; high and low are accepted for a uniform pattern API.
    pub fn append(&mut self, open: f64, _high: f64, _low: f64, close: f64) -> Option<i32> {
        let previous = self.previous.replace((open, close));
        let (previous_open, previous_close) = previous?;
        let bullish =
            previous_close < previous_open && close >= previous_open && open <= previous_close;
        let bearish = previous_close >= previous_open
            && close < open
            && open >= previous_close
            && close <= previous_open;
        self.value = Some((bullish as i32) * 100 - (bearish as i32) * 100);
        self.value
    }
    /// Bulk-append aligned OHLC slices, pushing one score per bar into `output`.
    ///
    /// From a pristine state this runs a direct slice kernel and reconstructs
    /// the one-bar trailing state without replaying the input. A non-pristine
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
        const LOOKBACK: usize = 1;
        if self.previous.is_some() || len <= LOOKBACK {
            for i in 0..len {
                output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
            }
            return Ok(());
        }
        let start = output.len();
        output.resize(start + len, 0);
        for ((opens, closes), out) in open
            .windows(2)
            .zip(close.windows(2))
            .zip(output[start + LOOKBACK..].iter_mut())
        {
            let bullish = closes[0] < opens[0] && closes[1] >= opens[0] && opens[1] <= closes[0];
            let bearish = closes[0] >= opens[0]
                && closes[1] < opens[1]
                && opens[1] >= closes[0]
                && closes[1] <= opens[0];
            *out = (bullish as i32) * 100 - (bearish as i32) * 100;
        }
        self.previous = Some((open[len - 1], close[len - 1]));
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
        self.previous = None;
        self.value = None;
    }
}
