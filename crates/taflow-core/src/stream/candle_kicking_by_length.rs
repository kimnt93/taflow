//! Incremental Kicking - bull/bear determined by the longer body - (CDLKICKINGBYLENGTH).
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
    fn lower(self) -> f64 {
        self.o.min(self.c) - self.l
    }
    fn color(self) -> i32 {
        if self.c >= self.o {
            1
        } else {
            -1
        }
    }
}
/// Stateful CandleKickingByLength candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleKickingByLength {
    candles: VecDeque<Candle>,
    shadow_sum: [f64; 2],
    body_sum: [f64; 2],
    value: Option<i32>,
}
impl Default for CandleKickingByLength {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleKickingByLength {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(11),
            shadow_sum: [0.0; 2],
            body_sum: [0.0; 2],
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
            let vs_prev = ca_highlow_scalar(SHADOW_VERY_SHORT, self.shadow_sum[1], prev.h, prev.l);
            let vs_cur = ca_highlow_scalar(SHADOW_VERY_SHORT, self.shadow_sum[0], h, l);
            let body_prev = ca_realbody_scalar(BODY_LONG, self.body_sum[1], prev.o, prev.c);
            let body_cur = ca_realbody_scalar(BODY_LONG, self.body_sum[0], o, c);
            // Slide sums exactly like the batch loop: sum += cr(bar) - cr(bar - 10).
            self.shadow_sum[1] += cr_highlow_scalar(prev.h, prev.l)
                - cr_highlow_scalar(self.candles[0].h, self.candles[0].l);
            self.shadow_sum[0] +=
                cr_highlow_scalar(h, l) - cr_highlow_scalar(self.candles[1].h, self.candles[1].l);
            self.body_sum[1] += cr_realbody_scalar(prev.o, prev.c)
                - cr_realbody_scalar(self.candles[0].o, self.candles[0].c);
            self.body_sum[0] +=
                cr_realbody_scalar(o, c) - cr_realbody_scalar(self.candles[1].o, self.candles[1].c);
            let color_prev = prev.color();
            let color_cur = cur.color();
            let base = color_prev != color_cur
                && prev.body() > body_prev
                && prev.upper() < vs_prev
                && prev.lower() < vs_prev
                && cur.body() > body_cur
                && cur.upper() < vs_cur
                && cur.lower() < vs_cur;
            let has_gap = base
                && ((color_prev == -1 && color_cur == 1 && cur.l > prev.h)
                    || (color_prev == 1 && color_cur == -1 && cur.h < prev.l));
            let curr_longer = cur.body() > prev.body();
            let color = if curr_longer { color_cur } else { color_prev };
            Some(has_gap as i32 * color * 100)
        } else {
            // Warm-up: seed the sums exactly like the batch prologue.
            let i = self.candles.len();
            if i < 10 {
                self.shadow_sum[1] += cr_highlow_scalar(h, l);
                self.body_sum[1] += cr_realbody_scalar(o, c);
            }
            if (1..11).contains(&i) {
                self.shadow_sum[0] += cr_highlow_scalar(h, l);
                self.body_sum[0] += cr_realbody_scalar(o, c);
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
        self.shadow_sum = [0.0; 2];
        self.body_sum = [0.0; 2];
        self.value = None;
    }
}
