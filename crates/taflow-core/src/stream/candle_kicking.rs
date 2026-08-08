//! Incremental Kicking candlestick recognition (CDLKICKING).
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
/// Stateful CandleKicking candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleKicking {
    candles: VecDeque<Candle>,
    shadow_sum: [f64; 2],
    body_sum: [f64; 2],
    value: Option<i32>,
}
impl Default for CandleKicking {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleKicking {
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
            let bull = base && color_prev == -1 && color_cur == 1 && cur.o > prev.o;
            let bear = base && color_prev == 1 && color_cur == -1 && cur.o < prev.o;
            Some((bull as i32) * 100 - (bear as i32) * 100)
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
        let scores = candle_kicking(open, high, low, close)?;
        output.extend_from_slice(&scores);
        // Every field of this state is a function of the last `BULK_REPLAY_BARS`
        // bars at most (deepest candle window is 10-bar average + 4 offset), so
        // replaying that tail from empty reproduces the full-run state exactly,
        // including `value` (set by the final `append`).
        let replay = len.min(BULK_REPLAY_BARS);
        for i in (len - replay)..len {
            self.append(open[i], high[i], low[i], close[i]);
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

/// Compute the candle pattern signal for aligned OHLC bars.
///
/// # Parameters
///
/// * `open`, `high`, `low`, `close` - Equal-length chronological OHLC series.
///
/// # Returns
///
/// A same-length vector containing -100, 0, or 100 pattern signals; bars
/// Compute the candle kicking result for the supplied aligned series.
///
/// # Parameters
///
/// * `open` - Input series or configuration value.
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `close` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn candle_kicking(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = SHADOW_VERY_SHORT.avg_period.max(BODY_LONG.avg_period) + 1;
    if len <= lookback {
        return Ok(output);
    }

    let mut shadow_sum = [0.0f64; 2];
    let mut body_sum = [0.0f64; 2];
    let start = lookback;
    for i in (start - 1 - SHADOW_VERY_SHORT.avg_period)..(start - 1) {
        shadow_sum[1] += cr_highlow(open, high, low, close, i);
    }
    for i in (start - SHADOW_VERY_SHORT.avg_period)..start {
        shadow_sum[0] += cr_highlow(open, high, low, close, i);
    }
    for i in (start - 1 - BODY_LONG.avg_period)..(start - 1) {
        body_sum[1] += cr_realbody(open, high, low, close, i);
    }
    for i in (start - BODY_LONG.avg_period)..start {
        body_sum[0] += cr_realbody(open, high, low, close, i);
    }

    for i in start..len {
        let color_prev = candle_color(open[i - 1], close[i - 1]);
        let color_curr = candle_color(open[i], close[i]);
        if color_prev != color_curr
            && real_body(open[i - 1], close[i - 1])
                > ca_realbody(BODY_LONG, body_sum[1], open, high, low, close, i - 1)
            && upper_shadow(open[i - 1], high[i - 1], close[i - 1])
                < ca_highlow(
                    SHADOW_VERY_SHORT,
                    shadow_sum[1],
                    open,
                    high,
                    low,
                    close,
                    i - 1,
                )
            && lower_shadow(open[i - 1], low[i - 1], close[i - 1])
                < ca_highlow(
                    SHADOW_VERY_SHORT,
                    shadow_sum[1],
                    open,
                    high,
                    low,
                    close,
                    i - 1,
                )
            && real_body(open[i], close[i])
                > ca_realbody(BODY_LONG, body_sum[0], open, high, low, close, i)
            && upper_shadow(open[i], high[i], close[i])
                < ca_highlow(SHADOW_VERY_SHORT, shadow_sum[0], open, high, low, close, i)
            && lower_shadow(open[i], low[i], close[i])
                < ca_highlow(SHADOW_VERY_SHORT, shadow_sum[0], open, high, low, close, i)
        {
            // Gap: black then white = bullish, white then black = bearish
            let bull = color_prev == -1 && color_curr == 1 && open[i] > open[i - 1];
            let bear = color_prev == 1 && color_curr == -1 && open[i] < open[i - 1];
            output[i] = (bull as i32) * 100 - (bear as i32) * 100;
        }
        shadow_sum[1] += cr_highlow(open, high, low, close, i - 1)
            - cr_highlow(open, high, low, close, i - 1 - SHADOW_VERY_SHORT.avg_period);
        shadow_sum[0] += cr_highlow(open, high, low, close, i)
            - cr_highlow(open, high, low, close, i - SHADOW_VERY_SHORT.avg_period);
        body_sum[1] += cr_realbody(open, high, low, close, i - 1)
            - cr_realbody(open, high, low, close, i - 1 - BODY_LONG.avg_period);
        body_sum[0] += cr_realbody(open, high, low, close, i)
            - cr_realbody(open, high, low, close, i - BODY_LONG.avg_period);
    }
    Ok(output)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let o: Vec<f64> = (0..40).map(|i| 100.0 + i as f64 * 0.2).collect();
        let h: Vec<f64> = o.iter().map(|x| x + 2.0).collect();
        let l: Vec<f64> = o.iter().map(|x| x - 2.0).collect();
        let c: Vec<f64> = o
            .iter()
            .enumerate()
            .map(|(i, x)| x + if i % 3 == 0 { -1.0 } else { 1.0 })
            .collect();
        let e = crate::stream::candle_kicking(&o, &h, &l, &c).unwrap();
        let mut s = CandleKicking::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
