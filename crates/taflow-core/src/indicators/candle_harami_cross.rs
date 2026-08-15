//! Incremental Harami Cross candlestick recognition (CDLHARAMICROSS).
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
    fn color(self) -> i32 {
        if self.c >= self.o {
            1
        } else {
            -1
        }
    }
}
/// Stateful CandleHaramiCross candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleHaramiCross {
    candles: VecDeque<Candle>,
    body_long_sum: f64,
    body_doji_sum: f64,
    value: Option<i32>,
}
impl Default for CandleHaramiCross {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleHaramiCross {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(11),
            body_long_sum: 0.0,
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
        // Deque holds bars i-11..=i-1; bar j maps to index 11 - (i - j).
        let value = if self.candles.len() == 11 {
            let prev = self.candles[10]; // bar i-1
            let long = ca_realbody_scalar(BODY_LONG, self.body_long_sum, prev.o, prev.c);
            let doji = ca_highlow_scalar(BODY_DOJI, self.body_doji_sum, h, l);
            // Slide sums exactly like the batch loop: sum += cr(bar) - cr(bar - 10).
            self.body_long_sum += cr_realbody_scalar(prev.o, prev.c)
                - cr_realbody_scalar(self.candles[0].o, self.candles[0].c);
            self.body_doji_sum +=
                cr_highlow_scalar(h, l) - cr_highlow_scalar(self.candles[1].h, self.candles[1].l);
            Some(
                (prev.body() > long
                    && cur.body() <= doji
                    && cur.o.max(cur.c) < prev.o.max(prev.c)
                    && cur.o.min(cur.c) > prev.o.min(prev.c)) as i32
                    * -prev.color()
                    * 100,
            )
        } else {
            // Warm-up: seed the sums exactly like the batch prologue.
            let i = self.candles.len();
            if i < 10 {
                self.body_long_sum += cr_realbody_scalar(o, c);
            }
            if (1..11).contains(&i) {
                self.body_doji_sum += cr_highlow_scalar(h, l);
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
        let mut body_long_sum = open[..10]
            .iter()
            .zip(&close[..10])
            .fold(0.0, |sum, (&open, &close)| {
                sum + cr_realbody_scalar(open, close)
            });
        let mut body_doji_sum = high[1..11]
            .iter()
            .zip(&low[1..11])
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
            let previous_open = open_window[10];
            let previous_close = close_window[10];
            let current_open = open_window[11];
            let current_close = close_window[11];
            let long = ca_realbody_scalar(BODY_LONG, body_long_sum, previous_open, previous_close);
            let doji = ca_highlow_scalar(BODY_DOJI, body_doji_sum, high_window[11], low_window[11]);
            *output = (((previous_close - previous_open).abs() > long
                && (current_close - current_open).abs() <= doji
                && current_open.max(current_close) < previous_open.max(previous_close)
                && current_open.min(current_close) > previous_open.min(previous_close))
                as i32)
                * if previous_close >= previous_open {
                    -100
                } else {
                    100
                };

            body_long_sum += cr_realbody_scalar(previous_open, previous_close)
                - cr_realbody_scalar(open_window[0], close_window[0]);
            body_doji_sum += cr_highlow_scalar(high_window[11], low_window[11])
                - cr_highlow_scalar(high_window[1], low_window[1]);
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
        self.body_long_sum = body_long_sum;
        self.body_doji_sum = body_doji_sum;
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
        self.body_long_sum = 0.0;
        self.body_doji_sum = 0.0;
        self.value = None;
    }
}
