//! Incremental Stalled Pattern candlestick recognition (CDLSTALLEDPATTERN).
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
    fn upper(self) -> f64 {
        self.h - self.o.max(self.c)
    }
    fn color(self) -> i32 {
        if self.c >= self.o {
            1
        } else {
            -1
        }
    }
}
/// Stateful CandleStalledPattern candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleStalledPattern {
    candles: VecDeque<Candle>,
    body_long_sum: [f64; 2],
    body_short_sum: f64,
    shadow_sum: f64,
    near_sum: [f64; 2],
    value: Option<i32>,
}
impl Default for CandleStalledPattern {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleStalledPattern {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(12),
            body_long_sum: [0.0; 2],
            body_short_sum: 0.0,
            shadow_sum: 0.0,
            near_sum: [0.0; 2],
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
            let long0 = ca_realbody_scalar(BODY_LONG, self.body_long_sum[0], a.o, a.c);
            let long1 = ca_realbody_scalar(BODY_LONG, self.body_long_sum[1], b.o, b.c);
            let short = ca_realbody_scalar(BODY_SHORT, self.body_short_sum, o, c);
            let shadow = ca_highlow_scalar(SHADOW_VERY_SHORT, self.shadow_sum, b.h, b.l);
            let near0 = ca_highlow_scalar(NEAR, self.near_sum[0], a.h, a.l);
            let near1 = ca_highlow_scalar(NEAR, self.near_sum[1], b.h, b.l);
            // Slide sums exactly like the batch loop: sum += cr(bar) - cr(bar - period).
            self.body_long_sum[0] += cr_realbody_scalar(a.o, a.c)
                - cr_realbody_scalar(self.candles[0].o, self.candles[0].c);
            self.body_long_sum[1] += cr_realbody_scalar(b.o, b.c)
                - cr_realbody_scalar(self.candles[1].o, self.candles[1].c);
            self.body_short_sum +=
                cr_realbody_scalar(o, c) - cr_realbody_scalar(self.candles[2].o, self.candles[2].c);
            self.shadow_sum += cr_highlow_scalar(b.h, b.l)
                - cr_highlow_scalar(self.candles[1].h, self.candles[1].l);
            self.near_sum[0] += cr_highlow_scalar(a.h, a.l)
                - cr_highlow_scalar(self.candles[5].h, self.candles[5].l);
            self.near_sum[1] += cr_highlow_scalar(b.h, b.l)
                - cr_highlow_scalar(self.candles[6].h, self.candles[6].l);
            Some(
                (a.color() == 1
                    && b.color() == 1
                    && cur.color() == 1
                    && b.c > a.c
                    && cur.c > b.c
                    && a.body() > long0
                    && b.body() > long1
                    && b.upper() < shadow
                    && b.o > a.o
                    && b.o <= a.c + near0
                    && cur.body() < short
                    && cur.o >= b.c - cur.body() - near1) as i32
                    * -100,
            )
        } else {
            // Warm-up: seed the sums exactly like the batch prologue.
            let i = self.candles.len();
            if i < 10 {
                self.body_long_sum[0] += cr_realbody_scalar(o, c);
            }
            if (1..11).contains(&i) {
                self.body_long_sum[1] += cr_realbody_scalar(o, c);
                self.shadow_sum += cr_highlow_scalar(h, l);
            }
            if (2..12).contains(&i) {
                self.body_short_sum += cr_realbody_scalar(o, c);
            }
            if (5..10).contains(&i) {
                self.near_sum[0] += cr_highlow_scalar(h, l);
            }
            if (6..11).contains(&i) {
                self.near_sum[1] += cr_highlow_scalar(h, l);
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
        self.body_long_sum = [0.0; 2];
        self.body_short_sum = 0.0;
        self.shadow_sum = 0.0;
        self.near_sum = [0.0; 2];
        self.value = None;
    }
}
