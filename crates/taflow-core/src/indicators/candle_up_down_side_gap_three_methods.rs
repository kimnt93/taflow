//! Incremental Upside/Downside Gap Three Methods recognition (CDLXSIDEGAP3METHODS).
use crate::error::TaResult;
use crate::stream::pattern::*;
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    open: f64,
    close: f64,
}
/// Incremental CDLXSIDEGAP3METHODS state.
/// Persistent Rust state or aligned output type for `CandleUpDownSideGapThreeMethods`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleUpDownSideGapThreeMethods {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleUpDownSideGapThreeMethods {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleUpDownSideGapThreeMethods {
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
            let first_color = if first.close >= first.open { 1 } else { -1 };
            let second_color = if second.close >= second.open { 1 } else { -1 };
            let current_color = if close >= open { 1 } else { -1 };
            let base = first_color == second_color
                && current_color != first_color
                && open > second.open.min(second.close)
                && open < second.open.max(second.close)
                && close > first.open.min(first.close)
                && close < first.open.max(first.close);
            let bull = base
                && first_color == 1
                && second.open.min(second.close) > first.open.max(first.close);
            let bear = base
                && first_color == -1
                && second.open.max(second.close) < first.open.min(first.close);
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
            let first = Candle {
                open: opens[0],
                close: closes[0],
            };
            let second = Candle {
                open: opens[1],
                close: closes[1],
            };
            let current_open = opens[2];
            let current_close = closes[2];
            let first_color = if first.close >= first.open { 1 } else { -1 };
            let second_color = if second.close >= second.open { 1 } else { -1 };
            let current_color = if current_close >= current_open { 1 } else { -1 };
            let base = first_color == second_color
                && current_color != first_color
                && current_open > second.open.min(second.close)
                && current_open < second.open.max(second.close)
                && current_close > first.open.min(first.close)
                && current_close < first.open.max(first.close);
            let bull = base
                && first_color == 1
                && second.open.min(second.close) > first.open.max(first.close);
            let bear = base
                && first_color == -1
                && second.open.max(second.close) < first.open.min(first.close);
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
