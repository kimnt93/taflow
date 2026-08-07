//! Incremental Kicking - bull/bear determined by the longer body - (CDLKICKINGBYLENGTH).
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
    fn range(self) -> f64 {
        self.h - self.l
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
/// Stateful CandleKickingByLength candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleKickingByLength {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleKickingByLength {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleKickingByLength {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(11),
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
        let value = if self.candles.len() == 11 {
            let prev = self.candles[10];
            let vs_prev = self.candles.iter().take(10).map(|x| x.range()).sum::<f64>() * 0.01;
            let vs_cur = self.candles.iter().skip(1).map(|x| x.range()).sum::<f64>() * 0.01;
            let body_prev = self.candles.iter().take(10).map(|x| x.body()).sum::<f64>() / 10.0;
            let body_cur = self.candles.iter().skip(1).map(|x| x.body()).sum::<f64>() / 10.0;
            let color_prev = prev.color();
            let color_cur = cur.color();
            let base = color_prev != color_cur
                && prev.body() > body_prev
                && prev.upper() < vs_prev
                && prev.lower() < vs_prev
                && cur.body() > body_cur
                && cur.upper() < vs_cur
                && cur.lower() < vs_cur;
            let has_gap = base
                && ((color_prev == -1 && color_cur == 1 && cur.o > prev.o)
                    || (color_prev == 1 && color_cur == -1 && cur.o < prev.o));
            let curr_longer = cur.body() >= prev.body();
            let color = if curr_longer { color_cur } else { color_prev };
            Some(has_gap as i32 * color * 100)
        } else {
            None
        };
        if self.candles.len() == 11 {
            self.candles.pop_front();
        }
        self.candles.push_back(cur);
        self.value = value;
        value
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
        *self = Self::new();
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
/// Compute the candle kicking by length result for the supplied aligned series.
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
pub fn candle_kicking_by_length(
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
        shadow_sum[1] += cr(SHADOW_VERY_SHORT, open, high, low, close, i);
    }
    for i in (start - SHADOW_VERY_SHORT.avg_period)..start {
        shadow_sum[0] += cr(SHADOW_VERY_SHORT, open, high, low, close, i);
    }
    for i in (start - 1 - BODY_LONG.avg_period)..(start - 1) {
        body_sum[1] += cr(BODY_LONG, open, high, low, close, i);
    }
    for i in (start - BODY_LONG.avg_period)..start {
        body_sum[0] += cr(BODY_LONG, open, high, low, close, i);
    }

    for i in start..len {
        let color_prev = candle_color(open[i - 1], close[i - 1]);
        let color_curr = candle_color(open[i], close[i]);
        if color_prev != color_curr
            && real_body(open[i - 1], close[i - 1])
                > ca(BODY_LONG, body_sum[1], open, high, low, close, i - 1)
            && upper_shadow(open[i - 1], high[i - 1], close[i - 1])
                < ca(
                    SHADOW_VERY_SHORT,
                    shadow_sum[1],
                    open,
                    high,
                    low,
                    close,
                    i - 1,
                )
            && lower_shadow(open[i - 1], low[i - 1], close[i - 1])
                < ca(
                    SHADOW_VERY_SHORT,
                    shadow_sum[1],
                    open,
                    high,
                    low,
                    close,
                    i - 1,
                )
            && real_body(open[i], close[i]) > ca(BODY_LONG, body_sum[0], open, high, low, close, i)
            && upper_shadow(open[i], high[i], close[i])
                < ca(SHADOW_VERY_SHORT, shadow_sum[0], open, high, low, close, i)
            && lower_shadow(open[i], low[i], close[i])
                < ca(SHADOW_VERY_SHORT, shadow_sum[0], open, high, low, close, i)
        {
            // Gap check
            let has_gap = (color_prev == -1 && color_curr == 1 && open[i] > open[i - 1])
                || (color_prev == 1 && color_curr == -1 && open[i] < open[i - 1]);
            let curr_longer = real_body(open[i], close[i]) >= real_body(open[i - 1], close[i - 1]);
            // Branchless: select color based on which marubozu is longer
            let color = if curr_longer { color_curr } else { color_prev };
            output[i] = has_gap as i32 * color * 100;
        }
        shadow_sum[1] += cr(SHADOW_VERY_SHORT, open, high, low, close, i - 1)
            - cr(
                SHADOW_VERY_SHORT,
                open,
                high,
                low,
                close,
                i - 1 - SHADOW_VERY_SHORT.avg_period,
            );
        shadow_sum[0] += cr(SHADOW_VERY_SHORT, open, high, low, close, i)
            - cr(
                SHADOW_VERY_SHORT,
                open,
                high,
                low,
                close,
                i - SHADOW_VERY_SHORT.avg_period,
            );
        body_sum[1] += cr(BODY_LONG, open, high, low, close, i - 1)
            - cr(
                BODY_LONG,
                open,
                high,
                low,
                close,
                i - 1 - BODY_LONG.avg_period,
            );
        body_sum[0] += cr(BODY_LONG, open, high, low, close, i)
            - cr(BODY_LONG, open, high, low, close, i - BODY_LONG.avg_period);
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
        let e = crate::stream::candle_kicking_by_length(&o, &h, &l, &c).unwrap();
        let mut s = CandleKickingByLength::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
