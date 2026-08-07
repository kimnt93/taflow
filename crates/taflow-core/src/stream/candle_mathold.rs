//! Incremental Mat Hold candlestick recognition (CDLMATHOLD).
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
/// Stateful CandleMatHold candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleMatHold {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleMatHold {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleMatHold {
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
            let short0 = self
                .candles
                .iter()
                .skip(1)
                .take(10)
                .map(|x| x.body())
                .sum::<f64>()
                / 10.0;
            let short1 = self
                .candles
                .iter()
                .skip(2)
                .take(10)
                .map(|x| x.body())
                .sum::<f64>()
                / 10.0;
            let short2 = self.candles.iter().skip(3).map(|x| x.body()).sum::<f64>() / 10.0;
            Some(
                (a.body() > long
                    && b.body() < short0
                    && cnd.body() < short1
                    && d.body() < short2
                    && a.color() == 1
                    && b.color() == -1
                    && cur.color() == 1
                    && b.o.min(b.c) > a.o.max(a.c)
                    && cnd.o.min(cnd.c) < a.c
                    && d.o.min(d.c) < a.c
                    && cnd.o.min(cnd.c) > a.c - a.body() * 0.5
                    && d.o.min(d.c) > a.c - a.body() * 0.5
                    && cnd.o.max(cnd.c) < b.o
                    && d.o.max(d.c) < cnd.o.max(cnd.c)
                    && cur.o > d.c
                    && cur.c > b.h.max(cnd.h).max(d.h)) as i32
                    * 100,
            )
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
/// Compute the candle mat hold result for the supplied aligned series.
///
/// # Parameters
///
/// * `open` - Input series or configuration value.
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `close` - Input series or configuration value.
/// * `penetration` - Fraction of the first candle body required for continuation.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn candle_mat_hold(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
    penetration: f64,
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_SHORT.avg_period.max(BODY_LONG.avg_period) + 4;
    if len <= lookback {
        return Ok(output);
    }

    let mut body_sum = [0.0f64; 5];
    let start = lookback;
    // Init long body sum for i-4, short for i-3..i-1, long for i
    for i in (start - 4 - BODY_LONG.avg_period)..(start - 4) {
        body_sum[4] += cr(BODY_LONG, open, high, low, close, i);
    }
    for k in 1..4 {
        let bar = start - 4 + k;
        for j in (bar - BODY_SHORT.avg_period)..bar {
            body_sum[4 - k] += cr(BODY_SHORT, open, high, low, close, j);
        }
    }

    for i in start..len {
        output[i] = (real_body(open[i-4], close[i-4]) > ca(BODY_LONG, body_sum[4], open, high, low, close, i-4)
            && real_body(open[i-3], close[i-3]) < ca(BODY_SHORT, body_sum[3], open, high, low, close, i-3)
            && real_body(open[i-2], close[i-2]) < ca(BODY_SHORT, body_sum[2], open, high, low, close, i-2)
            && real_body(open[i-1], close[i-1]) < ca(BODY_SHORT, body_sum[1], open, high, low, close, i-1)
            // white, black, ?, ?, white
            && candle_color(open[i-4], close[i-4]) == 1
            && candle_color(open[i-3], close[i-3]) == -1
            && candle_color(open[i], close[i]) == 1
            // upside gap 1st to 2nd
            && real_body_gap_up(open, close, i-3, i-4)
            // 3rd and 4th hold within 1st close
            && open[i-2].min(close[i-2]) < close[i-4]
            && open[i-1].min(close[i-1]) < close[i-4]
            // penetration check
            && open[i-2].min(close[i-2]) > close[i-4] - real_body(open[i-4], close[i-4]) * penetration
            && open[i-1].min(close[i-1]) > close[i-4] - real_body(open[i-4], close[i-4]) * penetration
            // 2nd to 4th are falling
            && open[i-2].max(close[i-2]) < open[i-3]
            && open[i-1].max(close[i-1]) < open[i-2].max(close[i-2])
            // 5th opens above prior close, closes above highest reaction high
            && open[i] > close[i-1]
            && close[i] > high[i-3].max(high[i-2]).max(high[i-1])) as i32
            * 100;
        body_sum[4] += cr(BODY_LONG, open, high, low, close, i - 4)
            - cr(
                BODY_LONG,
                open,
                high,
                low,
                close,
                i - 4 - BODY_LONG.avg_period,
            );
        for k in 1..4 {
            let bar = i - 4 + k;
            body_sum[4 - k] += cr(BODY_SHORT, open, high, low, close, bar)
                - cr(
                    BODY_SHORT,
                    open,
                    high,
                    low,
                    close,
                    bar - BODY_SHORT.avg_period,
                );
        }
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
        let e = crate::stream::candle_mat_hold(&o, &h, &l, &c, 0.5).unwrap();
        let mut s = CandleMatHold::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
