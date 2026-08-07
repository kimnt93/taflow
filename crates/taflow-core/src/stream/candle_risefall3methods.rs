//! Incremental Rising/Falling Three Methods candlestick recognition (CDLRISEFALL3METHODS).
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
/// Stateful CandleRiseFallThreeMethods candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleRiseFallThreeMethods {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleRiseFallThreeMethods {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleRiseFallThreeMethods {
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
            let long0 = self.candles.iter().skip(4).map(|x| x.body()).sum::<f64>() / 10.0;
            let long4 = self.candles.iter().take(10).map(|x| x.body()).sum::<f64>() / 10.0;
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
            let c4 = a.color();
            let c0 = cur.color();
            let mut out = 0;
            if a.body() > long4 && cur.body() > long0 {
                let mid_short = b.body() < short0 && cnd.body() < short1 && d.body() < short2;
                let bull = c4 == 1
                    && mid_short
                    && b.color() == -1
                    && cnd.color() == -1
                    && d.color() == -1
                    && b.c < a.c
                    && cnd.c < b.c
                    && d.c < cnd.c
                    && b.l > a.l
                    && cnd.l > a.l
                    && d.l > a.l
                    && b.h < a.h
                    && cnd.h < a.h
                    && d.h < a.h
                    && c0 == 1
                    && cur.o > d.c
                    && cur.c > a.c;
                let bear = c4 == -1
                    && mid_short
                    && b.color() == 1
                    && cnd.color() == 1
                    && d.color() == 1
                    && b.c > a.c
                    && cnd.c > b.c
                    && d.c > cnd.c
                    && b.h < a.h
                    && cnd.h < a.h
                    && d.h < a.h
                    && b.l > a.l
                    && cnd.l > a.l
                    && d.l > a.l
                    && c0 == -1
                    && cur.o < d.c
                    && cur.c < a.c;
                out = (bull as i32) * 100 - (bear as i32) * 100;
            }
            Some(out)
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
/// Compute the candle rise fall three methods result for the supplied aligned series.
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
pub fn candle_rise_fall_three_methods(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_SHORT.avg_period.max(BODY_LONG.avg_period) + 4;
    if len <= lookback {
        return Ok(output);
    }

    let mut body_long_sum = [0.0f64; 2]; // [0]=i, [1]=i-4
    let mut body_short_sum = [0.0f64; 3]; // for i-3, i-2, i-1
    let start = lookback;
    for i in (start - 4 - BODY_LONG.avg_period)..(start - 4) {
        body_long_sum[1] += cr(BODY_LONG, open, high, low, close, i);
    }
    for i in (start - BODY_LONG.avg_period)..start {
        body_long_sum[0] += cr(BODY_LONG, open, high, low, close, i);
    }
    for k in 0..3 {
        let bar = start - 3 + k;
        for j in (bar - BODY_SHORT.avg_period)..bar {
            body_short_sum[k] += cr(BODY_SHORT, open, high, low, close, j);
        }
    }

    for i in start..len {
        let c4 = candle_color(open[i - 4], close[i - 4]);
        let c0 = candle_color(open[i], close[i]);
        if real_body(open[i - 4], close[i - 4])
            > ca(BODY_LONG, body_long_sum[1], open, high, low, close, i - 4)
            && real_body(open[i], close[i])
                > ca(BODY_LONG, body_long_sum[0], open, high, low, close, i)
        {
            // 1st long, 3 short middle, 5th long
            let mid_short = real_body(open[i - 3], close[i - 3])
                < ca(BODY_SHORT, body_short_sum[0], open, high, low, close, i - 3)
                && real_body(open[i - 2], close[i - 2])
                    < ca(BODY_SHORT, body_short_sum[1], open, high, low, close, i - 2)
                && real_body(open[i - 1], close[i - 1])
                    < ca(BODY_SHORT, body_short_sum[2], open, high, low, close, i - 1);

            let bull = c4 == 1
                && mid_short
                && candle_color(open[i - 3], close[i - 3]) == -1
                && candle_color(open[i - 2], close[i - 2]) == -1
                && candle_color(open[i - 1], close[i - 1]) == -1
                && close[i - 3] < close[i - 4]
                && close[i - 2] < close[i - 3]
                && close[i - 1] < close[i - 2]
                && low[i - 3] > low[i - 4]
                && low[i - 2] > low[i - 4]
                && low[i - 1] > low[i - 4]
                && high[i - 3] < high[i - 4]
                && high[i - 2] < high[i - 4]
                && high[i - 1] < high[i - 4]
                && c0 == 1
                && open[i] > close[i - 1]
                && close[i] > close[i - 4];
            let bear = c4 == -1
                && mid_short
                && candle_color(open[i - 3], close[i - 3]) == 1
                && candle_color(open[i - 2], close[i - 2]) == 1
                && candle_color(open[i - 1], close[i - 1]) == 1
                && close[i - 3] > close[i - 4]
                && close[i - 2] > close[i - 3]
                && close[i - 1] > close[i - 2]
                && high[i - 3] < high[i - 4]
                && high[i - 2] < high[i - 4]
                && high[i - 1] < high[i - 4]
                && low[i - 3] > low[i - 4]
                && low[i - 2] > low[i - 4]
                && low[i - 1] > low[i - 4]
                && c0 == -1
                && open[i] < close[i - 1]
                && close[i] < close[i - 4];
            output[i] = (bull as i32) * 100 - (bear as i32) * 100;
        }
        body_long_sum[1] += cr(BODY_LONG, open, high, low, close, i - 4)
            - cr(
                BODY_LONG,
                open,
                high,
                low,
                close,
                i - 4 - BODY_LONG.avg_period,
            );
        body_long_sum[0] += cr(BODY_LONG, open, high, low, close, i)
            - cr(BODY_LONG, open, high, low, close, i - BODY_LONG.avg_period);
        for k in 0..3 {
            let bar = i - 3 + k;
            body_short_sum[k] += cr(BODY_SHORT, open, high, low, close, bar)
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
        let e = crate::stream::candle_rise_fall_three_methods(&o, &h, &l, &c).unwrap();
        let mut s = CandleRiseFallThreeMethods::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
