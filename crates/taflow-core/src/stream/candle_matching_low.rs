//! Incremental Matching Low candlestick recognition (CDLMATCHINGLOW).
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
    fn color(self) -> i32 {
        if self.c >= self.o {
            1
        } else {
            -1
        }
    }
}
/// Stateful CandleMatchingLow candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleMatchingLow {
    candles: VecDeque<Candle>,
    equal_sum: f64,
    value: Option<i32>,
}
impl Default for CandleMatchingLow {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleMatchingLow {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(6),
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
        // Deque holds bars i-6..=i-1; bar j maps to index 6 - (i - j).
        let value = if self.candles.len() == 6 {
            let prev = self.candles[5]; // bar i-1
            let equal = ca_highlow_scalar(EQUAL, self.equal_sum, prev.h, prev.l);
            // Slide the sum exactly like the batch loop: sum += cr(bar) - cr(bar - 5).
            self.equal_sum += cr_highlow_scalar(prev.h, prev.l)
                - cr_highlow_scalar(self.candles[0].h, self.candles[0].l);
            Some(
                (prev.color() == -1 && cur.color() == -1 && (cur.c - prev.c).abs() <= equal) as i32
                    * 100,
            )
        } else {
            // Warm-up: seed the sum exactly like the batch prologue.
            if self.candles.len() < 5 {
                self.equal_sum += cr_highlow_scalar(h, l);
            }
            None
        };
        if self.candles.len() == 6 {
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
        self.equal_sum = 0.0;
        self.value = None;
    }
}
