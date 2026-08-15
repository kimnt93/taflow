//! Incremental Tasuki Gap candlestick recognition (CDLTASUKIGAP).
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
/// Stateful CandleTasukiGap candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleTasukiGap {
    candles: [Candle; 7],
    head: usize,
    len: usize,
    near_sum: f64,
    value: Option<i32>,
}
impl Default for CandleTasukiGap {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleTasukiGap {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: [Candle::default(); 7],
            head: 0,
            len: 0,
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
        let value = if self.len == 7 {
            let a = self.candles[(self.head + 5) % 7];
            let b = self.candles[(self.head + 6) % 7];
            let evicted = self.candles[(self.head + 1) % 7];
            let near = ca_highlow_scalar(NEAR, self.near_sum, b.h, b.l);
            // Slide the sum exactly like the batch loop: sum += cr(bar) - cr(bar - 5).
            self.near_sum += cr_highlow_scalar(b.h, b.l) - cr_highlow_scalar(evicted.h, evicted.l);
            let c1 = b.color();
            let c0 = cur.color();
            let near_same = (b.body() - cur.body()).abs() < near;
            let bull = b.o.min(b.c) > a.o.max(a.c)
                && c1 == 1
                && c0 == -1
                && cur.o < b.c
                && cur.o > b.o
                && cur.c < b.o
                && cur.c > a.o.max(a.c)
                && near_same;
            let bear = b.o.max(b.c) < a.o.min(a.c)
                && c1 == -1
                && c0 == 1
                && cur.o < b.o
                && cur.o > b.c
                && cur.c > b.o
                && cur.c < a.o.min(a.c)
                && near_same;
            Some((bull as i32 | bear as i32) * c1 * 100)
        } else {
            // Warm-up: seed the sum exactly like the batch prologue.
            if (1..6).contains(&self.len) {
                self.near_sum += cr_highlow_scalar(h, l);
            }
            None
        };
        if self.len == 7 {
            self.candles[self.head] = cur;
            self.head = (self.head + 1) % 7;
        } else {
            self.candles[(self.head + self.len) % 7] = cur;
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
        const LOOKBACK: usize = 7;
        if self.len != 0 || len <= LOOKBACK {
            output.reserve(len);
            for i in 0..len {
                output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
            }
            return Ok(());
        }

        let start = output.len();
        output.resize(start + len, 0);
        let mut near_sum = (1..6).fold(0.0, |sum, i| sum + cr_highlow_scalar(high[i], low[i]));
        for i in LOOKBACK..len {
            let a = i - 2;
            let b = i - 1;
            let color_b = if close[b] >= open[b] { 1 } else { -1 };
            let color_current = if close[i] >= open[i] { 1 } else { -1 };
            let near = ca_highlow_scalar(NEAR, near_sum, high[b], low[b]);
            let near_same = ((close[b] - open[b]).abs() - (close[i] - open[i]).abs()).abs() < near;
            let bull = open[b].min(close[b]) > open[a].max(close[a])
                && color_b == 1
                && color_current == -1
                && open[i] < close[b]
                && open[i] > open[b]
                && close[i] < open[b]
                && close[i] > open[a].max(close[a])
                && near_same;
            let bear = open[b].max(close[b]) < open[a].min(close[a])
                && color_b == -1
                && color_current == 1
                && open[i] < open[b]
                && open[i] > close[b]
                && close[i] > open[b]
                && close[i] < open[a].min(close[a])
                && near_same;
            output[start + i] = (bull as i32 | bear as i32) * color_b * 100;
            near_sum +=
                cr_highlow_scalar(high[b], low[b]) - cr_highlow_scalar(high[i - 6], low[i - 6]);
        }

        self.near_sum = near_sum;
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
        self.near_sum = 0.0;
        self.value = None;
    }
}
