//! Incremental Doji candlestick recognition (CDLDOJI).

use std::collections::VecDeque;

use super::pattern::*;
use crate::error::TaResult;
/// Incremental CDLDOJI state using TA-Lib's ten-bar High-Low average.
/// Persistent Rust state or aligned output type for `CandleDoji`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleDoji {
    ranges: VecDeque<f64>,
    sum: f64,
    value: Option<i32>,
}
impl Default for CandleDoji {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleDoji {
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
    /// Appends OHLC data and returns +100 for a doji after the ten-bar warmup.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        if self.ranges.len() < 10 {
            self.ranges.push_back(high - low);
            self.sum += high - low;
            return None;
        }
        let threshold = self.sum * 0.01;
        self.value = Some(if (close - open).abs() <= threshold {
            100
        } else {
            0
        });
        self.sum += high - low - self.ranges.pop_front().expect("window is full");
        self.ranges.push_back(high - low);
        self.value
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

/// CDL_DOJI — copysign-based branchless output
///
/// Uses `100.0_f64.copysign(thresh - body).max(0.0) as i32` to produce 0 or 100
/// without any conditional branch. This stays entirely in float registers (NEON fmaxnm),
/// avoiding the conditional-store penalty that LLVM generates for bool→i32 patterns.
/// Compute the candle pattern signal for aligned OHLC bars.
///
/// # Parameters
///
/// * `open`, `high`, `low`, `close` - Equal-length chronological OHLC series.
///
/// # Returns
///
/// A same-length vector containing -100, 0, or 100 pattern signals; bars
/// Compute the candle doji result for the supplied aligned series.
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
pub fn candle_doji(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    let lookback = BODY_DOJI.avg_period; // 10
    if len <= lookback {
        return Ok(output);
    }

    let factor_div = BODY_DOJI.factor / lookback as f64; // 0.01

    let mut sum = 0.0_f64;
    for i in 0..lookback {
        sum += high[i] - low[i];
    }

    for i in lookback..len {
        let body = (close[i] - open[i]).abs();
        let thresh = sum * factor_div;
        // copysign(100, thresh-body): +100 if doji (body<=thresh), -100 if not
        // max(0): clamp -100 to 0 → result is 0 or 100, zero branches
        output[i] = 100.0_f64.copysign(thresh - body).max(0.0) as i32;
        sum += (high[i] - low[i]) - (high[i - lookback] - low[i - lookback]);
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
        let expected = crate::stream::candle_doji(&open, &high, &low, &close).unwrap();
        let mut state = CandleDoji::new();
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
