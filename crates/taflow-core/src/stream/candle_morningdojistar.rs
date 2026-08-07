//! Incremental Morning Doji Star candlestick recognition (CDLMORNINGDOJISTAR).
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
    fn color(self) -> i32 {
        if self.c >= self.o {
            1
        } else {
            -1
        }
    }
}
/// Stateful CandleMorningDojiStar candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleMorningDojiStar {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleMorningDojiStar {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleMorningDojiStar {
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
            let doji = self
                .candles
                .iter()
                .skip(1)
                .take(10)
                .map(|x| x.range())
                .sum::<f64>()
                * 0.01;
            let short = self.candles.iter().skip(2).map(|x| x.body()).sum::<f64>() / 10.0;
            Some(
                (a.color() == -1
                    && a.body() > long
                    && b.body() <= doji
                    && b.o.max(b.c) < a.o.min(a.c)
                    && cur.color() == 1
                    && cur.body() > short
                    && cur.c > a.c + a.body() * 0.3) as i32
                    * 100,
            )
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
/// Compute the candle morning doji star result for the supplied aligned series.
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
pub fn candle_morning_doji_star(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let penetration = 0.3;
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
        body_long_sum += cr(BODY_LONG, open, high, low, close, i);
    }
    for i in (start - 1 - BODY_DOJI.avg_period)..(start - 1) {
        body_doji_sum += cr(BODY_DOJI, open, high, low, close, i);
    }
    for i in (start - BODY_SHORT.avg_period)..start {
        body_short_sum += cr(BODY_SHORT, open, high, low, close, i);
    }

    for i in start..len {
        output[i] = (candle_color(open[i - 2], close[i - 2]) == -1
            && real_body(open[i - 2], close[i - 2])
                > ca(BODY_LONG, body_long_sum, open, high, low, close, i - 2)
            && real_body(open[i - 1], close[i - 1])
                <= ca(BODY_DOJI, body_doji_sum, open, high, low, close, i - 1)
            && real_body_gap_down(open, close, i - 1, i - 2)
            && candle_color(open[i], close[i]) == 1
            && real_body(open[i], close[i])
                > ca(BODY_SHORT, body_short_sum, open, high, low, close, i)
            && close[i] > close[i - 2] + real_body(open[i - 2], close[i - 2]) * penetration)
            as i32
            * 100;
        body_long_sum += cr(BODY_LONG, open, high, low, close, i - 2)
            - cr(
                BODY_LONG,
                open,
                high,
                low,
                close,
                i - 2 - BODY_LONG.avg_period,
            );
        body_doji_sum += cr(BODY_DOJI, open, high, low, close, i - 1)
            - cr(
                BODY_DOJI,
                open,
                high,
                low,
                close,
                i - 1 - BODY_DOJI.avg_period,
            );
        body_short_sum += cr(BODY_SHORT, open, high, low, close, i)
            - cr(
                BODY_SHORT,
                open,
                high,
                low,
                close,
                i - BODY_SHORT.avg_period,
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
        let e = crate::stream::candle_morning_doji_star(&o, &h, &l, &c).unwrap();
        let mut s = CandleMorningDojiStar::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
