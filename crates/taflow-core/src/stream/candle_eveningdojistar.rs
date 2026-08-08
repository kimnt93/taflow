//! Incremental Evening Doji Star candlestick recognition (CDLEVENINGDOJISTAR).
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
/// Stateful CandleEveningDojiStar candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleEveningDojiStar {
    candles: VecDeque<Candle>,
    body_long_sum: f64,
    body_doji_sum: f64,
    body_short_sum: f64,
    value: Option<i32>,
}
impl Default for CandleEveningDojiStar {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleEveningDojiStar {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(12),
            body_long_sum: 0.0,
            body_doji_sum: 0.0,
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
            let long = ca_realbody_scalar(BODY_LONG, self.body_long_sum, a.o, a.c);
            let doji = ca_highlow_scalar(BODY_DOJI, self.body_doji_sum, b.h, b.l);
            let short = ca_realbody_scalar(BODY_SHORT, self.body_short_sum, o, c);
            // Slide sums exactly like the batch loop: sum += cr(bar) - cr(bar - 10).
            self.body_long_sum += cr_realbody_scalar(a.o, a.c)
                - cr_realbody_scalar(self.candles[0].o, self.candles[0].c);
            self.body_doji_sum += cr_highlow_scalar(b.h, b.l)
                - cr_highlow_scalar(self.candles[1].h, self.candles[1].l);
            self.body_short_sum +=
                cr_realbody_scalar(o, c) - cr_realbody_scalar(self.candles[2].o, self.candles[2].c);
            Some(
                (a.color() == 1
                    && a.body() > long
                    && b.body() <= doji
                    && b.o.min(b.c) > a.o.max(a.c)
                    && cur.color() == -1
                    && cur.body() > short
                    && cur.c < a.c - a.body() * 0.3) as i32
                    * -100,
            )
        } else {
            // Warm-up: seed the sums exactly like the batch prologue.
            let i = self.candles.len();
            if i < 10 {
                self.body_long_sum += cr_realbody_scalar(o, c);
            }
            if (1..11).contains(&i) {
                self.body_doji_sum += cr_highlow_scalar(h, l);
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
        let scores = candle_evening_doji_star(open, high, low, close, 0.3)?;
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
        self.body_doji_sum = 0.0;
        self.body_short_sum = 0.0;
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
/// Compute the candle evening doji star result for the supplied aligned series.
///
/// # Parameters
///
/// * `open` - Input series or configuration value.
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `close` - Input series or configuration value.
/// * `penetration` - Fraction of the first candle body that the final close must penetrate.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn candle_evening_doji_star(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
    penetration: f64,
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = *[
        BODY_DOJI.avg_period,
        BODY_LONG.avg_period,
        BODY_SHORT.avg_period,
    ]
    .iter()
    .max()
    .unwrap()
        + 2;
    if len <= lookback {
        return Ok(output);
    }

    let mut body_long_sum = 0.0;
    let mut body_doji_sum = 0.0;
    let mut body_short_sum = 0.0;
    let start = lookback;
    for i in (start - 2 - BODY_LONG.avg_period)..(start - 2) {
        body_long_sum += cr_realbody(open, high, low, close, i);
    }
    for i in (start - 1 - BODY_DOJI.avg_period)..(start - 1) {
        body_doji_sum += cr_highlow(open, high, low, close, i);
    }
    for i in (start - BODY_SHORT.avg_period)..start {
        body_short_sum += cr_realbody(open, high, low, close, i);
    }

    for i in start..len {
        output[i] = (candle_color(open[i - 2], close[i - 2]) == 1
            && real_body(open[i - 2], close[i - 2])
                > ca_realbody(BODY_LONG, body_long_sum, open, high, low, close, i - 2)
            && real_body(open[i - 1], close[i - 1])
                <= ca_highlow(BODY_DOJI, body_doji_sum, open, high, low, close, i - 1)
            && real_body_gap_up(open, close, i - 1, i - 2)
            && candle_color(open[i], close[i]) == -1
            && real_body(open[i], close[i])
                > ca_realbody(BODY_SHORT, body_short_sum, open, high, low, close, i)
            && close[i] < close[i - 2] - real_body(open[i - 2], close[i - 2]) * penetration)
            as i32
            * -100;
        body_long_sum += cr_realbody(open, high, low, close, i - 2)
            - cr_realbody(open, high, low, close, i - 2 - BODY_LONG.avg_period);
        body_doji_sum += cr_highlow(open, high, low, close, i - 1)
            - cr_highlow(open, high, low, close, i - 1 - BODY_DOJI.avg_period);
        body_short_sum += cr_realbody(open, high, low, close, i)
            - cr_realbody(open, high, low, close, i - BODY_SHORT.avg_period);
    }
    Ok(output)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let o: Vec<f64> = (0..45).map(|i| 100.0 + i as f64 * 0.2).collect();
        let h: Vec<f64> = o.iter().map(|x| x + 2.0).collect();
        let l: Vec<f64> = o.iter().map(|x| x - 2.0).collect();
        let c: Vec<f64> = o
            .iter()
            .enumerate()
            .map(|(i, x)| x + if i % 3 == 0 { -1.0 } else { 1.0 })
            .collect();
        let e = crate::stream::candle_evening_doji_star(&o, &h, &l, &c, 0.3).unwrap();
        let mut s = CandleEveningDojiStar::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
