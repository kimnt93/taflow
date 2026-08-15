//! Incremental Advance Block candlestick recognition (CDLADVANCEBLOCK).
use crate::error::TaResult;
use crate::stream::pattern::*;
#[derive(Clone, Copy, Default)]
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
    candles: [Candle; 12],
    head: usize,
    len: usize,
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
            candles: [Candle::default(); 12],
            head: 0,
            len: 0,
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
        let value = if self.len == 12 {
            let candle = |offset: usize| self.candles[(self.head + offset) % 12];
            let a = candle(10);
            let b = candle(11);
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
                - cr_shadows_scalar(candle(0).o, candle(0).h, candle(0).l, candle(0).c);
            self.shadow_short_sum[1] += cr_shadows_scalar(b.o, b.h, b.l, b.c)
                - cr_shadows_scalar(candle(1).o, candle(1).h, candle(1).l, candle(1).c);
            self.shadow_short_sum[2] += cr_shadows_scalar(o, h, l, c)
                - cr_shadows_scalar(candle(2).o, candle(2).h, candle(2).l, candle(2).c);
            self.near_sum[0] +=
                cr_highlow_scalar(a.h, a.l) - cr_highlow_scalar(candle(5).h, candle(5).l);
            self.near_sum[1] +=
                cr_highlow_scalar(b.h, b.l) - cr_highlow_scalar(candle(6).h, candle(6).l);
            self.far_sum[0] +=
                cr_highlow_scalar(a.h, a.l) - cr_highlow_scalar(candle(5).h, candle(5).l);
            self.far_sum[1] +=
                cr_highlow_scalar(b.h, b.l) - cr_highlow_scalar(candle(6).h, candle(6).l);
            self.body_long_sum +=
                cr_realbody_scalar(a.o, a.c) - cr_realbody_scalar(candle(0).o, candle(0).c);
            Some(weakness as i32 * -100)
        } else {
            // Warm-up: seed the sums exactly like the batch prologue.
            let i = self.len;
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
        if self.len == 12 {
            self.candles[self.head] = cur;
            self.head = (self.head + 1) % 12;
        } else {
            self.candles[(self.head + self.len) % 12] = cur;
            self.len += 1;
        }
        self.value = value;
        value
    }
    /// Bulk-append aligned OHLC slices, pushing one score per bar into `output`.
    ///
    /// From a pristine state this runs directly over the slices and rebuilds
    /// the bounded candle ring once after the loop. A non-pristine state falls
    /// back to the per-bar loop. Either route is bit-identical to calling
    /// `append` once per bar (warm-up `None` becomes `0`, matching the batch
    /// prologue).
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
        const LOOKBACK: usize = 12;
        if self.len != 0 || len <= LOOKBACK {
            output.reserve(len);
            for i in 0..len {
                output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
            }
            return Ok(());
        }

        let start = output.len();
        output.resize(start + len, 0);
        let shadow = |i: usize| cr_shadows_scalar(open[i], high[i], low[i], close[i]);
        let range = |i: usize| cr_highlow_scalar(high[i], low[i]);
        let body = |i: usize| cr_realbody_scalar(open[i], close[i]);
        let mut body_long_sum = (0..10).fold(0.0, |sum, i| sum + body(i));
        let mut shadow_short_sum = [
            (0..10).fold(0.0, |sum, i| sum + shadow(i)),
            (1..11).fold(0.0, |sum, i| sum + shadow(i)),
            (2..12).fold(0.0, |sum, i| sum + shadow(i)),
        ];
        let mut near_sum = [
            (5..10).fold(0.0, |sum, i| sum + range(i)),
            (6..11).fold(0.0, |sum, i| sum + range(i)),
        ];
        let mut far_sum = near_sum;

        for i in LOOKBACK..len {
            let a = i - 2;
            let b = i - 1;
            let body_a = (close[a] - open[a]).abs();
            let body_b = (close[b] - open[b]).abs();
            let body_current = (close[i] - open[i]).abs();
            let upper_a = high[a] - open[a].max(close[a]);
            let upper_b = high[b] - open[b].max(close[b]);
            let upper_current = high[i] - open[i].max(close[i]);
            let near_a = ca_highlow_scalar(NEAR, near_sum[0], high[a], low[a]);
            let near_b = ca_highlow_scalar(NEAR, near_sum[1], high[b], low[b]);
            let far_a = ca_highlow_scalar(FAR, far_sum[0], high[a], low[a]);
            let far_b = ca_highlow_scalar(FAR, far_sum[1], high[b], low[b]);
            let shadow_a = ca_shadows_scalar(
                SHADOW_SHORT,
                shadow_short_sum[0],
                open[a],
                high[a],
                low[a],
                close[a],
            );
            let shadow_b = ca_shadows_scalar(
                SHADOW_SHORT,
                shadow_short_sum[1],
                open[b],
                high[b],
                low[b],
                close[b],
            );
            let shadow_current = ca_shadows_scalar(
                SHADOW_SHORT,
                shadow_short_sum[2],
                open[i],
                high[i],
                low[i],
                close[i],
            );
            let base = close[a] >= open[a]
                && close[b] >= open[b]
                && close[i] >= open[i]
                && close[b] > close[a]
                && close[i] > close[b]
                && open[b] > open[a]
                && open[b] <= close[a] + near_a
                && open[i] > open[b]
                && open[i] <= close[b] + near_b
                && body_a > ca_realbody_scalar(BODY_LONG, body_long_sum, open[a], close[a])
                && upper_a < shadow_a;
            let weakness = base
                && ((body_b < body_a - far_a && body_current < body_b + near_b)
                    || body_current < body_b - far_b
                    || (body_current < body_b
                        && body_b < body_a
                        && (upper_current > shadow_current || upper_b > shadow_b))
                    || (body_current < body_b
                        && upper_current
                            > ca_realbody_scalar(SHADOW_LONG, 0.0, open[i], close[i])));
            output[start + i] = weakness as i32 * -100;

            shadow_short_sum[0] += shadow(a) - shadow(i - 12);
            shadow_short_sum[1] += shadow(b) - shadow(i - 11);
            shadow_short_sum[2] += shadow(i) - shadow(i - 10);
            near_sum[0] += range(a) - range(i - 7);
            near_sum[1] += range(b) - range(i - 6);
            far_sum[0] += range(a) - range(i - 7);
            far_sum[1] += range(b) - range(i - 6);
            body_long_sum += body(a) - body(i - 12);
        }

        self.body_long_sum = body_long_sum;
        self.shadow_short_sum = shadow_short_sum;
        self.near_sum = near_sum;
        self.far_sum = far_sum;
        for (slot, i) in (len - LOOKBACK..len).enumerate() {
            self.candles[slot] = Candle {
                o: open[i],
                h: high[i],
                l: low[i],
                c: close[i],
            };
        }
        self.head = 0;
        self.len = LOOKBACK;
        self.value = Some(output[start + len - 1]);
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
        self.head = 0;
        self.len = 0;
        self.body_long_sum = 0.0;
        self.shadow_short_sum = [0.0; 3];
        self.near_sum = [0.0; 2];
        self.far_sum = [0.0; 2];
        self.value = None;
    }
}
