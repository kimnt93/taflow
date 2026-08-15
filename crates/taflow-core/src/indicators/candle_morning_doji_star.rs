//! Incremental Morning Doji Star candlestick recognition (CDLMORNINGDOJISTAR).
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
    fn color(self) -> i32 {
        if self.c >= self.o {
            1
        } else {
            -1
        }
    }
}
/// Stateful CandleMorningDojiStar candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleMorningDojiStar {
    candles: [Candle; 12],
    head: usize,
    len: usize,
    body_long_sum: f64,
    body_doji_sum: f64,
    body_short_sum: f64,
    value: Option<i32>,
}
impl Default for CandleMorningDojiStar {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleMorningDojiStar {
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
            body_doji_sum: 0.0,
            body_short_sum: 0.0,
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
            let oldest = self.candles[self.head];
            let second = self.candles[(self.head + 1) % 12];
            let third = self.candles[(self.head + 2) % 12];
            let a = self.candles[(self.head + 10) % 12];
            let b = self.candles[(self.head + 11) % 12];
            let long = ca_realbody_scalar(BODY_LONG, self.body_long_sum, a.o, a.c);
            let doji = ca_highlow_scalar(BODY_DOJI, self.body_doji_sum, b.h, b.l);
            let short = ca_realbody_scalar(BODY_SHORT, self.body_short_sum, o, c);
            // Slide sums exactly like the batch loop: sum += cr(bar) - cr(bar - 10).
            self.body_long_sum +=
                cr_realbody_scalar(a.o, a.c) - cr_realbody_scalar(oldest.o, oldest.c);
            self.body_doji_sum +=
                cr_highlow_scalar(b.h, b.l) - cr_highlow_scalar(second.h, second.l);
            self.body_short_sum += cr_realbody_scalar(o, c) - cr_realbody_scalar(third.o, third.c);
            Some(
                (a.color() == -1
                    && a.body() > long
                    && b.body() <= doji
                    && b.o.max(b.c) < a.o.min(a.c)
                    && cur.color() == 1
                    && cur.body() > short
                    && cur.c > a.c + a.body() * 0.3) as i32
                    * 100,
            )
        } else {
            // Warm-up: seed the sums exactly like the batch prologue.
            let i = self.len;
            if i < 10 {
                self.body_long_sum += cr_realbody_scalar(o, c);
            }
            if (1..11).contains(&i) {
                self.body_doji_sum += cr_highlow_scalar(h, l);
            }
            if (2..12).contains(&i) {
                self.body_short_sum += cr_realbody_scalar(o, c);
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
        if self.len != 0 || len <= LOOKBACK {
            output.reserve(len);
            for i in 0..len {
                output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
            }
            return Ok(());
        }

        let start = output.len();
        output.resize(start + len, 0);
        let mut long_sum = open[..10]
            .iter()
            .zip(&close[..10])
            .map(|(&o, &c)| cr_realbody_scalar(o, c))
            .sum::<f64>();
        let mut doji_sum = high[1..11]
            .iter()
            .zip(&low[1..11])
            .map(|(&h, &l)| cr_highlow_scalar(h, l))
            .sum::<f64>();
        let mut short_sum = open[2..12]
            .iter()
            .zip(&close[2..12])
            .map(|(&o, &c)| cr_realbody_scalar(o, c))
            .sum::<f64>();
        for ((((slot, open), high), low), close) in output[start + LOOKBACK..]
            .iter_mut()
            .zip(open.windows(LOOKBACK + 1))
            .zip(high.windows(LOOKBACK + 1))
            .zip(low.windows(LOOKBACK + 1))
            .zip(close.windows(LOOKBACK + 1))
        {
            let long = ca_realbody_scalar(BODY_LONG, long_sum, open[10], close[10]);
            let doji = ca_highlow_scalar(BODY_DOJI, doji_sum, high[11], low[11]);
            let short = ca_realbody_scalar(BODY_SHORT, short_sum, open[12], close[12]);
            *slot = (candle_color(open[10], close[10]) == -1
                && real_body(open[10], close[10]) > long
                && real_body(open[11], close[11]) <= doji
                && open[11].max(close[11]) < open[10].min(close[10])
                && candle_color(open[12], close[12]) == 1
                && real_body(open[12], close[12]) > short
                && close[12] > close[10] + real_body(open[10], close[10]) * 0.3)
                as i32
                * 100;
            long_sum +=
                cr_realbody_scalar(open[10], close[10]) - cr_realbody_scalar(open[0], close[0]);
            doji_sum += cr_highlow_scalar(high[11], low[11]) - cr_highlow_scalar(high[1], low[1]);
            short_sum +=
                cr_realbody_scalar(open[12], close[12]) - cr_realbody_scalar(open[2], close[2]);
        }
        self.body_long_sum = long_sum;
        self.body_doji_sum = doji_sum;
        self.body_short_sum = short_sum;
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
        self.head = 0;
        self.len = 0;
        self.body_long_sum = 0.0;
        self.body_doji_sum = 0.0;
        self.body_short_sum = 0.0;
        self.value = None;
    }
}
