//! Incremental Breakaway candlestick recognition (CDLBREAKAWAY).
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
/// Stateful CandleBreakaway candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleBreakaway {
    candles: VecDeque<Candle>,
    body_long_sum: f64,
    value: Option<i32>,
}
impl Default for CandleBreakaway {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleBreakaway {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(14),
            body_long_sum: 0.0,
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
            let long = ca_realbody_scalar(BODY_LONG, self.body_long_sum, a.o, a.c);
            let base = a.body() > long
                && a.color() == b.color()
                && b.color() == d.color()
                && d.color() == -cur.color();
            let bear_first = base
                && a.color() == -1
                && b.o.max(b.c) < a.o.min(a.c)
                && cnd.h < b.h
                && cnd.l < b.l
                && d.h < cnd.h
                && d.l < cnd.l
                && cur.c > b.o
                && cur.c < a.c;
            let bull_first = base
                && a.color() == 1
                && b.o.min(b.c) > a.o.max(a.c)
                && cnd.h > b.h
                && cnd.l > b.l
                && d.h > cnd.h
                && d.l > cnd.l
                && cur.c < b.o
                && cur.c > a.c;
            // Slide the sum exactly like the batch loop: sum += cr(bar) - cr(bar - 10).
            self.body_long_sum += cr_realbody_scalar(a.o, a.c)
                - cr_realbody_scalar(self.candles[0].o, self.candles[0].c);
            Some((bear_first as i32 | bull_first as i32) * cur.color() * 100)
        } else {
            // Warm-up: seed the sum exactly like the batch prologue.
            if self.candles.len() < 10 {
                self.body_long_sum += cr_realbody_scalar(o, c);
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
    /// `body_long_sum` is *carried* through the steady loop in a local rather
    /// than reconstructed, so this needs no from-empty precondition and no
    /// intermediate score buffer. Only the 14-bar candle ring is window
    /// bounded, and it is rebuilt from the slice tail afterwards; a
    /// `PROLOGUE`-bar per-bar prefix guarantees the steady loop's `i-14 .. i`
    /// reads land inside this slice. Bit-identical to calling `append` once per
    /// bar (warm-up `None` becomes `0`, matching the batch prologue).
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
        /// Bars the ring spans; after this many appends it holds only slice bars.
        const PROLOGUE: usize = 14;
        let len = validate_ohlc(open, high, low, close)?;
        output.reserve(len);
        let prologue = len.min(PROLOGUE);
        for i in 0..prologue {
            output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
        }
        if len <= PROLOGUE {
            return Ok(());
        }
        let mut body_long_sum = self.body_long_sum;
        let mut last = 0;
        // Write through a pre-sized slice rather than `push`: the length
        // write-back of `push` sits on the critical path of every iteration.
        let base = output.len();
        output.resize(base + len - PROLOGUE, 0);
        let scores = &mut output[base..];
        // Lag-aligned subslices, all exactly `len - PROLOGUE` long, so the
        // steady loop's reads carry no bounds checks.
        let a_open_s = &open[PROLOGUE - 4..len - 4];
        let a_close_s = &close[PROLOGUE - 4..len - 4];
        let b_open_s = &open[PROLOGUE - 3..len - 3];
        let b_close_s = &close[PROLOGUE - 3..len - 3];
        let d_open_s = &open[PROLOGUE - 1..len - 1];
        let d_close_s = &close[PROLOGUE - 1..len - 1];
        let cur_open_s = &open[PROLOGUE..];
        let cur_close_s = &close[PROLOGUE..];
        let old_open_s = &open[..len - PROLOGUE];
        let old_close_s = &close[..len - PROLOGUE];
        let b_high_s = &high[PROLOGUE - 3..len - 3];
        let b_low_s = &low[PROLOGUE - 3..len - 3];
        let c_high_s = &high[PROLOGUE - 2..len - 2];
        let c_low_s = &low[PROLOGUE - 2..len - 2];
        let d_high_s = &high[PROLOGUE - 1..len - 1];
        let d_low_s = &low[PROLOGUE - 1..len - 1];
        for k in 0..scores.len() {
            // Bars i-4 (`a`), i-3 (`b`), i-2 (`c`), i-1 (`d`) and i (current).
            // The high/low reads of bars i-3..i-1 are loaded lazily: `base`
            // almost never holds, so they stay off the hot path exactly as in
            // the batch loop's short-circuit chain.
            let (a_open, a_close) = (a_open_s[k], a_close_s[k]);
            let (cur_open, cur_close) = (cur_open_s[k], cur_close_s[k]);
            let a_white = a_close >= a_open;
            let cur_white = cur_close >= cur_open;
            // Colour agreement first, branchlessly (`&`: every operand is an
            // already-loaded compare, so short-circuiting here would only add
            // unpredictable branches), and only then the long-body test. That
            // ordering keeps `ca_realbody_scalar`'s `sum / avg_period` division
            // - measured at ~40% of this kernel's cost - off the hot path, since
            // it is now needed on the one bar in eight where the colours line
            // up. Pure predicate reordering: every operand is a total function
            // of already-computed values, so the result is unchanged bit for
            // bit.
            let b_white = b_close_s[k] >= b_open_s[k];
            let d_white = d_close_s[k] >= d_open_s[k];
            let base = ((a_white == b_white) & (b_white == d_white) & (a_white != cur_white))
                && (a_close - a_open).abs()
                    > ca_realbody_scalar(BODY_LONG, body_long_sum, a_open, a_close);
            let hit = base && {
                let (b_open, b_high, b_low, b_close) =
                    (b_open_s[k], b_high_s[k], b_low_s[k], b_close_s[k]);
                let (c_high, c_low) = (c_high_s[k], c_low_s[k]);
                let (d_high, d_low) = (d_high_s[k], d_low_s[k]);
                if a_white {
                    b_open.min(b_close) > a_open.max(a_close)
                        && c_high > b_high
                        && c_low > b_low
                        && d_high > c_high
                        && d_low > c_low
                        && cur_close < b_open
                        && cur_close > a_close
                } else {
                    b_open.max(b_close) < a_open.min(a_close)
                        && c_high < b_high
                        && c_low < b_low
                        && d_high < c_high
                        && d_low < c_low
                        && cur_close > b_open
                        && cur_close < a_close
                }
            };
            // Slide the sum exactly like `append`: `+= cr(i-4) - cr(i-14)`.
            body_long_sum += cr_realbody_scalar(a_open, a_close)
                - cr_realbody_scalar(old_open_s[k], old_close_s[k]);
            last = if hit {
                if cur_white {
                    100
                } else {
                    -100
                }
            } else {
                0
            };
            scores[k] = last;
        }
        self.body_long_sum = body_long_sum;
        self.value = Some(last);
        // Rebuild the window-bounded ring so subsequent appends continue
        // bit-identically.
        self.candles.clear();
        for i in (len - PROLOGUE)..len {
            self.candles.push_back(Candle {
                o: open[i],
                h: high[i],
                l: low[i],
                c: close[i],
            });
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
        self.body_long_sum = 0.0;
        self.value = None;
    }
}
