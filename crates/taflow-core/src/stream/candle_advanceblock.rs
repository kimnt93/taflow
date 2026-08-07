//! Incremental Advance Block candlestick recognition (CDLADVANCEBLOCK).
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
    fn shadow(self) -> f64 {
        self.range() - self.body()
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
/// Stateful CandleAdvanceBlock candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleAdvanceBlock {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleAdvanceBlock {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleAdvanceBlock {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(12),
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
        let value = if self.candles.len() == 12 {
            let a = self.candles[10];
            let b = self.candles[11];
            let long = self.candles.iter().take(10).map(|x| x.body()).sum::<f64>() / 10.0;
            let shadow_short = self
                .candles
                .iter()
                .take(10)
                .map(|x| x.shadow())
                .sum::<f64>()
                / 20.0;
            let near1 = self
                .candles
                .iter()
                .skip(6)
                .take(5)
                .map(|x| x.range())
                .sum::<f64>()
                * 0.04;
            let near2 = self
                .candles
                .iter()
                .skip(7)
                .take(5)
                .map(|x| x.range())
                .sum::<f64>()
                * 0.04;
            let far1 = self
                .candles
                .iter()
                .skip(6)
                .take(5)
                .map(|x| x.range())
                .sum::<f64>()
                * 0.12;
            let far2 = self
                .candles
                .iter()
                .skip(7)
                .take(5)
                .map(|x| x.range())
                .sum::<f64>()
                * 0.12;
            let base = a.color() == 1
                && b.color() == 1
                && cur.color() == 1
                && b.c > a.c
                && cur.c > b.c
                && b.o > a.o
                && b.o <= a.c + near1
                && cur.o > b.o
                && cur.o <= b.c + near2
                && a.body() > long
                && a.upper() < shadow_short;
            let weakness = base
                && ((b.body() < a.body() - far1 && cur.body() < b.body() + near2)
                    || (cur.body() < b.body()
                        && b.body() < a.body()
                        && (cur.upper() > cur.body() || b.upper() > b.body()))
                    || cur.body() < b.body() - far2);
            Some(weakness as i32 * -100)
        } else {
            None
        };
        if self.candles.len() == 12 {
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
/// Compute the candle advance block result for the supplied aligned series.
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
pub fn candle_advance_block(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = *[
        SHADOW_LONG.avg_period,
        SHADOW_SHORT.avg_period,
        FAR.avg_period,
        NEAR.avg_period,
        BODY_LONG.avg_period,
    ]
    .iter()
    .max()
    .unwrap()
        + 2;
    if len <= lookback {
        return Ok(output);
    }

    let mut shadow_short_sum = [0.0f64; 3];
    let mut shadow_long_sum = [0.0f64; 3];
    let mut near_sum = [0.0f64; 3];
    let mut far_sum = [0.0f64; 3];
    let mut body_long_sum = 0.0;
    let start = lookback;

    for k in 0..3 {
        let bar = start - 2 + k;
        // SHADOW_LONG and SHADOW_SHORT init
        if SHADOW_SHORT.avg_period > 0 && bar >= SHADOW_SHORT.avg_period {
            for j in (bar - SHADOW_SHORT.avg_period)..bar {
                shadow_short_sum[k] += cr(SHADOW_SHORT, open, high, low, close, j);
            }
        }
        // SHADOW_LONG avg_period=0
        if NEAR.avg_period > 0 && bar >= NEAR.avg_period {
            for j in (bar - NEAR.avg_period)..bar {
                near_sum[k] += cr(NEAR, open, high, low, close, j);
            }
        }
        if FAR.avg_period > 0 && bar >= FAR.avg_period {
            for j in (bar - FAR.avg_period)..bar {
                far_sum[k] += cr(FAR, open, high, low, close, j);
            }
        }
    }
    for i in (start - 2 - BODY_LONG.avg_period)..(start - 2) {
        body_long_sum += cr(BODY_LONG, open, high, low, close, i);
    }

    for i in start..len {
        let base = candle_color(open[i-2], close[i-2]) == 1
            && candle_color(open[i-1], close[i-1]) == 1
            && candle_color(open[i], close[i]) == 1
            && close[i-1] > close[i-2] && close[i] > close[i-1]
            // Opens within/near previous body
            && open[i-1] > open[i-2] && open[i-1] <= close[i-2] + ca(NEAR, near_sum[1], open, high, low, close, i-1)
            && open[i] > open[i-1] && open[i] <= close[i-1] + ca(NEAR, near_sum[2], open, high, low, close, i)
            // 1st: long body, short upper shadow
            && real_body(open[i-2], close[i-2]) > ca(BODY_LONG, body_long_sum, open, high, low, close, i-2)
            && upper_shadow(open[i-2], high[i-2], close[i-2]) < ca(SHADOW_SHORT, shadow_short_sum[0], open, high, low, close, i-2);
        // Weakness: bodies getting smaller and/or shadows getting longer
        let weakness = base
            && ((real_body(open[i - 1], close[i - 1])
                < real_body(open[i - 2], close[i - 2])
                    - ca(FAR, far_sum[1], open, high, low, close, i - 1)
                && real_body(open[i], close[i])
                    < real_body(open[i - 1], close[i - 1])
                        + ca(NEAR, near_sum[2], open, high, low, close, i))
                || (real_body(open[i], close[i]) < real_body(open[i - 1], close[i - 1])
                    && real_body(open[i - 1], close[i - 1])
                        < real_body(open[i - 2], close[i - 2])
                    && (upper_shadow(open[i], high[i], close[i])
                        > ca(SHADOW_LONG, shadow_long_sum[2], open, high, low, close, i)
                        || upper_shadow(open[i - 1], high[i - 1], close[i - 1])
                            > ca(
                                SHADOW_LONG,
                                shadow_long_sum[1],
                                open,
                                high,
                                low,
                                close,
                                i - 1,
                            )))
                || (real_body(open[i], close[i])
                    < real_body(open[i - 1], close[i - 1])
                        - ca(FAR, far_sum[2], open, high, low, close, i)));
        output[i] = weakness as i32 * -100;
        // Update sums
        for k in 0..3 {
            let bar = i - 2 + k;
            if SHADOW_SHORT.avg_period > 0 && bar >= SHADOW_SHORT.avg_period {
                shadow_short_sum[k] += cr(SHADOW_SHORT, open, high, low, close, bar)
                    - cr(
                        SHADOW_SHORT,
                        open,
                        high,
                        low,
                        close,
                        bar - SHADOW_SHORT.avg_period,
                    );
            }
            if NEAR.avg_period > 0 && bar >= NEAR.avg_period {
                near_sum[k] += cr(NEAR, open, high, low, close, bar)
                    - cr(NEAR, open, high, low, close, bar - NEAR.avg_period);
            }
            if FAR.avg_period > 0 && bar >= FAR.avg_period {
                far_sum[k] += cr(FAR, open, high, low, close, bar)
                    - cr(FAR, open, high, low, close, bar - FAR.avg_period);
            }
        }
        body_long_sum += cr(BODY_LONG, open, high, low, close, i - 2)
            - cr(
                BODY_LONG,
                open,
                high,
                low,
                close,
                i - 2 - BODY_LONG.avg_period,
            );
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
        let e = crate::stream::candle_advance_block(&o, &h, &l, &c).unwrap();
        let mut s = CandleAdvanceBlock::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
