//! Incremental Takuri candlestick recognition (CDLTAKURI).

use std::collections::VecDeque;

use super::pattern::*;
use crate::error::TaResult;
/// Incremental CDLTAKURI state using TA-Lib's body and shadow averages.
/// Persistent Rust state or aligned output type for `CandleTakuri`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleTakuri {
    bodies: VecDeque<f64>,
    body_sum: f64,
    ranges: VecDeque<f64>,
    range_sum: f64,
    value: Option<i32>,
}

impl Default for CandleTakuri {
    fn default() -> Self {
        Self::new()
    }
}

impl CandleTakuri {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            bodies: VecDeque::with_capacity(10),
            body_sum: 0.0,
            ranges: VecDeque::with_capacity(10),
            range_sum: 0.0,
            value: None,
        }
    }

    fn push(window: &mut VecDeque<f64>, sum: &mut f64, value: f64) {
        if window.len() == 10 {
            *sum -= window.pop_front().expect("window is full");
        }
        window.push_back(value);
        *sum += value;
    }

    /// Appends OHLC data and returns +100 for a takuri after the ten-bar warmup.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let body = (close - open).abs();
        let range = high - low;
        let output = if self.bodies.len() == 10 && self.ranges.len() == 10 {
            let upper_shadow = high - open.max(close);
            let lower_shadow = open.min(close) - low;
            Some(
                (body <= self.body_sum / 10.0
                    && upper_shadow < self.range_sum * 0.01
                    && lower_shadow > body) as i32
                    * 100,
            )
        } else {
            None
        };
        Self::push(&mut self.bodies, &mut self.body_sum, body);
        Self::push(&mut self.ranges, &mut self.range_sum, range);
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
/// Compute the candle takuri result for the supplied aligned series.
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
pub fn candle_takuri(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = *[
        BODY_DOJI.avg_period,
        SHADOW_VERY_SHORT.avg_period,
        SHADOW_VERY_LONG.avg_period,
    ]
    .iter()
    .max()
    .unwrap();
    if len <= lookback {
        return Ok(output);
    }

    let mut body_sum = 0.0;
    let mut shadow_vs_sum = 0.0;
    let mut shadow_vl_sum = 0.0;
    let start = lookback;
    for i in (start - BODY_DOJI.avg_period)..start {
        body_sum += cr(BODY_DOJI, open, high, low, close, i);
    }
    for i in (start - SHADOW_VERY_SHORT.avg_period)..start {
        shadow_vs_sum += cr(SHADOW_VERY_SHORT, open, high, low, close, i);
    }
    // SHADOW_VERY_LONG avg_period=0

    for i in start..len {
        output[i] = (real_body(open[i], close[i])
            <= ca(BODY_DOJI, body_sum, open, high, low, close, i)
            && upper_shadow(open[i], high[i], close[i])
                < ca(SHADOW_VERY_SHORT, shadow_vs_sum, open, high, low, close, i)
            && lower_shadow(open[i], low[i], close[i])
                > ca(SHADOW_VERY_LONG, shadow_vl_sum, open, high, low, close, i))
            as i32
            * 100;
        body_sum += cr(BODY_DOJI, open, high, low, close, i)
            - cr(BODY_DOJI, open, high, low, close, i - BODY_DOJI.avg_period);
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
        let expected = crate::stream::candle_takuri(&open, &high, &low, &close).unwrap();
        let mut state = CandleTakuri::new();
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
