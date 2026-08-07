//! Incremental Gap Side-by-Side White candlestick recognition (CDLGAPSIDESIDEWHITE).
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
/// Stateful CandleGapSideSideWhite candle recognizer.
/// Consumes causal OHLC bars and returns an aligned pattern score.
pub struct CandleGapSideSideWhite {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleGapSideSideWhite {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleGapSideSideWhite {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(7),
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
        let value = if self.candles.len() == 7 {
            let a = self.candles[5];
            let b = self.candles[6];
            let near = self
                .candles
                .iter()
                .skip(1)
                .take(5)
                .map(|x| x.range())
                .sum::<f64>()
                * 0.04;
            let equal = self
                .candles
                .iter()
                .skip(1)
                .take(5)
                .map(|x| x.range())
                .sum::<f64>()
                * 0.01;
            let base = b.color() == 1
                && cur.color() == 1
                && (b.body() - cur.body()).abs() < near
                && (b.o - cur.o).abs() < equal;
            let bull = base && b.o.min(b.c) > a.o.max(a.c);
            let bear = base && b.o.max(b.c) < a.o.min(a.c);
            Some((bull as i32) * 100 - (bear as i32) * 100)
        } else {
            None
        };
        if self.candles.len() == 7 {
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
/// Compute the candle gap side side white result for the supplied aligned series.
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
pub fn candle_gap_side_side_white(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = NEAR.avg_period.max(EQUAL.avg_period) + 2;
    if len <= lookback {
        return Ok(output);
    }

    let mut near_sum = 0.0;
    let mut equal_sum = 0.0;
    let start = lookback;
    for i in (start - 1 - NEAR.avg_period)..(start - 1) {
        near_sum += cr(NEAR, open, high, low, close, i);
    }
    for i in (start - 1 - EQUAL.avg_period)..(start - 1) {
        equal_sum += cr(EQUAL, open, high, low, close, i);
    }

    for i in start..len {
        let base = candle_color(open[i - 1], close[i - 1]) == 1
            && candle_color(open[i], close[i]) == 1
            && (real_body(open[i - 1], close[i - 1]) - real_body(open[i], close[i])).abs()
                < ca(NEAR, near_sum, open, high, low, close, i - 1)
            && (open[i - 1] - open[i]).abs() < ca(EQUAL, equal_sum, open, high, low, close, i - 1);
        // Upside gap
        let bull = base && real_body_gap_up(open, close, i - 1, i - 2);
        // Downside gap
        let bear = base && real_body_gap_down(open, close, i - 1, i - 2);
        output[i] = (bull as i32) * 100 - (bear as i32) * 100;
        near_sum += cr(NEAR, open, high, low, close, i - 1)
            - cr(NEAR, open, high, low, close, i - 1 - NEAR.avg_period);
        equal_sum += cr(EQUAL, open, high, low, close, i - 1)
            - cr(EQUAL, open, high, low, close, i - 1 - EQUAL.avg_period);
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
        let e = crate::stream::candle_gap_side_side_white(&o, &h, &l, &c).unwrap();
        let mut s = CandleGapSideSideWhite::new();
        for ((((&o, &h), &l), &c), &e) in o.iter().zip(&h).zip(&l).zip(&c).zip(&e) {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
