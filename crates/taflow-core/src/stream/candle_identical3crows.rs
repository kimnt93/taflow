//! Incremental Identical Three Crows candlestick recognition (CDLIDENTICAL3CROWS).
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
/// Stateful CandleIdenticalThreeCrows candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleIdenticalThreeCrows {
    candles: VecDeque<Candle>,
    shadow_sum: [f64; 3],
    equal_sum: [f64; 2],
    value: Option<i32>,
}
impl Default for CandleIdenticalThreeCrows {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleIdenticalThreeCrows {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(12),
            shadow_sum: [0.0; 3],
            equal_sum: [0.0; 2],
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
            let equal0 = ca_highlow_scalar(EQUAL, self.equal_sum[0], a.h, a.l);
            let equal1 = ca_highlow_scalar(EQUAL, self.equal_sum[1], b.h, b.l);
            // Slide sums exactly like the batch loop: sum += cr(bar) - cr(bar - period).
            self.shadow_sum[0] += cr_highlow_scalar(a.h, a.l)
                - cr_highlow_scalar(self.candles[0].h, self.candles[0].l);
            self.shadow_sum[1] += cr_highlow_scalar(b.h, b.l)
                - cr_highlow_scalar(self.candles[1].h, self.candles[1].l);
            self.shadow_sum[2] +=
                cr_highlow_scalar(h, l) - cr_highlow_scalar(self.candles[2].h, self.candles[2].l);
            self.equal_sum[0] += cr_highlow_scalar(a.h, a.l)
                - cr_highlow_scalar(self.candles[5].h, self.candles[5].l);
            self.equal_sum[1] += cr_highlow_scalar(b.h, b.l)
                - cr_highlow_scalar(self.candles[6].h, self.candles[6].l);
            Some(
                (a.color() == -1
                    && b.color() == -1
                    && cur.color() == -1
                    && b.c < a.c
                    && cur.c < b.c
                    && a.lower() < shadow0
                    && b.lower() < shadow1
                    && cur.lower() < shadow2
                    && (b.o - a.c).abs() <= equal0
                    && (cur.o - b.c).abs() <= equal1) as i32
                    * -100,
            )
        } else {
            // Warm-up: seed the sums exactly like the batch prologue.
            let i = self.candles.len();
            for k in 0..3 {
                if i >= k && i < 10 + k {
                    self.shadow_sum[k] += cr_highlow_scalar(h, l);
                }
            }
            if (5..10).contains(&i) {
                self.equal_sum[0] += cr_highlow_scalar(h, l);
            }
            if (6..11).contains(&i) {
                self.equal_sum[1] += cr_highlow_scalar(h, l);
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
        let scores = candle_identical_three_crows(open, high, low, close)?;
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
        self.equal_sum = [0.0; 2];
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
/// Compute the candle identical three crows result for the supplied aligned series.
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
pub fn candle_identical_three_crows(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = SHADOW_VERY_SHORT.avg_period.max(EQUAL.avg_period) + 2;
    if len <= lookback {
        return Ok(output);
    }

    let mut shadow_sum = [0.0f64; 3];
    let mut equal_sum = [0.0f64; 3];
    let start = lookback;
    for k in 0..3 {
        let bar = start - 2 + k;
        if bar >= SHADOW_VERY_SHORT.avg_period {
            for j in (bar - SHADOW_VERY_SHORT.avg_period)..bar {
                shadow_sum[k] += cr_highlow(open, high, low, close, j);
            }
        }
        if k < 2 && bar >= EQUAL.avg_period {
            for j in (bar - EQUAL.avg_period)..bar {
                equal_sum[k] += cr_highlow(open, high, low, close, j);
            }
        }
    }

    for i in start..len {
        output[i] = (candle_color(open[i-2], close[i-2]) == -1
            && candle_color(open[i-1], close[i-1]) == -1
            && candle_color(open[i], close[i]) == -1
            && close[i-1] < close[i-2] && close[i] < close[i-1]
            // Very short lower shadows
            && lower_shadow(open[i-2], low[i-2], close[i-2]) < ca_highlow(SHADOW_VERY_SHORT, shadow_sum[0], open, high, low, close, i-2)
            && lower_shadow(open[i-1], low[i-1], close[i-1]) < ca_highlow(SHADOW_VERY_SHORT, shadow_sum[1], open, high, low, close, i-1)
            && lower_shadow(open[i], low[i], close[i]) < ca_highlow(SHADOW_VERY_SHORT, shadow_sum[2], open, high, low, close, i)
            // Each opens equal to prior close
            && (open[i-1] - close[i-2]).abs() <= ca_highlow(EQUAL, equal_sum[0], open, high, low, close, i-2)
            && (open[i] - close[i-1]).abs() <= ca_highlow(EQUAL, equal_sum[1], open, high, low, close, i-1))
            as i32
            * -100;
        for k in 0..3 {
            let bar = i - 2 + k;
            if SHADOW_VERY_SHORT.avg_period > 0 && bar >= SHADOW_VERY_SHORT.avg_period {
                shadow_sum[k] += cr_highlow(open, high, low, close, bar)
                    - cr_highlow(open, high, low, close, bar - SHADOW_VERY_SHORT.avg_period);
            }
        }
        for k in 0..2 {
            let bar = i - 2 + k;
            if EQUAL.avg_period > 0 && bar >= EQUAL.avg_period {
                equal_sum[k] += cr_highlow(open, high, low, close, bar)
                    - cr_highlow(open, high, low, close, bar - EQUAL.avg_period);
            }
        }
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
        let e = crate::stream::candle_identical_three_crows(&o, &h, &l, &c).unwrap();
        let mut s = CandleIdenticalThreeCrows::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
