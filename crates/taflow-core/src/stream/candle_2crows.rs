//! Incremental Two Crows candlestick recognition (CDL2CROWS).

use super::pattern::*;
use crate::error::TaResult;
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    open: f64,
    close: f64,
    body: f64,
}
/// Incremental CDL2CROWS state.
/// Persistent Rust state or aligned output type for `CandleTwoCrows`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleTwoCrows {
    candles: VecDeque<Candle>,
    bodies: VecDeque<f64>,
    sum: f64,
    value: Option<i32>,
}
impl Default for CandleTwoCrows {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleTwoCrows {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(3),
            bodies: VecDeque::with_capacity(10),
            sum: 0.0,
            value: None,
        }
    }
    fn push_body(&mut self, value: f64) {
        if self.bodies.len() == 10 {
            self.sum -= self.bodies.pop_front().expect("window full");
        }
        self.bodies.push_back(value);
        self.sum += value;
    }
    /// Appends OHLC data and returns -100 for a two-crows pattern after warmup.
    pub fn append(&mut self, open: f64, _high: f64, _low: f64, close: f64) -> Option<i32> {
        let current = Candle {
            open,
            close,
            body: (close - open).abs(),
        };
        let output = if self.bodies.len() == 10 && self.candles.len() == 2 {
            let first = self.candles[0];
            let second = self.candles[1];
            let pattern = first.close >= first.open
                && first.body > self.sum / 10.0
                && second.close < second.open
                && second.open.min(second.close) > first.open.max(first.close)
                && close < open
                && open < second.open
                && open > second.close
                && close > first.open
                && close < first.close;
            Some(-(pattern as i32) * 100)
        } else {
            None
        };
        if self.candles.len() == 2 {
            self.push_body(self.candles[0].body);
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
/// Compute the candle two crows result for the supplied aligned series.
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
pub fn candle_two_crows(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_LONG.avg_period + 2;
    if len <= lookback {
        return Ok(output);
    }

    let mut body_sum = 0.0;
    let start = lookback;
    for i in (start - 2 - BODY_LONG.avg_period)..(start - 2) {
        body_sum += cr(BODY_LONG, open, high, low, close, i);
    }

    for i in start..len {
        // 1st: long white
        output[i] = (candle_color(open[i-2], close[i-2]) == 1
            && real_body(open[i-2], close[i-2]) > ca(BODY_LONG, body_sum, open, high, low, close, i-2)
            // 2nd: black, gap up
            && candle_color(open[i-1], close[i-1]) == -1
            && real_body_gap_up(open, close, i-1, i-2)
            // 3rd: black, opens within 2nd body, closes within 1st body
            && candle_color(open[i], close[i]) == -1
            && open[i] < open[i-1] && open[i] > close[i-1]
            && close[i] > open[i-2] && close[i] < close[i-2]) as i32
            * -100;
        body_sum += cr(BODY_LONG, open, high, low, close, i - 2)
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
        let open: Vec<f64> = (0..40).map(|i| 100. + i as f64 * 0.2).collect();
        let high: Vec<f64> = open.iter().map(|x| x + 2.).collect();
        let low: Vec<f64> = open.iter().map(|x| x - 2.).collect();
        let close: Vec<f64> = open.iter().map(|x| x + 1.).collect();
        let expected = crate::stream::candle_two_crows(&open, &high, &low, &close).unwrap();
        let mut state = CandleTwoCrows::new();
        for (((&o, &h), &l), (&c, &expected)) in open
            .iter()
            .zip(&high)
            .zip(&low)
            .zip(close.iter().zip(&expected))
        {
            match state.append(o, h, l, c) {
                Some(value) => assert_eq!(value, expected),
                None => assert_eq!(expected, 0),
            }
        }
    }
}
