//! Incremental Advance Block candlestick recognition (CDLADVANCEBLOCK).
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
/// Stateful CandleAdvanceBlock candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleAdvanceBlock {
    candles: VecDeque<Candle>,
    body_long_sum: f64,
    shadow_short_sum: [f64; 3],
    near_sum: [f64; 2],
    far_sum: [f64; 2],
    value: Option<i32>,
}
impl Default for CandleAdvanceBlock {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleAdvanceBlock {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(12),
            body_long_sum: 0.0,
            shadow_short_sum: [0.0; 3],
            near_sum: [0.0; 2],
            far_sum: [0.0; 2],
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
            let long = ca_realbody_scalar(BODY_LONG, self.body_long_sum, a.o, a.c);
            let shadow_a =
                ca_shadows_scalar(SHADOW_SHORT, self.shadow_short_sum[0], a.o, a.h, a.l, a.c);
            let shadow_b =
                ca_shadows_scalar(SHADOW_SHORT, self.shadow_short_sum[1], b.o, b.h, b.l, b.c);
            let shadow_cur = ca_shadows_scalar(SHADOW_SHORT, self.shadow_short_sum[2], o, h, l, c);
            let near_a = ca_highlow_scalar(NEAR, self.near_sum[0], a.h, a.l);
            let near_b = ca_highlow_scalar(NEAR, self.near_sum[1], b.h, b.l);
            let far_a = ca_highlow_scalar(FAR, self.far_sum[0], a.h, a.l);
            let far_b = ca_highlow_scalar(FAR, self.far_sum[1], b.h, b.l);
            let base = a.color() == 1
                && b.color() == 1
                && cur.color() == 1
                && b.c > a.c
                && cur.c > b.c
                && b.o > a.o
                && b.o <= a.c + near_a
                && cur.o > b.o
                && cur.o <= b.c + near_b
                && a.body() > long
                && a.upper() < shadow_a;
            let weakness = base
                && ((b.body() < a.body() - far_a && cur.body() < b.body() + near_b)
                    || cur.body() < b.body() - far_b
                    || (cur.body() < b.body()
                        && b.body() < a.body()
                        && (cur.upper() > shadow_cur || b.upper() > shadow_b))
                    || (cur.body() < b.body()
                        && cur.upper() > ca_realbody_scalar(SHADOW_LONG, 0.0, cur.o, cur.c)));
            // Slide sums exactly like the batch loop: sum += cr(bar) - cr(bar - period).
            self.shadow_short_sum[0] += cr_shadows_scalar(a.o, a.h, a.l, a.c)
                - cr_shadows_scalar(
                    self.candles[0].o,
                    self.candles[0].h,
                    self.candles[0].l,
                    self.candles[0].c,
                );
            self.shadow_short_sum[1] += cr_shadows_scalar(b.o, b.h, b.l, b.c)
                - cr_shadows_scalar(
                    self.candles[1].o,
                    self.candles[1].h,
                    self.candles[1].l,
                    self.candles[1].c,
                );
            self.shadow_short_sum[2] += cr_shadows_scalar(o, h, l, c)
                - cr_shadows_scalar(
                    self.candles[2].o,
                    self.candles[2].h,
                    self.candles[2].l,
                    self.candles[2].c,
                );
            self.near_sum[0] += cr_highlow_scalar(a.h, a.l)
                - cr_highlow_scalar(self.candles[5].h, self.candles[5].l);
            self.near_sum[1] += cr_highlow_scalar(b.h, b.l)
                - cr_highlow_scalar(self.candles[6].h, self.candles[6].l);
            self.far_sum[0] += cr_highlow_scalar(a.h, a.l)
                - cr_highlow_scalar(self.candles[5].h, self.candles[5].l);
            self.far_sum[1] += cr_highlow_scalar(b.h, b.l)
                - cr_highlow_scalar(self.candles[6].h, self.candles[6].l);
            self.body_long_sum += cr_realbody_scalar(a.o, a.c)
                - cr_realbody_scalar(self.candles[0].o, self.candles[0].c);
            Some(weakness as i32 * -100)
        } else {
            // Warm-up: seed the sums exactly like the batch prologue.
            let i = self.candles.len();
            if i < 10 {
                self.shadow_short_sum[0] += cr_shadows_scalar(o, h, l, c);
                self.body_long_sum += cr_realbody_scalar(o, c);
            }
            if (1..11).contains(&i) {
                self.shadow_short_sum[1] += cr_shadows_scalar(o, h, l, c);
            }
            if (2..12).contains(&i) {
                self.shadow_short_sum[2] += cr_shadows_scalar(o, h, l, c);
            }
            if (5..10).contains(&i) {
                self.near_sum[0] += cr_highlow_scalar(h, l);
                self.far_sum[0] += cr_highlow_scalar(h, l);
            }
            if (6..11).contains(&i) {
                self.near_sum[1] += cr_highlow_scalar(h, l);
                self.far_sum[1] += cr_highlow_scalar(h, l);
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
        self.body_long_sum = 0.0;
        self.shadow_short_sum = [0.0; 3];
        self.near_sum = [0.0; 2];
        self.far_sum = [0.0; 2];
        self.value = None;
    }
}
