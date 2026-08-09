//! Incremental Ladder Bottom candlestick recognition (CDLLADDERBOTTOM).
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
/// Stateful CandleLadderBottom candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleLadderBottom {
    candles: VecDeque<Candle>,
    shadow_sum: f64,
    value: Option<i32>,
}
impl Default for CandleLadderBottom {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleLadderBottom {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(14),
            shadow_sum: 0.0,
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
        // Deque holds bars i-14..=i-1; bar j maps to index 14 - (i - j).
        let value = if self.candles.len() == 14 {
            let a = self.candles[10]; // bar i-4
            let b = self.candles[11];
            let cnd = self.candles[12];
            let d = self.candles[13];
            let shadow = ca_highlow_scalar(SHADOW_VERY_SHORT, self.shadow_sum, d.h, d.l);
            // Slide the sum exactly like the batch loop: sum += cr(bar) - cr(bar - 10).
            self.shadow_sum += cr_highlow_scalar(d.h, d.l)
                - cr_highlow_scalar(self.candles[3].h, self.candles[3].l);
            Some(
                (a.color() == -1
                    && b.color() == -1
                    && cnd.color() == -1
                    && d.color() == -1
                    && a.o > b.o
                    && b.o > cnd.o
                    && a.c > b.c
                    && b.c > cnd.c
                    && d.color() == -1
                    && d.upper() > shadow
                    && cur.color() == 1
                    && cur.o > d.o
                    && cur.c > d.h) as i32
                    * 100,
            )
        } else {
            // Warm-up: seed the sum exactly like the batch prologue.
            if (3..13).contains(&self.candles.len()) {
                self.shadow_sum += cr_highlow_scalar(h, l);
            }
            None
        };
        if self.candles.len() == 14 {
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
        self.shadow_sum = 0.0;
        self.value = None;
    }
}
