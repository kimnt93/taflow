//! Incremental Abandoned Baby candlestick recognition (CDLABANDONEDBABY).
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
/// Stateful CandleAbandonedBaby candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleAbandonedBaby {
    candles: VecDeque<Candle>,
    body_long_sum: f64,
    body_doji_sum: f64,
    body_short_sum: f64,
    value: Option<i32>,
}
impl Default for CandleAbandonedBaby {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleAbandonedBaby {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(12),
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
        // Deque holds bars i-12..=i-1; bar j maps to index 12 - (i - j).
        let value = if self.candles.len() == 12 {
            let a = self.candles[10]; // bar i-2
            let b = self.candles[11]; // bar i-1
            let long = ca_realbody_scalar(BODY_LONG, self.body_long_sum, a.o, a.c);
            let doji = ca_highlow_scalar(BODY_DOJI, self.body_doji_sum, b.h, b.l);
            let short = ca_realbody_scalar(BODY_SHORT, self.body_short_sum, o, c);
            let base = a.body() > long && b.body() <= doji && cur.body() > short;
            let bull = base
                && a.color() == -1
                && cur.color() == 1
                && b.h < a.l
                && cur.l > b.h
                && cur.c > a.c + a.body() * 0.3;
            let bear = base
                && a.color() == 1
                && cur.color() == -1
                && b.l > a.h
                && cur.h < b.l
                && cur.c < a.c - a.body() * 0.3;
            // Slide sums exactly like the batch loop: sum += cr(bar) - cr(bar - 10).
            self.body_long_sum += cr_realbody_scalar(a.o, a.c)
                - cr_realbody_scalar(self.candles[0].o, self.candles[0].c);
            self.body_doji_sum += cr_highlow_scalar(b.h, b.l)
                - cr_highlow_scalar(self.candles[1].h, self.candles[1].l);
            self.body_short_sum +=
                cr_realbody_scalar(o, c) - cr_realbody_scalar(self.candles[2].o, self.candles[2].c);
            Some((bull as i32) * 100 - (bear as i32) * 100)
        } else {
            // Warm-up: seed the sums exactly like the batch prologue.
            let i = self.candles.len();
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
        for i in LOOKBACK..len {
            let a = i - 2;
            let b = i - 1;
            let long = ca_realbody_scalar(BODY_LONG, long_sum, open[a], close[a]);
            let doji = ca_highlow_scalar(BODY_DOJI, doji_sum, high[b], low[b]);
            let short = ca_realbody_scalar(BODY_SHORT, short_sum, open[i], close[i]);
            let base = real_body(open[a], close[a]) > long
                && real_body(open[b], close[b]) <= doji
                && real_body(open[i], close[i]) > short;
            let bull = base
                && candle_color(open[a], close[a]) == -1
                && candle_color(open[i], close[i]) == 1
                && high[b] < low[a]
                && low[i] > high[b]
                && close[i] > close[a] + real_body(open[a], close[a]) * 0.3;
            let bear = base
                && candle_color(open[a], close[a]) == 1
                && candle_color(open[i], close[i]) == -1
                && low[b] > high[a]
                && high[i] < low[b]
                && close[i] < close[a] - real_body(open[a], close[a]) * 0.3;
            output[start + i] = (bull as i32) * 100 - (bear as i32) * 100;
            long_sum += cr_realbody_scalar(open[a], close[a])
                - cr_realbody_scalar(open[i - 12], close[i - 12]);
            doji_sum +=
                cr_highlow_scalar(high[b], low[b]) - cr_highlow_scalar(high[i - 11], low[i - 11]);
            short_sum += cr_realbody_scalar(open[i], close[i])
                - cr_realbody_scalar(open[i - 10], close[i - 10]);
        }
        self.body_long_sum = long_sum;
        self.body_doji_sum = doji_sum;
        self.body_short_sum = short_sum;
        self.candles.extend((len - LOOKBACK..len).map(|i| Candle {
            o: open[i],
            h: high[i],
            l: low[i],
            c: close[i],
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
        self.body_long_sum = 0.0;
        self.body_doji_sum = 0.0;
        self.body_short_sum = 0.0;
        self.value = None;
    }
}
