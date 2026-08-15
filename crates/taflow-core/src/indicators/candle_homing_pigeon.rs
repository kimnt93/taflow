//! Incremental Homing Pigeon candlestick recognition (CDLHOMINGPIGEON).
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
/// Stateful CandleHomingPigeon candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleHomingPigeon {
    candles: VecDeque<Candle>,
    body_long_sum: f64,
    body_short_sum: f64,
    value: Option<i32>,
}
impl Default for CandleHomingPigeon {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleHomingPigeon {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(11),
            body_long_sum: 0.0,
            body_short_sum: 0.0,
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
        // Deque holds bars i-11..=i-1; bar j maps to index 11 - (i - j).
        let value = if self.candles.len() == 11 {
            let prev = self.candles[10]; // bar i-1
            let long = ca_realbody_scalar(BODY_LONG, self.body_long_sum, prev.o, prev.c);
            let short = ca_realbody_scalar(BODY_SHORT, self.body_short_sum, o, c);
            // Slide sums exactly like the batch loop: sum += cr(bar) - cr(bar - 10).
            self.body_long_sum += cr_realbody_scalar(prev.o, prev.c)
                - cr_realbody_scalar(self.candles[0].o, self.candles[0].c);
            self.body_short_sum +=
                cr_realbody_scalar(o, c) - cr_realbody_scalar(self.candles[1].o, self.candles[1].c);
            Some(
                (prev.color() == -1
                    && cur.color() == -1
                    && prev.body() > long
                    && cur.body() <= short
                    && cur.o < prev.o
                    && cur.c > prev.c) as i32
                    * 100,
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
        let mut body_long_sum = open[..10]
            .iter()
            .zip(&close[..10])
            .fold(0.0, |sum, (&o, &c)| sum + cr_realbody_scalar(o, c));
        let mut body_short_sum = open[1..11]
            .iter()
            .zip(&close[1..11])
            .fold(0.0, |sum, (&o, &c)| sum + cr_realbody_scalar(o, c));
        let start = output.len();
        output.resize(start + len, 0);
        for ((opens, closes), out) in open
            .windows(12)
            .zip(close.windows(12))
            .zip(output[start + LOOKBACK..].iter_mut())
        {
            let prev = Candle {
                o: opens[10],
                c: closes[10],
            };
            let cur = Candle {
                o: opens[11],
                c: closes[11],
            };
            let long = ca_realbody_scalar(BODY_LONG, body_long_sum, prev.o, prev.c);
            let short = ca_realbody_scalar(BODY_SHORT, body_short_sum, cur.o, cur.c);
            *out = (prev.color() == -1
                && cur.color() == -1
                && prev.body() > long
                && cur.body() <= short
                && cur.o < prev.o
                && cur.c > prev.c) as i32
                * 100;
            body_long_sum +=
                cr_realbody_scalar(prev.o, prev.c) - cr_realbody_scalar(opens[0], closes[0]);
            body_short_sum +=
                cr_realbody_scalar(cur.o, cur.c) - cr_realbody_scalar(opens[1], closes[1]);
        }
        self.body_long_sum = body_long_sum;
        self.body_short_sum = body_short_sum;
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
        self.value = None;
    }
}
