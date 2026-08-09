//! Incremental Mat Hold candlestick recognition (CDLMATHOLD).
use super::pattern::*;
use crate::error::TaResult;
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    o: f64,
    h: f64,
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
/// Stateful CandleMatHold candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleMatHold {
    candles: VecDeque<Candle>,
    body_sum: [f64; 5],
    value: Option<i32>,
}
impl Default for CandleMatHold {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleMatHold {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(14),
            body_sum: [0.0; 5],
            value: None,
        }
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, o: f64, h: f64, _l: f64, c: f64) -> Option<i32> {
        let cur = Candle { o, h, c };
        // Deque holds bars i-14..=i-1; bar j maps to index 14 - (i - j).
        let value = if self.candles.len() == 14 {
            let a = self.candles[10]; // bar i-4
            let b = self.candles[11];
            let cnd = self.candles[12];
            let d = self.candles[13];
            let long = ca_realbody_scalar(BODY_LONG, self.body_sum[4], a.o, a.c);
            let short0 = ca_realbody_scalar(BODY_SHORT, self.body_sum[3], b.o, b.c);
            let short1 = ca_realbody_scalar(BODY_SHORT, self.body_sum[2], cnd.o, cnd.c);
            let short2 = ca_realbody_scalar(BODY_SHORT, self.body_sum[1], d.o, d.c);
            // Slide sums exactly like the batch loop: sum += cr(bar) - cr(bar - 10).
            self.body_sum[4] += cr_realbody_scalar(a.o, a.c)
                - cr_realbody_scalar(self.candles[0].o, self.candles[0].c);
            self.body_sum[3] += cr_realbody_scalar(b.o, b.c)
                - cr_realbody_scalar(self.candles[1].o, self.candles[1].c);
            self.body_sum[2] += cr_realbody_scalar(cnd.o, cnd.c)
                - cr_realbody_scalar(self.candles[2].o, self.candles[2].c);
            self.body_sum[1] += cr_realbody_scalar(d.o, d.c)
                - cr_realbody_scalar(self.candles[3].o, self.candles[3].c);
            Some(
                (a.body() > long
                    && b.body() < short0
                    && cnd.body() < short1
                    && d.body() < short2
                    && a.color() == 1
                    && b.color() == -1
                    && cur.color() == 1
                    && b.o.min(b.c) > a.o.max(a.c)
                    && cnd.o.min(cnd.c) < a.c
                    && d.o.min(d.c) < a.c
                    && cnd.o.min(cnd.c) > a.c - a.body() * 0.5
                    && d.o.min(d.c) > a.c - a.body() * 0.5
                    && cnd.o.max(cnd.c) < b.o
                    && d.o.max(d.c) < cnd.o.max(cnd.c)
                    && cur.o > d.c
                    && cur.c > b.h.max(cnd.h).max(d.h)) as i32
                    * 100,
            )
        } else {
            // Warm-up: seed the sums exactly like the batch prologue.
            let i = self.candles.len();
            if i < 10 {
                self.body_sum[4] += cr_realbody_scalar(o, c);
            }
            if (1..11).contains(&i) {
                self.body_sum[3] += cr_realbody_scalar(o, c);
            }
            if (2..12).contains(&i) {
                self.body_sum[2] += cr_realbody_scalar(o, c);
            }
            if (3..13).contains(&i) {
                self.body_sum[1] += cr_realbody_scalar(o, c);
            }
            None
        };
        if self.candles.len() == 14 {
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
        self.body_sum = [0.0; 5];
        self.value = None;
    }
}
