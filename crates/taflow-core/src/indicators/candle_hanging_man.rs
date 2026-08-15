//! Incremental Hanging Man candlestick recognition (CDLHANGINGMAN).
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
    fn lower(self) -> f64 {
        self.o.min(self.c) - self.l
    }
}
/// Stateful CandleHangingMan candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleHangingMan {
    candles: VecDeque<Candle>,
    body_sum: f64,
    shadow_vs_sum: f64,
    near_sum: f64,
    value: Option<i32>,
}
impl Default for CandleHangingMan {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleHangingMan {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(11),
            body_sum: 0.0,
            shadow_vs_sum: 0.0,
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
        // Deque holds bars i-11..=i-1; bar j maps to index 11 - (i - j).
        let value = if self.candles.len() == 11 {
            let prev = self.candles[10]; // bar i-1
            let body = ca_realbody_scalar(BODY_SHORT, self.body_sum, o, c);
            let vs = ca_highlow_scalar(SHADOW_VERY_SHORT, self.shadow_vs_sum, h, l);
            let near = ca_highlow_scalar(NEAR, self.near_sum, prev.h, prev.l);
            let out = (cur.body() < body
                && cur.lower() > cur.body()
                && cur.upper() < vs
                && cur.o.min(cur.c) >= prev.h - near) as i32
                * -100;
            // Slide sums exactly like the batch loop: sum += cr(bar) - cr(bar - period).
            self.body_sum +=
                cr_realbody_scalar(o, c) - cr_realbody_scalar(self.candles[1].o, self.candles[1].c);
            self.shadow_vs_sum +=
                cr_highlow_scalar(h, l) - cr_highlow_scalar(self.candles[1].h, self.candles[1].l);
            self.near_sum += cr_highlow_scalar(prev.h, prev.l)
                - cr_highlow_scalar(self.candles[5].h, self.candles[5].l);
            Some(out)
        } else {
            // Warm-up: seed the sums exactly like the batch prologue.
            let i = self.candles.len();
            if (1..11).contains(&i) {
                self.body_sum += cr_realbody_scalar(o, c);
                self.shadow_vs_sum += cr_highlow_scalar(h, l);
            }
            if (5..10).contains(&i) {
                self.near_sum += cr_highlow_scalar(h, l);
            }
            None
        };
        if self.candles.len() == 11 {
            self.candles.pop_front();
        }
        self.candles.push_back(cur);
        self.value = value;
        value
    }
    /// Bulk-append aligned OHLC slices, pushing one score per bar into `output`.
    ///
    /// From a pristine state this runs directly over the slices and rebuilds
    /// the rolling sums and bounded candle tail once after the loop. A
    /// non-pristine state falls back to the per-bar loop. Either route is bit-identical to
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
        const LOOKBACK: usize = 11;
        if !self.candles.is_empty() || len <= LOOKBACK {
            for i in 0..len {
                output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
            }
            return Ok(());
        }

        let output_start = output.len();
        output.resize(output_start + len, 0);
        let mut body_sum = open[1..11]
            .iter()
            .zip(&close[1..11])
            .fold(0.0, |sum, (&open, &close)| {
                sum + cr_realbody_scalar(open, close)
            });
        let mut shadow_sum = high[1..11]
            .iter()
            .zip(&low[1..11])
            .fold(0.0, |sum, (&high, &low)| sum + cr_highlow_scalar(high, low));
        let mut near_sum = high[5..10]
            .iter()
            .zip(&low[5..10])
            .fold(0.0, |sum, (&high, &low)| sum + cr_highlow_scalar(high, low));

        for (((open_window, high_window), low_window), (close_window, output)) in open
            .windows(LOOKBACK + 1)
            .zip(high.windows(LOOKBACK + 1))
            .zip(low.windows(LOOKBACK + 1))
            .zip(
                close
                    .windows(LOOKBACK + 1)
                    .zip(&mut output[output_start + LOOKBACK..]),
            )
        {
            let current_open = open_window[11];
            let current_high = high_window[11];
            let current_low = low_window[11];
            let current_close = close_window[11];
            let body = (current_close - current_open).abs();
            let body_threshold =
                ca_realbody_scalar(BODY_SHORT, body_sum, current_open, current_close);
            let shadow_threshold =
                ca_highlow_scalar(SHADOW_VERY_SHORT, shadow_sum, current_high, current_low);
            let near_threshold = ca_highlow_scalar(NEAR, near_sum, high_window[10], low_window[10]);
            *output = ((body < body_threshold
                && current_open.min(current_close) - current_low > body
                && current_high - current_open.max(current_close) < shadow_threshold
                && current_open.min(current_close) >= high_window[10] - near_threshold)
                as i32)
                * -100;

            body_sum += cr_realbody_scalar(current_open, current_close)
                - cr_realbody_scalar(open_window[1], close_window[1]);
            shadow_sum += cr_highlow_scalar(current_high, current_low)
                - cr_highlow_scalar(high_window[1], low_window[1]);
            near_sum += cr_highlow_scalar(high_window[10], low_window[10])
                - cr_highlow_scalar(high_window[5], low_window[5]);
        }

        let tail = len - LOOKBACK;
        self.candles.extend(
            open[tail..]
                .iter()
                .zip(&high[tail..])
                .zip(&low[tail..])
                .zip(&close[tail..])
                .map(|(((&o, &h), &l), &c)| Candle { o, h, l, c }),
        );
        self.body_sum = body_sum;
        self.shadow_vs_sum = shadow_sum;
        self.near_sum = near_sum;
        self.value = Some(output[output_start + len - 1]);
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
        self.body_sum = 0.0;
        self.shadow_vs_sum = 0.0;
        self.near_sum = 0.0;
        self.value = None;
    }
}
