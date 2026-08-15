//! Incremental Three Outside pattern recognition (CDL3OUTSIDE).
use crate::error::TaResult;
use crate::stream::pattern::*;
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    open: f64,
    close: f64,
}
/// Incremental CDL3OUTSIDE state.
/// Persistent Rust state or aligned output type for `CandleThreeOutside`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleThreeOutside {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleThreeOutside {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleThreeOutside {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(2),
            value: None,
        }
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, open: f64, _high: f64, _low: f64, close: f64) -> Option<i32> {
        let output = if self.candles.len() == 2 {
            let first = self.candles[0];
            let second = self.candles[1];
            let bull = first.close < first.open
                && second.close >= first.open
                && second.open <= first.close
                && close > second.close;
            let bear = first.close >= first.open
                && second.close < second.open
                && second.open >= first.close
                && second.close <= first.open
                && close < second.close;
            Some((bull as i32) * 100 - (bear as i32) * 100)
        } else {
            None
        };
        if self.candles.len() == 2 {
            self.candles.pop_front();
        }
        self.candles.push_back(Candle { open, close });
        self.value = output;
        output
    }
    /// Bulk-append aligned OHLC slices, pushing one score per bar into `output`.
    ///
    /// From a pristine state this runs a direct slice kernel and reconstructs
    /// the two-bar trailing state without replaying the input. A non-pristine
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
        const LOOKBACK: usize = 2;
        if !self.candles.is_empty() || len <= LOOKBACK {
            for i in 0..len {
                output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
            }
            return Ok(());
        }
        let start = output.len();
        output.resize(start + len, 0);
        for ((opens, closes), out) in open
            .windows(3)
            .zip(close.windows(3))
            .zip(output[start + LOOKBACK..].iter_mut())
        {
            let bull = closes[0] < opens[0]
                && closes[1] >= opens[0]
                && opens[1] <= closes[0]
                && closes[2] > closes[1];
            let bear = closes[0] >= opens[0]
                && closes[1] < opens[1]
                && opens[1] >= closes[0]
                && closes[1] <= opens[0]
                && closes[2] < closes[1];
            *out = (bull as i32) * 100 - (bear as i32) * 100;
        }
        self.candles.extend((len - LOOKBACK..len).map(|i| Candle {
            open: open[i],
            close: close[i],
        }));
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
        self.candles.clear();
        self.value = None;
    }
}
