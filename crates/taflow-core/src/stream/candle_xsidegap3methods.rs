//! Incremental Upside/Downside Gap Three Methods recognition (CDLXSIDEGAP3METHODS).
use super::pattern::*;
use crate::error::TaResult;
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    open: f64,
    close: f64,
}
/// Incremental CDLXSIDEGAP3METHODS state.
/// Persistent Rust state or aligned output type for `CandleUpDownSideGapThreeMethods`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleUpDownSideGapThreeMethods {
    candles: VecDeque<Candle>,
    value: Option<i32>,
}
impl Default for CandleUpDownSideGapThreeMethods {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleUpDownSideGapThreeMethods {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(2),
            value: None,
        }
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, open: f64, _high: f64, _low: f64, close: f64) -> Option<i32> {
        let output = if self.candles.len() == 2 {
            let first = self.candles[0];
            let second = self.candles[1];
            let first_color = if first.close >= first.open { 1 } else { -1 };
            let second_color = if second.close >= second.open { 1 } else { -1 };
            let current_color = if close >= open { 1 } else { -1 };
            let base = first_color == second_color
                && current_color != first_color
                && open > second.open.min(second.close)
                && open < second.open.max(second.close)
                && close > first.open.min(first.close)
                && close < first.open.max(first.close);
            let bull = base
                && first_color == 1
                && second.open.min(second.close) > first.open.max(first.close);
            let bear = base
                && first_color == -1
                && second.open.max(second.close) < first.open.min(first.close);
            Some((bull as i32) * 100 - (bear as i32) * 100)
        } else {
            None
        };
        if self.candles.len() == 2 {
            self.candles.pop_front();
        }
        self.candles.push_back(Candle { open, close });
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
/// Compute the candle xside gap three methods result for the supplied aligned series.
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
pub fn candle_xside_gap_three_methods(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<Vec<i32>> {
    let len = validate_ohlc(open, high, low, close)?;
    let mut output = vec![0i32; len];
    // lookback = 2
    if len < 3 {
        return Ok(output);
    }

    for i in 2..len {
        let c2 = candle_color(open[i - 2], close[i - 2]);
        let c1 = candle_color(open[i - 1], close[i - 1]);
        let c0 = candle_color(open[i], close[i]);

        // 3rd opens within 2nd body, closes within 1st body
        let opens_within =
            open[i] > open[i - 1].min(close[i - 1]) && open[i] < open[i - 1].max(close[i - 1]);
        let closes_within =
            close[i] > open[i - 2].min(close[i - 2]) && close[i] < open[i - 2].max(close[i - 2]);
        let base = c2 == c1 && c0 != c2 && opens_within && closes_within;
        // Upside gap
        let bull = base && c2 == 1 && real_body_gap_up(open, close, i - 1, i - 2);
        // Downside gap
        let bear = base && c2 == -1 && real_body_gap_down(open, close, i - 1, i - 2);
        output[i] = (bull as i32) * 100 - (bear as i32) * 100;
    }
    Ok(output)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let open = vec![10., 13., 13.5, 10., 7., 7.5];
        let high = vec![14.; 6];
        let low = vec![6.; 6];
        let close = vec![12., 14., 11., 8., 6., 9.];
        let e = crate::stream::candle_xside_gap_three_methods(&open, &high, &low, &close).unwrap();
        let mut s = CandleUpDownSideGapThreeMethods::new();
        for (((&o, &h), &l), (&c, &e)) in open.iter().zip(&high).zip(&low).zip(close.iter().zip(&e))
        {
            match s.append(o, h, l, c) {
                Some(v) => assert_eq!(v, e),
                None => assert_eq!(e, 0),
            }
        }
    }
}
