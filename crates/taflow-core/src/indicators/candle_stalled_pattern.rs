//! Incremental Stalled Pattern candlestick recognition (CDLSTALLEDPATTERN).
use crate::error::TaResult;
use crate::stream::pattern::*;
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
        const LOOKBACK: usize = 12;
        if !self.candles.is_empty() || len <= LOOKBACK {
            output.reserve(len);
            for i in 0..len {
                output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
            }
            return Ok(());
        }

        let start = output.len();
        output.resize(start + len, 0);
        let real_body_seed = |from: usize| {
            open[from..from + 10]
                .iter()
                .zip(&close[from..from + 10])
                .fold(0.0, |sum, (&o, &c)| sum + cr_realbody_scalar(o, c))
        };
        let high_low_seed = |from: usize, period: usize| {
            high[from..from + period]
                .iter()
                .zip(&low[from..from + period])
                .fold(0.0, |sum, (&h, &l)| sum + cr_highlow_scalar(h, l))
        };
        let mut body_long_sum = [real_body_seed(0), real_body_seed(1)];
        let mut body_short_sum = real_body_seed(2);
        let mut shadow_sum = high_low_seed(1, 10);
        let mut near_sum = [high_low_seed(5, 5), high_low_seed(6, 5)];

        for ((((slot, open), high), low), close) in output[start + LOOKBACK..]
            .iter_mut()
            .zip(open.windows(LOOKBACK + 1))
            .zip(high.windows(LOOKBACK + 1))
            .zip(low.windows(LOOKBACK + 1))
            .zip(close.windows(LOOKBACK + 1))
        {
            let long0 = ca_realbody_scalar(BODY_LONG, body_long_sum[0], open[10], close[10]);
            let long1 = ca_realbody_scalar(BODY_LONG, body_long_sum[1], open[11], close[11]);
            let short = ca_realbody_scalar(BODY_SHORT, body_short_sum, open[12], close[12]);
            let shadow = ca_highlow_scalar(SHADOW_VERY_SHORT, shadow_sum, high[11], low[11]);
            let near0 = ca_highlow_scalar(NEAR, near_sum[0], high[10], low[10]);
            let near1 = ca_highlow_scalar(NEAR, near_sum[1], high[11], low[11]);
            let body0 = real_body(open[10], close[10]);
            let body1 = real_body(open[11], close[11]);
            let body2 = real_body(open[12], close[12]);
            *slot = (candle_color(open[10], close[10]) == 1
                && candle_color(open[11], close[11]) == 1
                && candle_color(open[12], close[12]) == 1
                && close[11] > close[10]
                && close[12] > close[11]
                && body0 > long0
                && body1 > long1
                && upper_shadow(open[11], high[11], close[11]) < shadow
                && open[11] > open[10]
                && open[11] <= close[10] + near0
                && body2 < short
                && open[12] >= close[11] - body2 - near1) as i32
                * -100;

            body_long_sum[0] +=
                cr_realbody_scalar(open[10], close[10]) - cr_realbody_scalar(open[0], close[0]);
            body_long_sum[1] +=
                cr_realbody_scalar(open[11], close[11]) - cr_realbody_scalar(open[1], close[1]);
            body_short_sum +=
                cr_realbody_scalar(open[12], close[12]) - cr_realbody_scalar(open[2], close[2]);
            shadow_sum += cr_highlow_scalar(high[11], low[11]) - cr_highlow_scalar(high[1], low[1]);
            near_sum[0] +=
                cr_highlow_scalar(high[10], low[10]) - cr_highlow_scalar(high[5], low[5]);
            near_sum[1] +=
                cr_highlow_scalar(high[11], low[11]) - cr_highlow_scalar(high[6], low[6]);
        }

        self.body_long_sum = body_long_sum;
        self.body_short_sum = body_short_sum;
        self.shadow_sum = shadow_sum;
        self.near_sum = near_sum;
        self.candles
            .extend((len - LOOKBACK..len).map(|index| Candle {
                o: open[index],
                h: high[index],
                l: low[index],
                c: close[index],
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
        self.body_long_sum = [0.0; 2];
        self.body_short_sum = 0.0;
        self.shadow_sum = 0.0;
        self.near_sum = [0.0; 2];
        self.value = None;
    }
}
