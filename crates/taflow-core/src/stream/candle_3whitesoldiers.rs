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
        let scores = candle_three_white_soldiers(open, high, low, close)?;
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
        self.shadow_sum = [0.0; 3];
        self.near_sum = [0.0; 2];
        self.far_sum = [0.0; 2];
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
/// Compute the candle three white soldiers result for the supplied aligned series.
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
pub fn candle_three_white_soldiers(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = *[
        SHADOW_VERY_SHORT.avg_period,
        BODY_SHORT.avg_period,
        FAR.avg_period,
        NEAR.avg_period,
    ]
    .iter()
    .max()
    .unwrap()
        + 2;
    if len <= lookback {
        return Ok(output);
    }

    let mut shadow_sum = [0.0f64; 3];
    let mut near_sum = [0.0f64; 3];
    let mut far_sum = [0.0f64; 3];
    let mut body_short_sum = 0.0;
    let start = lookback;

    for k in 0..3 {
        let bar = start - 2 + k;
        if bar >= SHADOW_VERY_SHORT.avg_period {
            for j in (bar - SHADOW_VERY_SHORT.avg_period)..bar {
                shadow_sum[k] += cr_highlow(open, high, low, close, j);
            }
        }
        if bar >= NEAR.avg_period {
            for j in (bar - NEAR.avg_period)..bar {
                near_sum[k] += cr_highlow(open, high, low, close, j);
            }
        }
        if bar >= FAR.avg_period {
            for j in (bar - FAR.avg_period)..bar {
                far_sum[k] += cr_highlow(open, high, low, close, j);
            }
        }
    }
    for i in (start - BODY_SHORT.avg_period)..start {
        body_short_sum += cr_realbody(open, high, low, close, i);
    }

    for i in start..len {
        output[i] = (candle_color(open[i-2], close[i-2]) == 1
            && candle_color(open[i-1], close[i-1]) == 1
            && candle_color(open[i], close[i]) == 1
            && close[i-1] > close[i-2] && close[i] > close[i-1]
            // Short upper shadows
            && upper_shadow(open[i-2], high[i-2], close[i-2]) < ca_highlow(SHADOW_VERY_SHORT, shadow_sum[0], open, high, low, close, i-2)
            && upper_shadow(open[i-1], high[i-1], close[i-1]) < ca_highlow(SHADOW_VERY_SHORT, shadow_sum[1], open, high, low, close, i-1)
            && upper_shadow(open[i], high[i], close[i]) < ca_highlow(SHADOW_VERY_SHORT, shadow_sum[2], open, high, low, close, i)
            // Opens within or near previous body
            && open[i-1] > open[i-2] && open[i-1] <= close[i-2] + ca_highlow(NEAR, near_sum[1], open, high, low, close, i-1)
            && open[i] > open[i-1] && open[i] <= close[i-1] + ca_highlow(NEAR, near_sum[2], open, high, low, close, i)
            // Bodies not far shorter than prior
            && real_body(open[i-1], close[i-1]) > real_body(open[i-2], close[i-2]) - ca_highlow(FAR, far_sum[1], open, high, low, close, i-1)
            && real_body(open[i], close[i]) > real_body(open[i-1], close[i-1]) - ca_highlow(FAR, far_sum[2], open, high, low, close, i)
            // Last body not short
            && real_body(open[i], close[i]) > ca_realbody(BODY_SHORT, body_short_sum, open, high, low, close, i))
            as i32
            * 100;
        for k in 0..3 {
            let bar = i - 2 + k;
            if SHADOW_VERY_SHORT.avg_period > 0 && bar >= SHADOW_VERY_SHORT.avg_period {
                shadow_sum[k] += cr_highlow(open, high, low, close, bar)
                    - cr_highlow(open, high, low, close, bar - SHADOW_VERY_SHORT.avg_period);
            }
            if NEAR.avg_period > 0 && bar >= NEAR.avg_period {
                near_sum[k] += cr_highlow(open, high, low, close, bar)
                    - cr_highlow(open, high, low, close, bar - NEAR.avg_period);
            }
            if FAR.avg_period > 0 && bar >= FAR.avg_period {
                far_sum[k] += cr_highlow(open, high, low, close, bar)
                    - cr_highlow(open, high, low, close, bar - FAR.avg_period);
            }
        }
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
        let e = crate::stream::candle_three_white_soldiers(&o, &h, &l, &c).unwrap();
        let mut s = CandleThreeWhiteSoldiers::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
