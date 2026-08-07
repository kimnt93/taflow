//! Incremental Three Inside pattern recognition (CDL3INSIDE).
use super::pattern::*;
use crate::error::TaResult;
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    open: f64,
    close: f64,
}
impl Candle {
    fn body(self) -> f64 {
        (self.close - self.open).abs()
    }
    fn color(self) -> i32 {
        if self.close >= self.open {
            1
        } else {
            -1
        }
    }
}
/// Incremental CDL3INSIDE state.
/// Persistent Rust state or aligned output type for `CandleThreeInside`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleThreeInside {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleThreeInside {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleThreeInside {
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
    fn average(&self, start: usize) -> f64 {
        self.candles
            .iter()
            .skip(start)
            .take(10)
            .map(|c| c.body())
            .sum::<f64>()
            / 10.0
    }
    /// Appends OHLC data and returns a three-inside signal after warmup.
    pub fn append(&mut self, open: f64, _high: f64, _low: f64, close: f64) -> Option<i32> {
        let current = Candle { open, close };
        let output = if self.candles.len() == 12 {
            let first = self.candles[10];
            let second = self.candles[11];
            let inside = first.body() > self.average(0)
                && second.body() <= self.average(1)
                && second.open.max(second.close) < first.open.max(first.close)
                && second.open.min(second.close) > first.open.min(first.close);
            let reversal =
                (first.color() == 1 && current.color() == -1 && current.close < first.open)
                    || (first.color() == -1 && current.color() == 1 && current.close > first.open);
            Some(-((inside && reversal) as i32) * first.color() * 100)
        } else {
            None
        };
        if self.candles.len() == 12 {
            self.candles.pop_front();
        }
        self.candles.push_back(current);
        self.value = output;
        output
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
        *self = Self::new()
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
/// Compute the candle three inside result for the supplied aligned series.
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
pub fn candle_three_inside(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_SHORT.avg_period.max(BODY_LONG.avg_period) + 2;
    if len <= lookback {
        return Ok(output);
    }

    let mut body_long_sum = 0.0;
    let mut body_short_sum = 0.0;
    let start = lookback;
    for i in (start - 2 - BODY_LONG.avg_period)..(start - 2) {
        body_long_sum += cr(BODY_LONG, open, high, low, close, i);
    }
    for i in (start - 1 - BODY_SHORT.avg_period)..(start - 1) {
        body_short_sum += cr(BODY_SHORT, open, high, low, close, i);
    }

    for i in start..len {
        output[i] = (real_body(open[i - 2], close[i - 2])
            > ca(BODY_LONG, body_long_sum, open, high, low, close, i - 2)
            && real_body(open[i - 1], close[i - 1])
                <= ca(BODY_SHORT, body_short_sum, open, high, low, close, i - 1)
            && open[i - 1].max(close[i - 1]) < open[i - 2].max(close[i - 2])
            && open[i - 1].min(close[i - 1]) > open[i - 2].min(close[i - 2])
            && ((candle_color(open[i - 2], close[i - 2]) == 1
                && candle_color(open[i], close[i]) == -1
                && close[i] < open[i - 2])
                || (candle_color(open[i - 2], close[i - 2]) == -1
                    && candle_color(open[i], close[i]) == 1
                    && close[i] > open[i - 2]))) as i32
            * -candle_color(open[i - 2], close[i - 2])
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
        body_short_sum += cr(BODY_SHORT, open, high, low, close, i - 1)
            - cr(
                BODY_SHORT,
                open,
                high,
                low,
                close,
                i - 1 - BODY_SHORT.avg_period,
            );
    }
    Ok(output)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let open: Vec<f64> = (0..40).map(|i| 100. + i as f64 * 0.2).collect();
        let high: Vec<f64> = open.iter().map(|x| x + 2.).collect();
        let low: Vec<f64> = open.iter().map(|x| x - 2.).collect();
        let close: Vec<f64> = open.iter().map(|x| x + 1.).collect();
        let expected = crate::stream::candle_three_inside(&open, &high, &low, &close).unwrap();
        let mut s = CandleThreeInside::new();
        for (((&o, &h), &l), (&c, &e)) in open
            .iter()
            .zip(&high)
            .zip(&low)
            .zip(close.iter().zip(&expected))
        {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
