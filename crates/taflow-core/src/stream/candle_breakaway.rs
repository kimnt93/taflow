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
        let value = if self.candles.len() == 14 {
            let a = self.candles[10];
            let b = self.candles[11];
            let cnd = self.candles[12];
            let d = self.candles[13];
            let long = self.candles.iter().take(10).map(|x| x.body()).sum::<f64>() / 10.0;
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
            Some((bear_first as i32 | bull_first as i32) * cur.color() * 100)
        } else {
            None
        };
        if self.candles.len() == 14 {
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
        body_sum += cr(BODY_LONG, open, high, low, close, i);
    }

    for i in start..len {
        let base = real_body(open[i - 4], close[i - 4])
            > ca(BODY_LONG, body_sum, open, high, low, close, i - 4)
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
        body_sum += cr(BODY_LONG, open, high, low, close, i - 4)
            - cr(
                BODY_LONG,
                open,
                high,
                low,
                close,
                i - 4 - BODY_LONG.avg_period,
            );
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
