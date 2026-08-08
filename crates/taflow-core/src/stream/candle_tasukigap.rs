//! Incremental Tasuki Gap candlestick recognition (CDLTASUKIGAP).
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
/// Stateful CandleTasukiGap candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleTasukiGap {
    candles: VecDeque<Candle>,
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
            candles: VecDeque::with_capacity(7),
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
        // Deque holds bars i-7..=i-1; bar j maps to index 7 - (i - j).
        let value = if self.candles.len() == 7 {
            let a = self.candles[5]; // bar i-2
            let b = self.candles[6]; // bar i-1
            let near = ca_highlow_scalar(NEAR, self.near_sum, b.h, b.l);
            // Slide the sum exactly like the batch loop: sum += cr(bar) - cr(bar - 5).
            self.near_sum += cr_highlow_scalar(b.h, b.l)
                - cr_highlow_scalar(self.candles[1].h, self.candles[1].l);
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
            if (1..6).contains(&self.candles.len()) {
                self.near_sum += cr_highlow_scalar(h, l);
            }
            None
        };
        if self.candles.len() == 7 {
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
        let scores = candle_tasuki_gap(open, high, low, close)?;
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
        self.near_sum = 0.0;
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
/// Compute the candle tasuki gap result for the supplied aligned series.
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
pub fn candle_tasuki_gap(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = NEAR.avg_period + 2;
    if len <= lookback {
        return Ok(output);
    }

    let mut near_sum = 0.0;
    let start = lookback;
    for i in (start - 1 - NEAR.avg_period)..(start - 1) {
        near_sum += cr_highlow(open, high, low, close, i);
    }

    for i in start..len {
        let c1 = candle_color(open[i - 1], close[i - 1]);
        let c0 = candle_color(open[i], close[i]);

        // Bodies near same size
        let near_same = (real_body(open[i - 1], close[i - 1]) - real_body(open[i], close[i])).abs()
            < ca_highlow(NEAR, near_sum, open, high, low, close, i - 1);

        // Bullish: upside gap, white bar then black bar
        let bull = real_body_gap_up(open, close, i - 1, i - 2)
            && c1 == 1
            && c0 == -1
            && open[i] < close[i - 1]
            && open[i] > open[i - 1]
            && close[i] < open[i - 1]
            && close[i] > open[i - 2].max(close[i - 2])
            && near_same;
        // Bearish: downside gap, black bar then white bar
        let bear = real_body_gap_down(open, close, i - 1, i - 2)
            && c1 == -1
            && c0 == 1
            && open[i] < open[i - 1]
            && open[i] > close[i - 1]
            && close[i] > open[i - 1]
            && close[i] < open[i - 2].min(close[i - 2])
            && near_same;
        output[i] = (bull as i32 | bear as i32) * c1 * 100;
        near_sum += cr_highlow(open, high, low, close, i - 1)
            - cr_highlow(open, high, low, close, i - 1 - NEAR.avg_period);
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
        let e = crate::stream::candle_tasuki_gap(&o, &h, &l, &c).unwrap();
        let mut s = CandleTasukiGap::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
