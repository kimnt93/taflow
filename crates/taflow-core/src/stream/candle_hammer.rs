//! Incremental Hammer candlestick recognition (CDLHAMMER).

use std::collections::VecDeque;

use super::pattern::*;
use crate::error::TaResult;
/// Incremental CDLHAMMER state using TA-Lib's body, range, and near windows.
/// Persistent Rust state or aligned output type for `CandleHammer`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleHammer {
    bodies: VecDeque<f64>,
    body_sum: f64,
    ranges: VecDeque<f64>,
    range_sum: f64,
    near: VecDeque<f64>,
    near_sum: f64,
    previous: Option<(f64, f64)>,
    value: Option<i32>,
}
impl Default for CandleHammer {
    fn default() -> Self {
        Self::new()
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
/// Compute the candle hammer result for the supplied aligned series.
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
pub fn candle_hammer(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = *[
        BODY_SHORT.avg_period,
        SHADOW_LONG.avg_period,
        SHADOW_VERY_SHORT.avg_period,
        NEAR.avg_period,
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
    let mut near_sum = 0.0;

    let start = lookback;
    // BODY_SHORT: RealBody, SHADOW_LONG: RealBody(avg=0), SHADOW_VERY_SHORT: HighLow, NEAR: HighLow
    for i in (start - BODY_SHORT.avg_period)..start {
        body_sum += cr_realbody(open, high, low, close, i);
    }
    for i in (start - SHADOW_VERY_SHORT.avg_period)..start {
        shadow_vs_sum += cr_highlow(open, high, low, close, i);
    }
    for i in (start - 1 - NEAR.avg_period)..(start - 1) {
        near_sum += cr_highlow(open, high, low, close, i);
    }

    for i in start..len {
        output[i] = (real_body(open[i], close[i])
            < ca_realbody(BODY_SHORT, body_sum, open, high, low, close, i)
            && lower_shadow(open[i], low[i], close[i])
                > ca_realbody(SHADOW_LONG, shadow_long_sum, open, high, low, close, i)
            && upper_shadow(open[i], high[i], close[i])
                < ca_highlow(SHADOW_VERY_SHORT, shadow_vs_sum, open, high, low, close, i)
            && open[i].min(close[i])
                <= low[i - 1] + ca_highlow(NEAR, near_sum, open, high, low, close, i - 1))
            as i32
            * 100;
        // Update sums — monomorphized: no match dispatch
        if BODY_SHORT.avg_period > 0 {
            body_sum += cr_realbody(open, high, low, close, i)
                - cr_realbody(open, high, low, close, i - BODY_SHORT.avg_period);
        }
        if SHADOW_VERY_SHORT.avg_period > 0 {
            shadow_vs_sum += cr_highlow(open, high, low, close, i)
                - cr_highlow(open, high, low, close, i - SHADOW_VERY_SHORT.avg_period);
        }
        if NEAR.avg_period > 0 {
            near_sum += cr_highlow(open, high, low, close, i - 1)
                - cr_highlow(open, high, low, close, i - 1 - NEAR.avg_period);
        }
    }
    Ok(output)
}
impl CandleHammer {
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
            near: VecDeque::with_capacity(5),
            near_sum: 0.0,
            previous: None,
            value: None,
        }
    }
    fn push(window: &mut VecDeque<f64>, sum: &mut f64, capacity: usize, value: f64) {
        if window.len() == capacity {
            *sum -= window.pop_front().expect("window is full");
        }
        window.push_back(value);
        *sum += value;
    }
    /// Appends OHLC data and returns +100 for a hammer after warmup.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let range = high - low;
        let body = (close - open).abs();
        let output = if self.bodies.len() == 10 && self.ranges.len() == 10 && self.near.len() == 5 {
            let (previous_low, previous_range) = self.previous.expect("history exists");
            let short_body = body < self.body_sum / 10.0;
            let long_lower = open.min(close) - low > body;
            let short_upper = high - open.max(close) < self.range_sum * 0.01;
            let near_low = open.min(close) <= previous_low + self.near_sum * 0.04;
            let _ = previous_range;
            Some((short_body && long_lower && short_upper && near_low) as i32 * 100)
        } else {
            None
        };
        if let Some((_, previous_range)) = self.previous {
            Self::push(&mut self.near, &mut self.near_sum, 5, previous_range);
        }
        Self::push(&mut self.bodies, &mut self.body_sum, 10, body);
        Self::push(&mut self.ranges, &mut self.range_sum, 10, range);
        self.previous = Some((low, range));
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let open: Vec<f64> = (0..50).map(|i| 100.0 + i as f64 * 0.1).collect();
        let high: Vec<f64> = open.iter().map(|x| x + 2.0).collect();
        let low: Vec<f64> = open.iter().map(|x| x - 2.0).collect();
        let close: Vec<f64> = open
            .iter()
            .enumerate()
            .map(|(i, x)| x + if i % 5 == 0 { 0.1 } else { 1.0 })
            .collect();
        let expected = crate::stream::candle_hammer(&open, &high, &low, &close).unwrap();
        let mut state = CandleHammer::new();
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
