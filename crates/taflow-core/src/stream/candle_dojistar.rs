//! Incremental Doji Star candlestick recognition (CDLDOJISTAR).
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
}
/// Stateful CandleDojiStar candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleDojiStar {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleDojiStar {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleDojiStar {
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
    /// Appends OHLC data and returns a signed doji-star signal after the eleven-bar warmup.
    pub fn append(&mut self, o: f64, h: f64, l: f64, c: f64) -> Option<i32> {
        let cur = Candle { o, h, l, c };
        let value = if self.candles.len() == 11 {
            let long = self.candles.iter().take(10).map(|x| x.body()).sum::<f64>() / 10.0;
            let doji = self.candles.iter().skip(1).map(|x| x.range()).sum::<f64>() * 0.01;
            let prev = self.candles[10];
            let base = prev.body() > long && cur.body() <= doji;
            let bear = base && prev.c >= prev.o && cur.o.min(cur.c) > prev.o.max(prev.c);
            let bull = base && prev.c < prev.o && cur.o.max(cur.c) < prev.o.min(prev.c);
            Some((bull as i32) * 100 - (bear as i32) * 100)
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
/// Compute the candle doji star result for the supplied aligned series.
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
pub fn candle_doji_star(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_DOJI.avg_period.max(BODY_LONG.avg_period) + 1;
    if len <= lookback {
        return Ok(output);
    }

    let mut body_long_sum = 0.0;
    let mut body_doji_sum = 0.0;
    let start = lookback;
    for i in (start - 1 - BODY_LONG.avg_period)..(start - 1) {
        body_long_sum += cr(BODY_LONG, open, high, low, close, i);
    }
    for i in (start - BODY_DOJI.avg_period)..start {
        body_doji_sum += cr(BODY_DOJI, open, high, low, close, i);
    }

    for i in start..len {
        let base = real_body(open[i - 1], close[i - 1])
            > ca(BODY_LONG, body_long_sum, open, high, low, close, i - 1)
            && real_body(open[i], close[i])
                <= ca(BODY_DOJI, body_doji_sum, open, high, low, close, i);
        let bear = base
            && candle_color(open[i - 1], close[i - 1]) == 1
            && real_body_gap_up(open, close, i, i - 1);
        let bull = base
            && candle_color(open[i - 1], close[i - 1]) == -1
            && real_body_gap_down(open, close, i, i - 1);
        output[i] = (bull as i32) * 100 - (bear as i32) * 100;
        body_long_sum += cr(BODY_LONG, open, high, low, close, i - 1)
            - cr(
                BODY_LONG,
                open,
                high,
                low,
                close,
                i - 1 - BODY_LONG.avg_period,
            );
        body_doji_sum += cr(BODY_DOJI, open, high, low, close, i)
            - cr(BODY_DOJI, open, high, low, close, i - BODY_DOJI.avg_period);
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
            .map(|(i, x)| x + if i % 3 == 0 { -0.1 } else { 1.0 })
            .collect();
        let e = crate::stream::candle_doji_star(&o, &h, &l, &c).unwrap();
        let mut s = CandleDojiStar::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            assert_eq!(s.append(o, h, l, c).unwrap_or(0), e)
        }
    }
}
