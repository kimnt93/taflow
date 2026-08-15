//! Incremental Evening Star candlestick recognition (CDLEVENINGSTAR).
use crate::error::TaResult;
use crate::stream::pattern::*;
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    o: f64,
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
/// Stateful CandleEveningStar candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleEveningStar {
    candles: VecDeque<Candle>,
    body_long_sum: f64,
    body_short_sum: f64,
    body_short2_sum: f64,
    value: Option<i32>,
}
impl Default for CandleEveningStar {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleEveningStar {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(12),
            body_long_sum: 0.0,
            body_short_sum: 0.0,
            body_short2_sum: 0.0,
            value: None,
        }
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, o: f64, _h: f64, _l: f64, c: f64) -> Option<i32> {
        let cur = Candle { o, c };
        // Deque holds bars i-12..=i-1; bar j maps to index 12 - (i - j).
        let value = if self.candles.len() == 12 {
            let a = self.candles[10]; // bar i-2
            let b = self.candles[11]; // bar i-1
            let long = ca_realbody_scalar(BODY_LONG, self.body_long_sum, a.o, a.c);
            let short = ca_realbody_scalar(BODY_SHORT, self.body_short_sum, b.o, b.c);
            let short2 = ca_realbody_scalar(BODY_SHORT, self.body_short2_sum, o, c);
            // Slide sums exactly like the batch loop: sum += cr(bar) - cr(bar - 10).
            self.body_long_sum += cr_realbody_scalar(a.o, a.c)
                - cr_realbody_scalar(self.candles[0].o, self.candles[0].c);
            self.body_short_sum += cr_realbody_scalar(b.o, b.c)
                - cr_realbody_scalar(self.candles[1].o, self.candles[1].c);
            self.body_short2_sum +=
                cr_realbody_scalar(o, c) - cr_realbody_scalar(self.candles[2].o, self.candles[2].c);
            Some(
                (a.color() == 1
                    && a.body() > long
                    && b.body() <= short
                    && b.o.min(b.c) > a.o.max(a.c)
                    && cur.color() == -1
                    && cur.body() > short2
                    && cur.c < a.c - a.body() * 0.3) as i32
                    * -100,
            )
        } else {
            // Warm-up: seed the sums exactly like the batch prologue.
            let i = self.candles.len();
            if i < 10 {
                self.body_long_sum += cr_realbody_scalar(o, c);
            }
            if (1..11).contains(&i) {
                self.body_short_sum += cr_realbody_scalar(o, c);
            }
            if (2..12).contains(&i) {
                self.body_short2_sum += cr_realbody_scalar(o, c);
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
    /// From a pristine state this runs a direct slice kernel and reconstructs
    /// the bounded trailing state without replaying the input. A non-pristine
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
        const LOOKBACK: usize = 12;
        if !self.candles.is_empty() || len <= LOOKBACK {
            for i in 0..len {
                output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
            }
            return Ok(());
        }
        let start = output.len();
        output.resize(start + len, 0);
        let mut body_long_sum = open[..10]
            .iter()
            .zip(&close[..10])
            .fold(0.0, |sum, (&o, &c)| sum + cr_realbody_scalar(o, c));
        let mut body_short_sum = open[1..11]
            .iter()
            .zip(&close[1..11])
            .fold(0.0, |sum, (&o, &c)| sum + cr_realbody_scalar(o, c));
        let mut body_short2_sum = open[2..12]
            .iter()
            .zip(&close[2..12])
            .fold(0.0, |sum, (&o, &c)| sum + cr_realbody_scalar(o, c));
        for ((opens, closes), out) in open
            .windows(13)
            .zip(close.windows(13))
            .zip(output[start + LOOKBACK..].iter_mut())
        {
            let a = Candle {
                o: opens[10],
                c: closes[10],
            };
            let b = Candle {
                o: opens[11],
                c: closes[11],
            };
            let cur = Candle {
                o: opens[12],
                c: closes[12],
            };
            let long = ca_realbody_scalar(BODY_LONG, body_long_sum, a.o, a.c);
            let short = ca_realbody_scalar(BODY_SHORT, body_short_sum, b.o, b.c);
            let short2 = ca_realbody_scalar(BODY_SHORT, body_short2_sum, cur.o, cur.c);
            *out = (a.color() == 1
                && a.body() > long
                && b.body() <= short
                && b.o.min(b.c) > a.o.max(a.c)
                && cur.color() == -1
                && cur.body() > short2
                && cur.c < a.c - a.body() * 0.3) as i32
                * -100;
            body_long_sum += cr_realbody_scalar(a.o, a.c) - cr_realbody_scalar(opens[0], closes[0]);
            body_short_sum +=
                cr_realbody_scalar(b.o, b.c) - cr_realbody_scalar(opens[1], closes[1]);
            body_short2_sum +=
                cr_realbody_scalar(cur.o, cur.c) - cr_realbody_scalar(opens[2], closes[2]);
        }
        self.body_long_sum = body_long_sum;
        self.body_short_sum = body_short_sum;
        self.body_short2_sum = body_short2_sum;
        self.candles.extend((len - LOOKBACK..len).map(|i| Candle {
            o: open[i],
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
        self.body_short_sum = 0.0;
        self.body_short2_sum = 0.0;
        self.value = None;
    }
}
