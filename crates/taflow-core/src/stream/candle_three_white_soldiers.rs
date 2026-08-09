//! Incremental Three White Soldiers candlestick recognition (CDL3WHITESOLDIERS).
use super::pattern::*;
use crate::error::TaResult;
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
/// Stateful CandleThreeWhiteSoldiers candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleThreeWhiteSoldiers {
    candles: VecDeque<Candle>,
    shadow_sum: [f64; 3],
    near_sum: [f64; 2],
    far_sum: [f64; 2],
    body_short_sum: f64,
    value: Option<i32>,
}
impl Default for CandleThreeWhiteSoldiers {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleThreeWhiteSoldiers {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(12),
            shadow_sum: [0.0; 3],
            near_sum: [0.0; 2],
            far_sum: [0.0; 2],
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
            let shadow0 = ca_highlow_scalar(SHADOW_VERY_SHORT, self.shadow_sum[0], a.h, a.l);
            let shadow1 = ca_highlow_scalar(SHADOW_VERY_SHORT, self.shadow_sum[1], b.h, b.l);
            let shadow2 = ca_highlow_scalar(SHADOW_VERY_SHORT, self.shadow_sum[2], h, l);
            let near1 = ca_highlow_scalar(NEAR, self.near_sum[0], b.h, b.l);
            let near2 = ca_highlow_scalar(NEAR, self.near_sum[1], h, l);
            let far1 = ca_highlow_scalar(FAR, self.far_sum[0], b.h, b.l);
            let far2 = ca_highlow_scalar(FAR, self.far_sum[1], h, l);
            let body_short = ca_realbody_scalar(BODY_SHORT, self.body_short_sum, o, c);
            let pattern = a.color() == 1
                && b.color() == 1
                && cur.color() == 1
                && b.c > a.c
                && cur.c > b.c
                && a.upper() < shadow0
                && b.upper() < shadow1
                && cur.upper() < shadow2
                && b.o > a.o
                && b.o <= a.c + near1
                && cur.o > b.o
                && cur.o <= b.c + near2
                && b.body() > a.body() - far1
                && cur.body() > b.body() - far2
                && cur.body() > body_short;
            // Slide sums exactly like the batch loop: sum += cr(bar) - cr(bar - period).
            self.shadow_sum[0] += cr_highlow_scalar(a.h, a.l)
                - cr_highlow_scalar(self.candles[0].h, self.candles[0].l);
            self.shadow_sum[1] += cr_highlow_scalar(b.h, b.l)
                - cr_highlow_scalar(self.candles[1].h, self.candles[1].l);
            self.shadow_sum[2] +=
                cr_highlow_scalar(h, l) - cr_highlow_scalar(self.candles[2].h, self.candles[2].l);
            self.near_sum[0] += cr_highlow_scalar(b.h, b.l)
                - cr_highlow_scalar(self.candles[6].h, self.candles[6].l);
            self.near_sum[1] +=
                cr_highlow_scalar(h, l) - cr_highlow_scalar(self.candles[7].h, self.candles[7].l);
            self.far_sum[0] += cr_highlow_scalar(b.h, b.l)
                - cr_highlow_scalar(self.candles[6].h, self.candles[6].l);
            self.far_sum[1] +=
                cr_highlow_scalar(h, l) - cr_highlow_scalar(self.candles[7].h, self.candles[7].l);
            self.body_short_sum +=
                cr_realbody_scalar(o, c) - cr_realbody_scalar(self.candles[2].o, self.candles[2].c);
            Some((pattern) as i32 * 100)
        } else {
            // Warm-up: seed the sums exactly like the batch prologue.
            let i = self.candles.len();
            for k in 0..3 {
                if i >= k && i < 10 + k {
                    self.shadow_sum[k] += cr_highlow_scalar(h, l);
                }
            }
            if (6..11).contains(&i) {
                self.near_sum[0] += cr_highlow_scalar(h, l);
                self.far_sum[0] += cr_highlow_scalar(h, l);
            }
            if (7..12).contains(&i) {
                self.near_sum[1] += cr_highlow_scalar(h, l);
                self.far_sum[1] += cr_highlow_scalar(h, l);
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
        output.reserve(len);
        if !self.candles.is_empty() {
            for i in 0..len {
                output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
            }
            return Ok(());
        }
        for i in 0..len {
            output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
        }
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
        self.shadow_sum = [0.0; 3];
        self.near_sum = [0.0; 2];
        self.far_sum = [0.0; 2];
        self.body_short_sum = 0.0;
        self.value = None;
    }
}
