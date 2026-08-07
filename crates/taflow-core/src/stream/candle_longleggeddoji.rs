//! Incremental Long-Legged Doji candlestick recognition (CDLLONGLEGGEDDOJI).

use std::collections::VecDeque;

use super::pattern::*;
use crate::error::TaResult;
/// Incremental CDLLONGLEGGEDDOJI state using TA-Lib's ten-bar doji range average.
/// Persistent Rust state or aligned output type for `CandleLongLeggedDoji`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleLongLeggedDoji {
    ranges: VecDeque<f64>,
    sum: f64,
    value: Option<i32>,
}
impl Default for CandleLongLeggedDoji {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleLongLeggedDoji {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            ranges: VecDeque::with_capacity(10),
            sum: 0.0,
            value: None,
        }
    }
    /// Appends OHLC data and returns +100 for a long-legged doji after warmup.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let body = (close - open).abs();
        let range = high - low;
        let output = if self.ranges.len() == 10 {
            Some(
                (body <= self.sum * 0.01
                    && (open.min(close) - low > body || high - open.max(close) > body))
                    as i32
                    * 100,
            )
        } else {
            None
        };
        if self.ranges.len() == 10 {
            self.sum -= self.ranges.pop_front().expect("window is full");
        }
        self.ranges.push_back(range);
        self.sum += range;
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
/// Compute the candle long legged doji result for the supplied aligned series.
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
pub fn candle_long_legged_doji(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_DOJI.avg_period.max(SHADOW_LONG.avg_period);
    if len <= lookback {
        return Ok(output);
    }

    let mut body_sum = 0.0;
    let mut shadow_sum = 0.0;
    let start = lookback;
    for i in (start - BODY_DOJI.avg_period)..start {
        body_sum += cr(BODY_DOJI, open, high, low, close, i);
    }
    // SHADOW_LONG avg_period=0, no init

    for i in start..len {
        output[i] = (real_body(open[i], close[i])
            <= ca(BODY_DOJI, body_sum, open, high, low, close, i)
            && (lower_shadow(open[i], low[i], close[i])
                > ca(SHADOW_LONG, shadow_sum, open, high, low, close, i)
                || upper_shadow(open[i], high[i], close[i])
                    > ca(SHADOW_LONG, shadow_sum, open, high, low, close, i)))
            as i32
            * 100;
        body_sum += cr(BODY_DOJI, open, high, low, close, i)
            - cr(BODY_DOJI, open, high, low, close, i - BODY_DOJI.avg_period);
    }
    Ok(output)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let open: Vec<f64> = (0..40).map(|i| 100.0 + i as f64 * 0.2).collect();
        let high: Vec<f64> = open.iter().map(|x| x + 2.0).collect();
        let low: Vec<f64> = open.iter().map(|x| x - 2.0).collect();
        let close: Vec<f64> = open
            .iter()
            .enumerate()
            .map(|(i, x)| x + if i % 3 == 0 { 0.1 } else { 1.0 })
            .collect();
        let expected = crate::stream::candle_long_legged_doji(&open, &high, &low, &close).unwrap();
        let mut state = CandleLongLeggedDoji::new();
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
