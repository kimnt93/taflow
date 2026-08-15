//! Incremental Kicking candlestick recognition (CDLKICKING).
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
    fn lower(self) -> f64 {
        self.o.min(self.c) - self.l
    }
    fn color(self) -> i32 {
        if self.c >= self.o {
            1
        } else {
            -1
        }
    }
}
/// Stateful CandleKicking candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleKicking {
    candles: [Candle; 11],
    head: usize,
    len: usize,
    shadow_sum: [f64; 2],
    body_sum: [f64; 2],
    value: Option<i32>,
}
impl Default for CandleKicking {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleKicking {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: [Candle::default(); 11],
            head: 0,
            len: 0,
            shadow_sum: [0.0; 2],
            body_sum: [0.0; 2],
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
        let value = if self.len == 11 {
            let oldest = self.candles[self.head];
            let prev = self.candles[(self.head + 10) % 11];
            let vs_prev = ca_highlow_scalar(SHADOW_VERY_SHORT, self.shadow_sum[1], prev.h, prev.l);
            let vs_cur = ca_highlow_scalar(SHADOW_VERY_SHORT, self.shadow_sum[0], h, l);
            let body_prev = ca_realbody_scalar(BODY_LONG, self.body_sum[1], prev.o, prev.c);
            let body_cur = ca_realbody_scalar(BODY_LONG, self.body_sum[0], o, c);
            // Slide sums exactly like the batch loop: sum += cr(bar) - cr(bar - 10).
            self.shadow_sum[1] +=
                cr_highlow_scalar(prev.h, prev.l) - cr_highlow_scalar(oldest.h, oldest.l);
            let next_oldest = self.candles[(self.head + 1) % 11];
            self.shadow_sum[0] +=
                cr_highlow_scalar(h, l) - cr_highlow_scalar(next_oldest.h, next_oldest.l);
            self.body_sum[1] +=
                cr_realbody_scalar(prev.o, prev.c) - cr_realbody_scalar(oldest.o, oldest.c);
            self.body_sum[0] +=
                cr_realbody_scalar(o, c) - cr_realbody_scalar(next_oldest.o, next_oldest.c);
            let color_prev = prev.color();
            let color_cur = cur.color();
            let base = color_prev != color_cur
                && prev.body() > body_prev
                && prev.upper() < vs_prev
                && prev.lower() < vs_prev
                && cur.body() > body_cur
                && cur.upper() < vs_cur
                && cur.lower() < vs_cur;
            let bull = base && color_prev == -1 && color_cur == 1 && cur.l > prev.h;
            let bear = base && color_prev == 1 && color_cur == -1 && cur.h < prev.l;
            Some((bull as i32) * 100 - (bear as i32) * 100)
        } else {
            // Warm-up: seed the sums exactly like the batch prologue.
            let i = self.len;
            if i < 10 {
                self.shadow_sum[1] += cr_highlow_scalar(h, l);
                self.body_sum[1] += cr_realbody_scalar(o, c);
            }
            if (1..11).contains(&i) {
                self.shadow_sum[0] += cr_highlow_scalar(h, l);
                self.body_sum[0] += cr_realbody_scalar(o, c);
            }
            None
        };
        if self.len == 11 {
            self.candles[self.head] = cur;
            self.head = (self.head + 1) % 11;
        } else {
            self.candles[(self.head + self.len) % 11] = cur;
            self.len += 1;
        }
        self.value = value;
        value
    }
    /// Bulk-append aligned OHLC slices, pushing one score per bar into `output`.
    ///
    /// From a pristine state this runs directly over the slices and rebuilds
    /// the bounded candle window once after the loop. A non-pristine state
    /// falls back to the per-bar loop. Either route is bit-identical to calling
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
        const LOOKBACK: usize = 11;
        if self.len != 0 || len <= LOOKBACK {
            output.reserve(len);
            for i in 0..len {
                output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
            }
            return Ok(());
        }

        let start = output.len();
        output.resize(start + len, 0);
        let mut shadow_prev = high[..10]
            .iter()
            .zip(&low[..10])
            .fold(0.0, |sum, (&high, &low)| sum + cr_highlow_scalar(high, low));
        let mut shadow_current = high[1..11]
            .iter()
            .zip(&low[1..11])
            .fold(0.0, |sum, (&high, &low)| sum + cr_highlow_scalar(high, low));
        let mut body_prev = open[..10]
            .iter()
            .zip(&close[..10])
            .fold(0.0, |sum, (&open, &close)| {
                sum + cr_realbody_scalar(open, close)
            });
        let mut body_current = open[1..11]
            .iter()
            .zip(&close[1..11])
            .fold(0.0, |sum, (&open, &close)| {
                sum + cr_realbody_scalar(open, close)
            });

        for i in LOOKBACK..len {
            let previous = i - 1;
            let previous_color = if close[previous] >= open[previous] {
                1
            } else {
                -1
            };
            let current_color = if close[i] >= open[i] { 1 } else { -1 };
            let previous_threshold = ca_highlow_scalar(
                SHADOW_VERY_SHORT,
                shadow_prev,
                high[previous],
                low[previous],
            );
            let current_threshold =
                ca_highlow_scalar(SHADOW_VERY_SHORT, shadow_current, high[i], low[i]);
            let previous_body = (close[previous] - open[previous]).abs();
            let current_body = (close[i] - open[i]).abs();
            let base = previous_color != current_color
                && previous_body
                    > ca_realbody_scalar(BODY_LONG, body_prev, open[previous], close[previous])
                && high[previous] - open[previous].max(close[previous]) < previous_threshold
                && open[previous].min(close[previous]) - low[previous] < previous_threshold
                && current_body > ca_realbody_scalar(BODY_LONG, body_current, open[i], close[i])
                && high[i] - open[i].max(close[i]) < current_threshold
                && open[i].min(close[i]) - low[i] < current_threshold;
            let bull =
                base && previous_color == -1 && current_color == 1 && low[i] > high[previous];
            let bear =
                base && previous_color == 1 && current_color == -1 && high[i] < low[previous];
            output[start + i] = (bull as i32) * 100 - (bear as i32) * 100;

            let evicted = i - LOOKBACK;
            shadow_prev += cr_highlow_scalar(high[previous], low[previous])
                - cr_highlow_scalar(high[evicted], low[evicted]);
            shadow_current += cr_highlow_scalar(high[i], low[i])
                - cr_highlow_scalar(high[evicted + 1], low[evicted + 1]);
            body_prev += cr_realbody_scalar(open[previous], close[previous])
                - cr_realbody_scalar(open[evicted], close[evicted]);
            body_current += cr_realbody_scalar(open[i], close[i])
                - cr_realbody_scalar(open[evicted + 1], close[evicted + 1]);
        }

        self.shadow_sum = [shadow_current, shadow_prev];
        self.body_sum = [body_current, body_prev];
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
        self.shadow_sum = [0.0; 2];
        self.body_sum = [0.0; 2];
        self.value = None;
    }
}
