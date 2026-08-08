//! Incremental Breakaway candlestick recognition (CDLBREAKAWAY).
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
        let scores = candle_breakaway(open, high, low, close)?;
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
        self.body_long_sum = 0.0;
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
/// Compute the candle breakaway result for the supplied aligned series.
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
pub fn candle_breakaway(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_LONG.avg_period + 4;
    if len <= lookback {
        return Ok(output);
    }

    let mut body_sum = 0.0;
    let start = lookback;
    for i in (start - 4 - BODY_LONG.avg_period)..(start - 4) {
        body_sum += cr_realbody(open, high, low, close, i);
    }

    for i in start..len {
        let base = real_body(open[i - 4], close[i - 4])
            > ca_realbody(BODY_LONG, body_sum, open, high, low, close, i - 4)
            && candle_color(open[i - 4], close[i - 4]) == candle_color(open[i - 3], close[i - 3])
            && candle_color(open[i - 3], close[i - 3]) == candle_color(open[i - 1], close[i - 1])
            && candle_color(open[i - 1], close[i - 1]) == -candle_color(open[i], close[i]);
        // Bearish first (black): gap down, progressive lower H/L, 5th closes in gap
        let bear_first = base
            && candle_color(open[i - 4], close[i - 4]) == -1
            && real_body_gap_down(open, close, i - 3, i - 4)
            && high[i - 2] < high[i - 3]
            && low[i - 2] < low[i - 3]
            && high[i - 1] < high[i - 2]
            && low[i - 1] < low[i - 2]
            && close[i] > open[i - 3]
            && close[i] < close[i - 4];
        // Bullish first (white): gap up, progressive higher H/L, 5th closes in gap
        let bull_first = base
            && candle_color(open[i - 4], close[i - 4]) == 1
            && real_body_gap_up(open, close, i - 3, i - 4)
            && high[i - 2] > high[i - 3]
            && low[i - 2] > low[i - 3]
            && high[i - 1] > high[i - 2]
            && low[i - 1] > low[i - 2]
            && close[i] < open[i - 3]
            && close[i] > close[i - 4];
        output[i] = (bear_first as i32 | bull_first as i32) * candle_color(open[i], close[i]) * 100;
        body_sum += cr_realbody(open, high, low, close, i - 4)
            - cr_realbody(open, high, low, close, i - 4 - BODY_LONG.avg_period);
    }
    Ok(output)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let o: Vec<f64> = (0..48).map(|i| 100.0 + i as f64 * 0.2).collect();
        let h: Vec<f64> = o.iter().map(|x| x + 2.0).collect();
        let l: Vec<f64> = o.iter().map(|x| x - 2.0).collect();
        let c: Vec<f64> = o
            .iter()
            .enumerate()
            .map(|(i, x)| x + if i % 3 == 0 { -1.0 } else { 1.0 })
            .collect();
        let e = crate::stream::candle_breakaway(&o, &h, &l, &c).unwrap();
        let mut s = CandleBreakaway::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
