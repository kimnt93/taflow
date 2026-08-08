//! Incremental Hanging Man candlestick recognition (CDLHANGINGMAN).
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
}
/// Stateful CandleHangingMan candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleHangingMan {
    candles: VecDeque<Candle>,
    body_sum: f64,
    shadow_vs_sum: f64,
    near_sum: f64,
    value: Option<i32>,
}
impl Default for CandleHangingMan {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleHangingMan {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(11),
            body_sum: 0.0,
            shadow_vs_sum: 0.0,
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
        // Deque holds bars i-11..=i-1; bar j maps to index 11 - (i - j).
        let value = if self.candles.len() == 11 {
            let prev = self.candles[10]; // bar i-1
            let body = ca_realbody_scalar(BODY_SHORT, self.body_sum, o, c);
            let vs = ca_highlow_scalar(SHADOW_VERY_SHORT, self.shadow_vs_sum, h, l);
            let near = ca_highlow_scalar(NEAR, self.near_sum, prev.h, prev.l);
            let out = (cur.body() < body
                && cur.lower() > cur.body()
                && cur.upper() < vs
                && cur.o.min(cur.c) >= prev.h - near) as i32
                * -100;
            // Slide sums exactly like the batch loop: sum += cr(bar) - cr(bar - period).
            self.body_sum +=
                cr_realbody_scalar(o, c) - cr_realbody_scalar(self.candles[1].o, self.candles[1].c);
            self.shadow_vs_sum +=
                cr_highlow_scalar(h, l) - cr_highlow_scalar(self.candles[1].h, self.candles[1].l);
            self.near_sum += cr_highlow_scalar(prev.h, prev.l)
                - cr_highlow_scalar(self.candles[5].h, self.candles[5].l);
            Some(out)
        } else {
            // Warm-up: seed the sums exactly like the batch prologue.
            let i = self.candles.len();
            if (1..11).contains(&i) {
                self.body_sum += cr_realbody_scalar(o, c);
                self.shadow_vs_sum += cr_highlow_scalar(h, l);
            }
            if (5..10).contains(&i) {
                self.near_sum += cr_highlow_scalar(h, l);
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
        let scores = candle_hanging_man(open, high, low, close)?;
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
        self.body_sum = 0.0;
        self.shadow_vs_sum = 0.0;
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
/// Compute the candle hanging man result for the supplied aligned series.
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
pub fn candle_hanging_man(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = *[
        BODY_SHORT.avg_period,
        SHADOW_LONG.avg_period,
        SHADOW_VERY_SHORT.avg_period,
        NEAR.avg_period,
    ]
    .iter()
    .max()
    .unwrap()
        + 1;
    if len <= lookback {
        return Ok(output);
    }

    let mut body_sum = 0.0;
    let shadow_long_sum = 0.0;
    let mut shadow_vs_sum = 0.0;
    let mut near_sum = 0.0;
    let start = lookback;
    for i in (start - BODY_SHORT.avg_period)..start {
        body_sum += cr_realbody(open, high, low, close, i);
    }
    for i in (start - SHADOW_VERY_SHORT.avg_period)..start {
        shadow_vs_sum += cr_highlow(open, high, low, close, i);
    }
    for i in (start - 1 - NEAR.avg_period)..(start - 1) {
        near_sum += cr_highlow(open, high, low, close, i);
    }

    for i in start..len {
        output[i] = (real_body(open[i], close[i])
            < ca_realbody(BODY_SHORT, body_sum, open, high, low, close, i)
            && lower_shadow(open[i], low[i], close[i])
                > ca_realbody(SHADOW_LONG, shadow_long_sum, open, high, low, close, i)
            && upper_shadow(open[i], high[i], close[i])
                < ca_highlow(SHADOW_VERY_SHORT, shadow_vs_sum, open, high, low, close, i)
            && open[i].min(close[i])
                >= high[i - 1] - ca_highlow(NEAR, near_sum, open, high, low, close, i - 1))
            as i32
            * -100;
        if BODY_SHORT.avg_period > 0 {
            body_sum += cr_realbody(open, high, low, close, i)
                - cr_realbody(open, high, low, close, i - BODY_SHORT.avg_period);
        }
        if SHADOW_VERY_SHORT.avg_period > 0 {
            shadow_vs_sum += cr_highlow(open, high, low, close, i)
                - cr_highlow(open, high, low, close, i - SHADOW_VERY_SHORT.avg_period);
        }
        if NEAR.avg_period > 0 {
            near_sum += cr_highlow(open, high, low, close, i - 1)
                - cr_highlow(open, high, low, close, i - 1 - NEAR.avg_period);
        }
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
        let e = crate::stream::candle_hanging_man(&o, &h, &l, &c).unwrap();
        let mut s = CandleHangingMan::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
