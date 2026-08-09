//! Incremental Tasuki Gap candlestick recognition (CDLTASUKIGAP).
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
    fn color(self) -> i32 {
        if self.c >= self.o {
            1
        } else {
            -1
        }
    }
}
/// Stateful CandleTasukiGap candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleTasukiGap {
    candles: VecDeque<Candle>,
    near_sum: f64,
    value: Option<i32>,
}
impl Default for CandleTasukiGap {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleTasukiGap {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(7),
            near_sum: 0.0,
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
        // Deque holds bars i-7..=i-1; bar j maps to index 7 - (i - j).
        let value = if self.candles.len() == 7 {
            let a = self.candles[5]; // bar i-2
            let b = self.candles[6]; // bar i-1
            let near = ca_highlow_scalar(NEAR, self.near_sum, b.h, b.l);
            // Slide the sum exactly like the batch loop: sum += cr(bar) - cr(bar - 5).
            self.near_sum += cr_highlow_scalar(b.h, b.l)
                - cr_highlow_scalar(self.candles[1].h, self.candles[1].l);
            let c1 = b.color();
            let c0 = cur.color();
            let near_same = (b.body() - cur.body()).abs() < near;
            let bull = b.o.min(b.c) > a.o.max(a.c)
                && c1 == 1
                && c0 == -1
                && cur.o < b.c
                && cur.o > b.o
                && cur.c < b.o
                && cur.c > a.o.max(a.c)
                && near_same;
            let bear = b.o.max(b.c) < a.o.min(a.c)
                && c1 == -1
                && c0 == 1
                && cur.o < b.o
                && cur.o > b.c
                && cur.c > b.o
                && cur.c < a.o.min(a.c)
                && near_same;
            Some((bull as i32 | bear as i32) * c1 * 100)
        } else {
            // Warm-up: seed the sum exactly like the batch prologue.
            if (1..6).contains(&self.candles.len()) {
                self.near_sum += cr_highlow_scalar(h, l);
            }
            None
        };
        if self.candles.len() == 7 {
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
        self.near_sum = 0.0;
        self.value = None;
    }
}
