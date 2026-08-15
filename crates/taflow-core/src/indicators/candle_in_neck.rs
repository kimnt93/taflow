//! Incremental In-Neck candlestick recognition (CDLINNECK).
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
/// Stateful CandleInNeck candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleInNeck {
    candles: VecDeque<Candle>,
    body_long_sum: f64,
    equal_sum: f64,
    value: Option<i32>,
}
impl Default for CandleInNeck {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleInNeck {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(11),
            body_long_sum: 0.0,
            equal_sum: 0.0,
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
            let equal = ca_highlow_scalar(EQUAL, self.equal_sum, prev.h, prev.l);
            // Slide sums exactly like the batch loop: sum += cr(bar) - cr(bar - period).
            self.equal_sum += cr_highlow_scalar(prev.h, prev.l)
                - cr_highlow_scalar(self.candles[5].h, self.candles[5].l);
            self.body_long_sum += cr_realbody_scalar(prev.o, prev.c)
                - cr_realbody_scalar(self.candles[0].o, self.candles[0].c);
            Some(
                (prev.color() == -1
                    && prev.body() > long
                    && cur.color() == 1
                    && cur.o < prev.l
                    && cur.c >= prev.c
                    && cur.c <= prev.c + equal) as i32
                    * -100,
            )
        } else {
            // Warm-up: seed the sums exactly like the batch prologue.
            let i = self.candles.len();
            if (5..10).contains(&i) {
                self.equal_sum += cr_highlow_scalar(h, l);
            }
            if i < 10 {
                self.body_long_sum += cr_realbody_scalar(o, c);
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
        const LOOKBACK: usize = 11;
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
        let mut equal_sum = high[5..10]
            .iter()
            .zip(&low[5..10])
            .fold(0.0, |sum, (&h, &l)| sum + cr_highlow_scalar(h, l));
        for ((((opens, highs), lows), closes), out) in open
            .windows(12)
            .zip(high.windows(12))
            .zip(low.windows(12))
            .zip(close.windows(12))
            .zip(output[start + LOOKBACK..].iter_mut())
        {
            let prev = Candle {
                o: opens[10],
                h: highs[10],
                l: lows[10],
                c: closes[10],
            };
            let cur = Candle {
                o: opens[11],
                h: highs[11],
                l: lows[11],
                c: closes[11],
            };
            let long = ca_realbody_scalar(BODY_LONG, body_long_sum, prev.o, prev.c);
            let equal = ca_highlow_scalar(EQUAL, equal_sum, prev.h, prev.l);
            *out = (prev.color() == -1
                && prev.body() > long
                && cur.color() == 1
                && cur.o < prev.l
                && cur.c >= prev.c
                && cur.c <= prev.c + equal) as i32
                * -100;
            equal_sum += cr_highlow_scalar(prev.h, prev.l) - cr_highlow_scalar(highs[5], lows[5]);
            body_long_sum +=
                cr_realbody_scalar(prev.o, prev.c) - cr_realbody_scalar(opens[0], closes[0]);
        }
        self.body_long_sum = body_long_sum;
        self.equal_sum = equal_sum;
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
        self.equal_sum = 0.0;
        self.value = None;
    }
}
