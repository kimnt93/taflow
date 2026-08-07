//! Incremental Dark Cloud Cover candlestick recognition (CDLDARKCLOUDCOVER).
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
/// Stateful CandleDarkCloudCover candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleDarkCloudCover {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleDarkCloudCover {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleDarkCloudCover {
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
            let long = self.candles.iter().take(10).map(|x| x.body()).sum::<f64>() / 10.0;
            Some(
                (prev.color() == 1
                    && prev.body() > long
                    && cur.color() == -1
                    && cur.o > prev.h
                    && cur.c > prev.o
                    && cur.c < prev.c - prev.body() * 0.5) as i32
                    * -100,
            )
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
/// Compute the candle dark cloud cover result for the supplied aligned series.
///
/// # Parameters
///
/// * `open` - Input series or configuration value.
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `close` - Input series or configuration value.
/// * `penetration` - Fraction of the prior body required for bearish penetration.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn candle_dark_cloud_cover(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
    penetration: f64,
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_LONG.avg_period + 1;
    if len <= lookback {
        return Ok(output);
    }

    let mut body_sum = 0.0;
    let start = lookback;
    for i in (start - 1 - BODY_LONG.avg_period)..(start - 1) {
        body_sum += cr(BODY_LONG, open, high, low, close, i);
    }

    for i in start..len {
        output[i] = (candle_color(open[i - 1], close[i - 1]) == 1
            && real_body(open[i - 1], close[i - 1])
                > ca(BODY_LONG, body_sum, open, high, low, close, i - 1)
            && candle_color(open[i], close[i]) == -1
            && open[i] > high[i - 1]
            && close[i] > open[i - 1]
            && close[i] < close[i - 1] - real_body(open[i - 1], close[i - 1]) * penetration)
            as i32
            * -100;
        body_sum += cr(BODY_LONG, open, high, low, close, i - 1)
            - cr(
                BODY_LONG,
                open,
                high,
                low,
                close,
                i - 1 - BODY_LONG.avg_period,
            );
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
        let e = crate::stream::candle_dark_cloud_cover(&o, &h, &l, &c, 0.5).unwrap();
        let mut s = CandleDarkCloudCover::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
