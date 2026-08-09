//! Incremental Tri-Star candlestick recognition (CDLTRISTAR).
use super::pattern::*;
use crate::error::TaResult;
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    o: f64,
    h: f64,
    l: f64,
    c: f64,
}
impl Candle {
    fn body(self) -> f64 {
        (self.c - self.o).abs()
    }
}
/// Stateful CandleTriStar candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleTriStar {
    candles: VecDeque<Candle>,
    body_doji_sum: f64,
    value: Option<i32>,
}
impl Default for CandleTriStar {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleTriStar {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(12),
            body_doji_sum: 0.0,
            value: None,
        }
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, o: f64, h: f64, l: f64, c: f64) -> Option<i32> {
        let cur = Candle { o, h, l, c };
        // Deque holds bars i-12..=i-1; bar j maps to index 12 - (i - j).
        let value = if self.candles.len() == 12 {
            let a = self.candles[10]; // bar i-2
            let b = self.candles[11]; // bar i-1
            let doji = ca_highlow_scalar(BODY_DOJI, self.body_doji_sum, a.h, a.l);
            // Slide the sum exactly like the batch loop: sum += cr(bar) - cr(bar - 10).
            self.body_doji_sum += cr_highlow_scalar(a.h, a.l)
                - cr_highlow_scalar(self.candles[0].h, self.candles[0].l);
            let base = a.body() <= doji && b.body() <= doji && cur.body() <= doji;
            let bear = base && b.o.min(b.c) > a.o.max(a.c) && cur.o.max(cur.c) < b.o.max(b.c);
            let bull = base && b.o.max(b.c) < a.o.min(a.c) && cur.o.min(cur.c) > b.o.min(b.c);
            Some((bull as i32) * 100 - (bear as i32) * 100)
        } else {
            // Warm-up: seed the sum exactly like the batch prologue.
            if self.candles.len() < 10 {
                self.body_doji_sum += cr_highlow_scalar(h, l);
            }
            None
        };
        if self.candles.len() == 12 {
            self.candles.pop_front();
        }
        self.candles.push_back(cur);
        self.value = value;
        value
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
        if !self.candles.is_empty() {
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
        self.candles.clear();
        self.body_doji_sum = 0.0;
        self.value = None;
    }
}
