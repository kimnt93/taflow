//! Incremental Inverted Hammer candlestick recognition (CDLINVERTEDHAMMER).
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
/// Compute the candle inverted hammer result for the supplied aligned series.
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
pub fn candle_inverted_hammer(
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
    ]
    .iter()
    .max()
    .unwrap()
        + 1;
    if len <= lookback {
        return Ok(output);
    }

    let mut body_sum = 0.0;
    let mut shadow_long_sum = 0.0;
    let mut shadow_vs_sum = 0.0;
    let start = lookback;
    for i in (start - BODY_SHORT.avg_period)..start {
        body_sum += cr(BODY_SHORT, open, high, low, close, i);
    }
    for i in (start - SHADOW_VERY_SHORT.avg_period)..start {
        shadow_vs_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i);
    }

    for i in start..len {
        output[i] = (real_body(open[i], close[i])
            < ca(BODY_SHORT, body_sum, open, high, low, close, i)
            && upper_shadow(open[i], high[i], close[i])
                > ca(SHADOW_LONG, shadow_long_sum, open, high, low, close, i)
            && lower_shadow(open[i], low[i], close[i])
                < ca(SHADOW_VERY_SHORT, shadow_vs_sum, open, high, low, close, i)
            && real_body_gap_down(open, close, i, i - 1)) as i32
            * 100;
        if BODY_SHORT.avg_period > 0 {
            body_sum += cr(BODY_SHORT, open, high, low, close, i)
                - cr(
                    BODY_SHORT,
                    open,
                    high,
                    low,
                    close,
                    i - BODY_SHORT.avg_period,
                );
        }
        if SHADOW_VERY_SHORT.avg_period > 0 {
            shadow_vs_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i)
                - cr(
                    SHADOW_VERY_SHORT,
                    open,
                    high,
                    low,
                    close,
                    i - SHADOW_VERY_SHORT.avg_period,
                );
        }
    }
    Ok(output)
}
/// Stateful CandleInvertedHammer candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleInvertedHammer {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleInvertedHammer {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleInvertedHammer {
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
            let body = self.candles.iter().skip(1).map(|x| x.body()).sum::<f64>() / 10.0;
            let vs = self.candles.iter().skip(1).map(|x| x.range()).sum::<f64>() * 0.01;
            Some(
                (cur.body() < body
                    && cur.upper() > cur.body()
                    && cur.lower() < vs
                    && cur.o.max(cur.c) < prev.o.min(prev.c)) as i32
                    * 100,
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
        let e = crate::stream::candle_inverted_hammer(&o, &h, &l, &c).unwrap();
        let mut s = CandleInvertedHammer::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
